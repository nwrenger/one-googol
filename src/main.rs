use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::Request,
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use clap::Parser;
use futures::{SinkExt, StreamExt};
use num_bigint::BigInt;
use num_traits::Signed;
use num_traits::Zero;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::{
    net::TcpListener,
    signal,
    sync::{
        broadcast::{self, Sender},
        RwLock,
    },
    time::{self, Duration},
};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const ONE_GOOGOL: &str = "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const UPDATE_PERIOD_MS: u64 = 250;

/// Command-line arguments structure using Clap
#[derive(Parser)]
#[command(name = PKG_NAME)]
struct Args {
    /// Host in the format ip:port
    host: String,

    /// Path to the view folder
    #[arg(short, default_value = "view")]
    view: String,

    /// Path to the database file
    #[arg(short, default_value = "db.txt")]
    db: String,

    /// Path to the SSL certificate
    #[arg(
        long,
        default_value = "/etc/letsencrypt/live/one-googol.nwrenger.dev/fullchain.pem"
    )]
    cert: String,

    /// Path to the SSL private key
    #[arg(
        long,
        default_value = "/etc/letsencrypt/live/one-googol.nwrenger.dev/privkey.pem"
    )]
    key: String,
}

/// Client state count
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Meter {
    increment: i64,
    decrement: i64,
    pending: i64,
}

/// Manages counter and should be accessible via state.
#[derive(Debug)]
struct Database {
    count: BigInt,
}

impl Database {
    /// Creates a new Database with count initialized to zero
    fn new() -> Self {
        Self {
            count: BigInt::zero(),
        }
    }

    /// Loads the counter from a plain text file if it exists
    fn load_from_file(&mut self, filename: &str) {
        if let Ok(content) = fs::read_to_string(filename) {
            if let Ok(count) = content.trim().parse::<BigInt>() {
                self.count = count;
                println!("Loaded count from file: {}", self.count);
            } else {
                println!("Invalid big.Int string in file: {}", content);
                self.count = BigInt::zero();
            }
        } else {
            println!(
                "Database file '{}' not found. Starting with zero.",
                filename
            );
        }
    }

    /// Saves the current counter to a plain text file
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        fs::write(filename, self.get_string())
    }

    /// Gets the current count as a string
    fn get_string(&self) -> String {
        self.count.to_str_radix(10)
    }

    /// Updates the counter based on the provided meter
    fn update_counter(&mut self, meter: &Meter) {
        let cmp_step = compute_step(&self.count);

        // Compute stepIncrement = meter.increment ^ cmp_step
        let step_increment = BigInt::from(meter.increment).pow(cmp_step);

        // Add stepIncrement to the count
        self.count += step_increment;

        // Ensure count does not exceed 10^100
        let one_googol = BigInt::parse_bytes(ONE_GOOGOL.as_bytes(), 10).unwrap();
        if self.count > one_googol {
            self.count = one_googol.clone();
        }

        // Compute stepDecrement = meter.decrement ^ cmp_step
        let step_decrement = BigInt::from(meter.decrement).pow(cmp_step);

        // Subtract stepDecrement from the count
        self.count -= step_decrement;

        // Ensure count does not go below zero
        if self.count < BigInt::zero() {
            self.count = BigInt::zero();
        }
    }
}

/// Function to compute the square root of the number of digits in the counter
fn compute_step(counter: &BigInt) -> u32 {
    let abs_value = counter.abs();
    let digit_length = abs_value.to_str_radix(10).len();
    (digit_length as f64).sqrt() as u32
}

/// Shared application state
struct AppState {
    database: RwLock<Database>,
    clients: RwLock<HashMap<usize, Client>>,
    sender: Sender<String>,
    next_client_id: RwLock<usize>,
}

/// Represents a connected WebSocket client
struct Client {
    state: ClientState,
}

/// Client state
#[derive(Default, Clone)]
enum ClientState {
    #[default]
    Pending = 0,
    Increment,
    Decrement,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if !PathBuf::from(&args.view).exists() {
        eprintln!("The path for view content '{}' is invalid!", args.view);
        std::process::exit(1);
    }

    let db_dir = PathBuf::from(&args.db)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    if !db_dir.exists() {
        eprintln!(
            "The directory for the Database '{}' does not exist!",
            db_dir.display()
        );
        std::process::exit(1);
    }

    if !PathBuf::from(&args.cert).exists() {
        eprintln!("The SSL certificate path '{}' does not exist!", args.cert);
        std::process::exit(1);
    }

    if !PathBuf::from(&args.key).exists() {
        eprintln!("The SSL key path '{}' does not exist!", args.key);
        std::process::exit(1);
    }

    let mut db = Database::new();
    db.load_from_file(&args.db);

    let (sender, _) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        database: RwLock::new(db),
        clients: RwLock::new(HashMap::new()),
        sender,
        next_client_id: RwLock::new(1),
    });

    let updater_state = app_state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(UPDATE_PERIOD_MS));
        loop {
            interval.tick().await;

            let client_states = {
                let clients = updater_state.clients.read().await;
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

            let mut db = updater_state.database.write().await;
            db.update_counter(&total_meter);
            let new_count = db.get_string();

            let message = format!(
                "{},{},{}",
                &new_count, total_meter.increment, total_meter.decrement
            );
            let _ = updater_state.sender.send(message);
        }
    });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/", get(static_index))
        .route("/{*file}", get(static_assets))
        .layer(Extension(app_state.clone()))
        .with_state(PathBuf::from(args.view.clone()));

    println!(
        "Server started on '{}' with frontend at '{}' and Database at '{}'",
        args.host, args.view, args.db
    );

    let listener = TcpListener::bind(&args.host).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    let db = app_state.database.read().await;
    if let Err(e) = db.save_to_file(&args.db) {
        eprintln!("\nError saving count to file: {}", e);
    } else {
        println!("\nCount saved successfully to {}", args.db);
    }
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
}

/// WebSocket handler for the `/ws` route
async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handles an individual WebSocket connection
async fn handle_socket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();

    let mut rx = state.sender.subscribe();

    let client_id = {
        let mut id_lock = state.next_client_id.write().await;
        let id = *id_lock;
        *id_lock += 1;
        id
    };

    {
        let mut clients = state.clients.write().await;
        clients.insert(
            client_id,
            Client {
                state: ClientState::default(),
            },
        );
    }

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

    {
        let mut clients = state.clients.write().await;
        clients.remove(&client_id);
    }

    send_task.abort();
}

async fn static_index(State(dir): State<PathBuf>, req: Request<Body>) -> impl IntoResponse {
    ServeFile::new(dir.join("index.html"))
        .oneshot(req)
        .await
        .unwrap()
        .into_response()
}

async fn static_assets(
    State(dir): State<PathBuf>,
    Path(file): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    if !file.contains('.') {
        ServeFile::new(dir.join(file).with_extension("html"))
            .oneshot(req)
            .await
            .unwrap()
            .into_response()
    } else {
        ServeDir::new(dir.clone())
            .oneshot(req)
            .await
            .unwrap()
            .into_response()
    }
}
