package ws

import (
	"fmt"
	"log"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/nwrenger/one-googol/db"
)

type WebSocket struct {
	clients sync.Map // map[*websocket.Conn]Status
}

// Client's state.
type Status int

const (
	Disconnected Status = iota
	Pending
	Increment
	Decrement
)

// Upgrades HTTP connections to WebSocket connections.
var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool { return true },
}

// Upgrades the HTTP connection to a WebSocket and handles the connection.
func (ws *WebSocket) WsHandler(database *db.Database, w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("WebSocket Upgrade error:", err)
		return
	}

	ws.clients.Store(conn, Pending)
	ws.sendCount(conn, database)

	go ws.handleConnection(conn)
}

// Manages the WebSocket connection lifecycle.
func (ws *WebSocket) handleConnection(conn *websocket.Conn) {
	defer func() {
		ws.clients.Delete(conn)
		conn.Close()
	}()

	// Set read deadlines and handle ping/pong to keep the connection alive.
	conn.SetReadLimit(512)
	conn.SetReadDeadline(time.Now().Add(60 * time.Second))
	conn.SetPongHandler(func(string) error {
		conn.SetReadDeadline(time.Now().Add(60 * time.Second))
		return nil
	})

	for {
		messageType, msg, err := conn.ReadMessage()
		if err != nil {
			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
				log.Printf("WebSocket unexpected close: %v", err)
			}
			break
		}

		if messageType != websocket.TextMessage {
			continue
		}

		command := string(msg)
		switch command {
		case "increment":
			ws.clients.Store(conn, Increment)
		case "decrement":
			ws.clients.Store(conn, Decrement)
		default:
			log.Printf("Unknown command: %s", command)
		}
	}
}

// Sends the current count and client metrics to a specific client.
func (ws *WebSocket) sendCount(conn *websocket.Conn, database *db.Database) {
	currentCount := database.GetString()
	meter := ws.meterClients()
	message := fmt.Sprintf("%s,%d,%d", currentCount, meter.Increment, meter.Decrement)

	if err := conn.WriteMessage(websocket.TextMessage, []byte(message)); err != nil {
		log.Println("Error sending initial count:", err)
		ws.clients.Delete(conn)
		conn.Close()
	}
}

// Sends a message to all connected clients.
func (ws *WebSocket) broadcast(message string) {
	ws.clients.Range(func(key, value interface{}) bool {
		conn, ok := key.(*websocket.Conn)
		if !ok {
			return true
		}

		if err := conn.WriteMessage(websocket.TextMessage, []byte(message)); err != nil {
			log.Println("Error writing to client:", err)
			ws.clients.Delete(conn)
			conn.Close()
		}

		return true
	})
}

// Counts the number of clients in each status.
func (ws *WebSocket) meterClients() db.Meter {
	meter := db.Meter{}

	ws.clients.Range(func(_, value interface{}) bool {
		if status, ok := value.(Status); ok {
			switch status {
			case Increment:
				meter.Increment++
			case Decrement:
				meter.Decrement++
			}
		}
		return true
	})

	return meter
}

// Periodically updates the counter and broadcasts changes.
func (ws *WebSocket) Updater(database *db.Database) {
	var lastCount string
	ticker := time.NewTicker(250 * time.Millisecond)
	defer ticker.Stop()

	for range ticker.C {
		meter := ws.meterClients()
		database.UpdateCounter(meter)
		newCount := database.GetString()

		if newCount != lastCount {
			message := fmt.Sprintf("%s,%d,%d", newCount, meter.Increment, meter.Decrement)
			ws.broadcast(message)
			lastCount = newCount
		}
	}
}
