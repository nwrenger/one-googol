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

type CountStore struct {
	mx    sync.Mutex
	count big.Int
}

var GlobalCount = newCountStore()

// Create a New CountStore
func newCountStore() CountStore {
	return CountStore{
		mx:    sync.Mutex{},
		count: *big.NewInt(0),
	}
}

// Loads the counter from a plain text file if it exists, otherwise does nothing
func (cs *CountStore) LoadCountFromFile(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		// If the file doesn't exist, start with zero
		return
	}
	defer file.Close()

	var countStr string
	if _, err := fmt.Fscanf(file, "%s", &countStr); err != nil {
		return
	}

	if _, ok := cs.count.SetString(countStr, 10); !ok {
		fmt.Printf("Invalid big.Int string in file: %s\n", countStr)
		cs.count.SetInt64(0)
	}
}

// Saves the current counter to a plain text file
func (cs *CountStore) SaveCountToFile(filename string) error {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	file, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.WriteString(cs.count.String())
	return err
}

// Get current count as a string
func (cs *CountStore) GetCounter() string {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	return cs.count.String()
}

// Get current count as big.Int
func (cs *CountStore) Get() *big.Int {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	return new(big.Int).Set(&cs.count)
}

// Increments the counter by the current step size
func (cs *CountStore) Increment() {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	step := computeStep(&cs.count)
	tmp := new(big.Int).Add(&cs.count, step)

	if tmp.Cmp(OneGoogol) > 0 {
		cs.count.Set(OneGoogol)
	} else {
		cs.count.Set(tmp)
	}
}

// Decrements the counter by the current step size
func (cs *CountStore) Decrement() {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	step := computeStep(&cs.count)
	tmp := new(big.Int).Sub(&cs.count, step)
	if cs.count.Cmp(OneGoogol) != 0 {
		if tmp.Sign() < 0 {
			cs.count.SetInt64(0)
		} else {
			cs.count.Set(tmp)
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
