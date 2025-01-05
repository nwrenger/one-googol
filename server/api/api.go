package api

import (
	"net/http"

	"github.com/nwrenger/one-googol/db"
)

// Handler for GET `/count`
func GetCount(w http.ResponseWriter, r *http.Request) {
	count := db.GlobalCount.GetCounter()
	w.Header().Set("Content-Type", "text/plain")
	w.Write([]byte(count))
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
