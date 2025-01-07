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
	clients sync.Map // map[*websocket.Conn]*Client
}

// Represents a WebSocket connection and its state.
type Client struct {
	conn   *websocket.Conn
	status Status
	mx     sync.Mutex
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

	client := &Client{
		conn:   conn,
		status: Pending,
	}
	ws.clients.Store(conn, client)
	ws.sendCount(client, database)

	go ws.handleConnection(client)
}

// Manages the WebSocket connection lifecycle.
func (ws *WebSocket) handleConnection(client *Client) {
	defer func() {
		ws.clients.Delete(client.conn)
		client.conn.Close()
	}()

	// Set read deadlines and handle ping/pong to keep the connection alive.
	client.conn.SetReadLimit(512)
	client.conn.SetReadDeadline(time.Now().Add(60 * time.Second))
	client.conn.SetPongHandler(func(string) error {
		client.conn.SetReadDeadline(time.Now().Add(60 * time.Second))
		return nil
	})

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
			client.status = Increment
		case "decrement":
			client.status = Decrement
		default:
			log.Printf("Unknown command: %s", command)
		}
	}
}

// Sends the current count and client metrics to a specific client.
func (ws *WebSocket) sendCount(client *Client, database *db.Database) {
	currentCount := database.GetString()
	meter := ws.meterClients()
	message := fmt.Sprintf("%s,%d,%d", currentCount, meter.Increment, meter.Decrement)

	// Messages need to be synchronously written to the websocket
	client.mx.Lock()
	err := client.conn.WriteMessage(websocket.TextMessage, []byte(message))
	client.mx.Unlock()

	if err != nil {
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
func (ws *WebSocket) meterClients() db.Meter {
	meter := db.Meter{}

	ws.clients.Range(func(_, value interface{}) bool {
		client, ok := value.(*Client)
		if ok {
			switch client.status {
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
