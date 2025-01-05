package db

import (
	"encoding/json"
	"fmt"
	"math"
	"math/big"
	"os"
	"sync"
)

// 10 ^ 100
var OneGoogol = new(big.Int).Exp(big.NewInt(10), big.NewInt(100), nil)

type Count struct {
	Counter big.Int
}

// Convert to readable json
func (c Count) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		Counter string `json:"counter"`
	}{
		Counter: c.Counter.String(),
	})
}

// Convert from readable json
func (c *Count) UnmarshalJSON(data []byte) error {
	aux := struct {
		Counter string `json:"counter"`
	}{}
	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}
	if _, ok := c.Counter.SetString(aux.Counter, 10); !ok {
		return fmt.Errorf("invalid big.Int string: %s", aux.Counter)
	}
	return nil
}

type CountStore struct {
	mx    sync.Mutex
	count Count
}

var GlobalCount = newCountStore()

// Create a New CountStore
func newCountStore() CountStore {
	return CountStore{mx: sync.Mutex{}, count: Count{Counter: *big.NewInt(0)}}
}

// Loads the counter from a JSON file if it exists, otherwise does nothing
func (cs *CountStore) LoadCountFromFile(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		return
	}
	defer file.Close()

	var data Count
	if err := json.NewDecoder(file).Decode(&data); err != nil {
		return
	}

	cs.mx.Lock()
	defer cs.mx.Unlock()
	cs.count = data
}

// Saves the current counter to a JSON file
func (cs *CountStore) SaveCountToFile(filename string) error {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	file, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	if err := json.NewEncoder(file).Encode(cs.count); err != nil {
		return err
	}
	return nil
}

// Get current count as a string
func (cs *CountStore) GetCounter() string {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	return cs.count.Counter.String()
}

// Get current count as a struct
func (cs *CountStore) Get() Count {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	return cs.count
}

// Increments the counter by the current step size
func (cs *CountStore) Increment() {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	step := computeStep(&cs.count.Counter)
	tmp := new(big.Int).Add(&cs.count.Counter, step)

	if tmp.Cmp(OneGoogol) > 0 {
		cs.count.Counter.Set(OneGoogol)
	} else {
		cs.count.Counter.Set(tmp)
	}
}

// Decrements the counter by the current step size
func (cs *CountStore) Decrement() {
	cs.mx.Lock()
	defer cs.mx.Unlock()

	step := computeStep(&cs.count.Counter)
	tmp := new(big.Int).Sub(&cs.count.Counter, step)
	if cs.count.Counter.Cmp(OneGoogol) != 0 {
		if tmp.Sign() < 0 {
			cs.count.Counter.SetInt64(0)
		} else {
			cs.count.Counter.Set(tmp)
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
