use crate::{
    counter::{CountMeter, Counter, PollMeter},
    util,
};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{broadcast::Sender, RwLock},
    time::{self, Duration},
};

/// Websocket state
#[derive(Debug)]
pub struct WebSocketState {
    pub counter: RwLock<Counter>,
    pub clients: RwLock<HashMap<usize, Client>>,
    pub sender: Sender<String>,
    pub next_client_id: RwLock<usize>,
}

impl WebSocketState {
    pub fn new(counter: Counter, sender: Sender<String>) -> Arc<Self> {
        Arc::new(Self {
            counter: RwLock::new(counter),
            clients: RwLock::new(HashMap::new()),
            sender,
            next_client_id: RwLock::new(1),
        })
    }
}

/// Represents a connected WebSocket client
#[derive(Debug, Default)]
pub struct Client {
    pub counter_state: CounterState,
    pub action_clicks: usize,
    pub poll_state: PollState,
}

/// Client counter state
#[derive(Default, Clone, Debug, PartialEq)]
pub enum CounterState {
    #[default]
    Pending = 0,
    Increment,
    Decrement,
}

impl CounterState {
    /// Counts clients states
    pub fn meter_counter(counter_states: &[Self]) -> CountMeter {
        let mut meter = CountMeter::default();
        for client_state in counter_states {
            match client_state {
                Self::Pending => meter.pending += 1,
                Self::Increment => meter.increment += 1,
                Self::Decrement => meter.decrement += 1,
            }
        }
        meter
    }
}

/// Client poll state
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PollState {
    #[default]
    Pending = 0,
    Base,
    Exponent,
}

impl PollState {
    /// Counts clients states
    pub fn meter_poll(poll_states: &[Self]) -> PollMeter {
        let mut meter = PollMeter::default();
        for poll_state in poll_states {
            match poll_state {
                Self::Pending => meter.pending += 1,
                Self::Base => meter.base += 1,
                Self::Exponent => meter.exponent += 1,
            }
        }
        meter
    }
}

/// Spawns an updater threads which updates the count and sends that to the clients via a channel
pub fn spawn_updater(state: Arc<WebSocketState>) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(util::UPDATE_PERIOD_MS));
        let mut before = Counter::default();
        loop {
            interval.tick().await;

            let (counter_states, poll_states) = {
                let clients = state.clients.read().await;
                clients
                    .values()
                    .map(|client| {
                        (
                            (client.counter_state.clone(), client.action_clicks),
                            client.poll_state.clone(),
                        )
                    })
                    .collect::<(Vec<(_, _)>, Vec<_>)>()
            };

            // Reset clicks
            for client in state.clients.write().await.values_mut() {
                client.action_clicks = 0;
            }

            let mut counter = state.counter.write().await;

            counter.update_poll(&poll_states);
            counter.update_count(&counter_states);

            if before != *counter {
                let message = serde_json::to_string(&*counter).unwrap();
                let _ = state.sender.send(message);
                before = counter.clone();
            }

            drop(counter);
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
async fn handle_socket(stream: WebSocket, state: Arc<WebSocketState>) {
    let (mut sender, mut receiver) = stream.split();

    let mut rx = state.sender.subscribe();

    let client_id = {
        let mut id_lock = state.next_client_id.write().await;
        let id = *id_lock;
        *id_lock += 1;
        id
    };

    let mut clients = state.clients.write().await;
    clients.insert(client_id, Client::default());
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
                        client.counter_state = CounterState::Increment;
                    }
                }
                "decrement" => {
                    let mut clients = state.clients.write().await;
                    if let Some(client) = clients.get_mut(&client_id) {
                        client.counter_state = CounterState::Decrement;
                    }
                }
                "base" => {
                    let mut clients = state.clients.write().await;
                    if let Some(client) = clients.get_mut(&client_id) {
                        client.poll_state = PollState::Base;
                    }
                }
                "exponent" => {
                    let mut clients = state.clients.write().await;
                    if let Some(client) = clients.get_mut(&client_id) {
                        client.poll_state = PollState::Exponent;
                    }
                }
                "action" => {
                    let mut clients = state.clients.write().await;
                    if let Some(client) = clients.get_mut(&client_id) {
                        client.action_clicks += 1;
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
