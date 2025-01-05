package api

import (
	"encoding/json"
	"net/http"

	"github.com/nwrenger/one-googol/db"
)

// Handler for GET `/count`
func GetCount(w http.ResponseWriter, r *http.Request) {
	response := db.GlobalCount.Get()
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}

// Handler for POST `/count/increment`
func IncrementCount(w http.ResponseWriter, r *http.Request) {
	db.GlobalCount.Increment()
	GetCount(w, r)
}

// Handler for POST `/count/decrement`
func DecrementCount(w http.ResponseWriter, r *http.Request) {
	db.GlobalCount.Decrement()
	GetCount(w, r)
}
