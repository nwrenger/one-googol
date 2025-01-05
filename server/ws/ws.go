package ws

import (
	"log"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/nwrenger/one-googol/db"
)

var (
	clients = make(map[*websocket.Conn]bool)
	mx      sync.Mutex
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool { return true },
}

// Websocket handler
func WsHandler(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("WebSocket Upgrade error:", err)
		return
	}

	mx.Lock()
	clients[conn] = true
	mx.Unlock()

	sendCount(conn)

	for {
		_, msg, err := conn.ReadMessage()
		if err != nil {
			mx.Lock()
			delete(clients, conn)
			mx.Unlock()
			conn.Close()
			break
		}

		command := string(msg)
		switch command {
		case "increment":
			db.GlobalCount.Increment()
		case "decrement":
			db.GlobalCount.Decrement()
		default:
			log.Printf("Unknown command: %s\n", command)
		}
	}
}

// Send current count to specific websocket connection
func sendCount(conn *websocket.Conn) {
	current := db.GlobalCount.GetCounter()
	if err := conn.WriteMessage(websocket.TextMessage, []byte(current)); err != nil {
		log.Println("Error sending initial count:", err)
		conn.Close()

		mx.Lock()
		delete(clients, conn)
		mx.Unlock()
	}
}

// Starts a periodic broadcast, which only broadcasts when the counter changed
func StartBroadcast() {
	var lastCount string

	go func() {
		ticker := time.NewTicker(250 * time.Millisecond)
		defer ticker.Stop()

		for range ticker.C {
			newCount := db.GlobalCount.GetCounter()
			if newCount != lastCount {
				broadcast(newCount)
				lastCount = newCount
			}
		}
	}()
}

// Writes the current count to all connected clients
func broadcast(newCount string) {
	mx.Lock()
	defer mx.Unlock()

	for conn := range clients {
		if err := conn.WriteMessage(websocket.TextMessage, []byte(newCount)); err != nil {
			log.Println("Error writing to client:", err)
			conn.Close()
			delete(clients, conn)
		}
	}
}
