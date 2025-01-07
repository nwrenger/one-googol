package db

import (
	"fmt"
	"math"
	"math/big"
	"os"
	"sync"
)

// 10 ^ 100
var OneGoogol = new(big.Int).Exp(big.NewInt(10), big.NewInt(100), nil)

type Database struct {
	mx    sync.RWMutex
	count big.Int
}

type Meter struct {
	Increment int64
	Decrement int64
}

// Loads the counter from a plain text file if it exists, otherwise does nothing
func (db *Database) LoadCountFromFile(filename string) {
	db.mx.Lock()
	defer db.mx.Unlock()

	file, err := os.Open(filename)
	if err != nil {
		return
	}
	defer file.Close()

	var countStr string
	if _, err := fmt.Fscanf(file, "%s", &countStr); err != nil {
		return
	}

	if _, ok := db.count.SetString(countStr, 10); !ok {
		fmt.Printf("Invalid big.Int string in file: %s\n", countStr)
		db.count.SetInt64(0)
	}
}

// Saves the current counter to a plain text file
func (db *Database) SaveCountToFile(filename string) error {
	db.mx.RLock()
	defer db.mx.RUnlock()

	file, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.WriteString(db.count.String())
	return err
}

// Get current count as a string
func (db *Database) GetString() string {
	db.mx.RLock()
	defer db.mx.RUnlock()

	return db.count.String()
}

// Get current count as big.Int
func (db *Database) Get() *big.Int {
	db.mx.RLock()
	defer db.mx.RUnlock()

	return new(big.Int).Set(&db.count)
}

// Updates the counter by the meter of the clients
func (db *Database) UpdateCounter(meter Meter) {
	db.mx.Lock()
	defer db.mx.Unlock()

	cmpStep := computeStep(&db.count)

	stepIncrement := big.NewInt(0).Mul(big.NewInt(meter.Increment), cmpStep)
	tmpIncrement := new(big.Int).Add(&db.count, stepIncrement)

	if tmpIncrement.Cmp(OneGoogol) > 0 {
		db.count.Set(OneGoogol)
	} else {
		db.count.Set(tmpIncrement)
	}

	stepDecrement := big.NewInt(0).Mul(big.NewInt(meter.Decrement), cmpStep)
	tmpDecrement := new(big.Int).Sub(&db.count, stepDecrement)
	if db.count.Cmp(OneGoogol) != 0 {
		if tmpDecrement.Sign() < 0 {
			db.count.SetInt64(0)
		} else {
			db.count.Set(tmpDecrement)
		}
	}
}

// Increases the size of the step by 100 ^ ((counterLength - 1) / 3)
func computeStep(counter *big.Int) *big.Int {
	absValue := new(big.Int).Abs(counter)
	valueStr := absValue.String()
	digitLength := len(valueStr)

	phaseIndex := int(math.Floor(float64(digitLength-1) / 3))
	return new(big.Int).Exp(big.NewInt(100), big.NewInt(int64(phaseIndex)), nil)
}
