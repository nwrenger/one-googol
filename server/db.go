package main

import (
	"fmt"
	"math/big"
	"os"
	"sync"
)

// 10 ^ 100
var OneGoogol = new(big.Int).Exp(big.NewInt(10), big.NewInt(100), nil)

type Database struct {
	mx    sync.RWMutex `json:"-"`
	Count big.Int      `json:"count"`
}

type Scaling struct {
	exponent int
	base     int
	// ..
}

type Meter struct {
	Increment int64
	Decrement int64
	Pending   int64
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

	if _, ok := db.Count.SetString(countStr, 10); !ok {
		fmt.Printf("Invalid big.Int string in file: %s\n", countStr)
		db.Count.SetInt64(0)
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

	_, err = file.WriteString(db.Count.String())
	return err
}

// Get current count as a string
func (db *Database) GetString() string {
	db.mx.RLock()
	defer db.mx.RUnlock()

	return db.Count.String()
}

// Get current count as big.Int
func (db *Database) Get() *big.Int {
	db.mx.RLock()
	defer db.mx.RUnlock()

	return new(big.Int).Set(&db.Count)
}

// Updates the counter by the meter of the clients
func (db *Database) UpdateCounter(meter Meter) {
	db.mx.Lock()
	defer db.mx.Unlock()

	cmpStep := computeStep(&db.Count)

	stepIncrement := new(big.Int).Exp(big.NewInt(meter.Increment), cmpStep, nil)
	tmpIncrement := new(big.Int).Add(&db.Count, stepIncrement)

	if tmpIncrement.Cmp(OneGoogol) > 0 {
		db.Count.Set(OneGoogol)
	} else {
		db.Count.Set(tmpIncrement)
	}

	stepDecrement := new(big.Int).Exp(big.NewInt(meter.Decrement), cmpStep, nil)
	tmpDecrement := new(big.Int).Sub(&db.Count, stepDecrement)
	if db.Count.Cmp(OneGoogol) != 0 {
		if tmpDecrement.Sign() < 0 {
			db.Count.SetInt64(0)
		} else {
			db.Count.Set(tmpDecrement)
		}
	}
}

// Calculates `sqrt(digits)`
func computeStep(counter *big.Int) *big.Int {
	absValue := new(big.Int).Abs(counter)
	valueStr := absValue.String()
	digitLength := len(valueStr)

	return new(big.Int).Sqrt(big.NewInt(int64(digitLength)))
}
