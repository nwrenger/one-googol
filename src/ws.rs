use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use tokio::{
    sync::{broadcast::Sender, RwLock},
    time::{self, Duration},
};

use crate::{db::Database, util};

/// Websocket state
pub struct WebSocketState {
    pub database: RwLock<Database>,
    pub clients: RwLock<HashMap<usize, Client>>,
    pub sender: Sender<String>,
    pub next_client_id: RwLock<usize>,
}

/// Represents a connected WebSocket client
#[derive(Debug)]
pub struct Client {
    pub state: ClientState,
}

/// Client state
#[derive(Default, Clone, Debug)]
pub enum ClientState {
    #[default]
    Pending = 0,
    Increment,
    Decrement,
}

/// Client state count
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Meter {
    pub increment: u32,
    pub decrement: u32,
    pub pending: u32,
}

/// Spawns an updater threads which updates the count and sends that to the clients via channel
pub fn spawn_updater(ws_state: Arc<WebSocketState>) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(util::UPDATE_PERIOD_MS));
        loop {
            interval.tick().await;

            let client_states = {
                let clients = ws_state.clients.read().await;
                clients
                    .values()
                    .map(|client| client.state.clone())
                    .collect::<Vec<_>>()
            };

            let mut total_meter = Meter::default();
            for client_state in client_states {
                match client_state {
                    ClientState::Pending => total_meter.pending += 1,
                    ClientState::Increment => total_meter.increment += 1,
                    ClientState::Decrement => total_meter.decrement += 1,
                }
            }

            let mut db = ws_state.database.write().await;
            db.update_counter(&total_meter);
            let new_count = db.get_string();

            let message = format!(
                "{},{},{}",
                &new_count, total_meter.increment, total_meter.decrement
            );
            let _ = ws_state.sender.send(message);
        }
    });
}

/// WebSocket handler for the `/ws` route
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WebSocketState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handles an individual WebSocket connection
pub async fn handle_socket(stream: WebSocket, state: Arc<WebSocketState>) {
    let (mut sender, mut receiver) = stream.split();

    let mut rx = state.sender.subscribe();

    let client_id = {
        let mut id_lock = state.next_client_id.write().await;
        let id = *id_lock;
        *id_lock += 1;
        id
    };

    let mut clients = state.clients.write().await;
    clients.insert(
        client_id,
        Client {
            state: ClientState::default(),
        },
    );
    drop(clients);

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            match text.as_str() {
                "increment" => {
                    let mut clients = state.clients.write().await;
                    if let Some(client) = clients.get_mut(&client_id) {
                        client.state = ClientState::Increment;
                    }
                }
                "decrement" => {
                    let mut clients = state.clients.write().await;
                    if let Some(client) = clients.get_mut(&client_id) {
                        client.state = ClientState::Decrement;
                    }
                }
                _ => {
                    println!("Unknown command from client {}: {}", client_id, text);
                }
            }
        } else if let Message::Close(_) = message {
            break;
        }
    }

    let mut clients = state.clients.write().await;
    clients.remove(&client_id);
    drop(clients);

    send_task.abort();
}
