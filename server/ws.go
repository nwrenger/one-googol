package main

import (
	"fmt"
	"log"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

// Upgrades HTTP connections to WebSocket connections.
var upgrader = websocket.Upgrader{
	ReadBufferSize:    1024,
	WriteBufferSize:   1024,
	EnableCompression: false,
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
}

// Send data updates to peer with this period.
const updatePeriod = 250 * time.Millisecond

type WebSocket struct {
	clients  sync.Map // map[*websocket.Conn]*Client
	database *Database
}

// Represents a WebSocket connection and its state.
type Client struct {
	conn   *websocket.Conn
	status Status
	mx     sync.RWMutex
}

// Client's state.
type Status int

const (
	Disconnected Status = iota
	Pending
	Increment
	Decrement
)

// Creates a new websocket and runs an Updater in Background
func NewWebSocket(database *Database) *WebSocket {
	websocket := WebSocket{database: database}
	go websocket.Updater()
	return &websocket
}

// Upgrades the HTTP connection to a WebSocket and handles the connection.
func (ws *WebSocket) WsHandler(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("WebSocket Upgrade error:", err)
		return
	}

	// Init client
	client := &Client{
		conn:   conn,
		status: Pending,
	}
	ws.clients.Store(conn, client)
	ws.sendCount(client)

	// Close everything on quit
	defer func() {
		ws.clients.Delete(client.conn)
		client.conn.Close()
	}()

	// Messages loop
	for {
		messageType, msg, err := client.conn.ReadMessage()
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
			client.mx.Lock()
			client.status = Increment
			client.mx.Unlock()
		case "decrement":
			client.mx.Lock()
			client.status = Decrement
			client.mx.Unlock()
		default:
			log.Printf("Unknown command: %s", command)
		}
	}
}

// Sends the current count and client metrics to a specific client.
func (ws *WebSocket) sendCount(client *Client) {
	currentCount := ws.database.GetString()
	meter := ws.meterClients()
	message := fmt.Sprintf("%s,%d,%d", currentCount, meter.Increment, meter.Decrement)

	client.mx.Lock()
	defer client.mx.Unlock()

	if err := client.conn.WriteMessage(websocket.TextMessage, []byte(message)); err != nil {
		log.Println("Error sending initial count:", err)
		ws.clients.Delete(client.conn)
		client.conn.Close()
	}
}

// Sends a message to all connected clients.
func (ws *WebSocket) broadcast(message string) {
	ws.clients.Range(func(key, value interface{}) bool {
		client, ok := value.(*Client)
		if !ok {
			return true
		}

		// Messages need to be synchronously written to the websocket
		client.mx.Lock()
		err := client.conn.WriteMessage(websocket.TextMessage, []byte(message))
		client.mx.Unlock()

		if err != nil {
			log.Println("Error writing to client:", err)
			ws.clients.Delete(client.conn)
			client.conn.Close()
		}

		return true
	})
}

// Counts the number of clients in each status.
func (ws *WebSocket) meterClients() Meter {
	meter := Meter{}

	ws.clients.Range(func(_, value interface{}) bool {
		client, ok := value.(*Client)
		client.mx.RLock()
		if ok {
			switch client.status {
			case Pending:
				meter.Pending++
			case Increment:
				meter.Increment++
			case Decrement:
				meter.Decrement++
			}
		}
		client.mx.RUnlock()
		return true
	})

	return meter
}

// Periodically updates the counter and broadcasts changes to peer.
func (ws *WebSocket) Updater() {
	var lastCount string
	var lastMeter Meter
	ticker := time.NewTicker(updatePeriod)
	defer ticker.Stop()

	for range ticker.C {
		newMeter := ws.meterClients()
		ws.database.UpdateCounter(newMeter)
		newCount := ws.database.GetString()

		if newCount != lastCount || lastMeter != newMeter {
			message := fmt.Sprintf("%s,%d,%d", newCount, newMeter.Increment, newMeter.Decrement)
			ws.broadcast(message)
			lastCount = newCount
		}
	}
}
