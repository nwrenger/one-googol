pub mod db;
pub mod util;
pub mod ws;

use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use db::Database;
use std::{collections::HashMap, net::ToSocketAddrs, path::PathBuf, sync::Arc};
use tokio::{
    sync::{broadcast, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder, ServiceExt};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{error, info};
use ws::{spawn_updater, ws_handler, WebSocketState};

/// Command-line arguments structure using Clap
#[derive(Parser)]
#[command(name = util::PKG_NAME)]
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

#[tokio::main]
async fn main() {
    util::logging();

    let args = Args::parse();

    if !PathBuf::from(&args.view).exists() {
        error!("The path for view content '{}' is invalid!", args.view);
        std::process::exit(1);
    }

    if !PathBuf::from(&args.db).exists() {
        error!("The path for the Database '{}' is invalid!", args.view);
        std::process::exit(1);
    }

    if !PathBuf::from(&args.cert).exists() {
        error!("The SSL certificate path '{}' does not exist!", args.cert);
        std::process::exit(1);
    }

    if !PathBuf::from(&args.key).exists() {
        error!("The SSL key path '{}' does not exist!", args.key);
        std::process::exit(1);
    }

    let mut db = Database::new();
    db.load_from_file(&args.db);

    let (sender, _) = broadcast::channel(100);

    let ws_state = Arc::new(WebSocketState {
        database: RwLock::new(db),
        clients: RwLock::new(HashMap::new()),
        sender,
        next_client_id: RwLock::new(1),
    });

    spawn_updater(ws_state.clone());

    let app = Router::new()
        .route("/ws", any(ws_handler).with_state(ws_state.clone()))
        .route("/", get(static_index).with_state(PathBuf::from(&args.view)))
        .route(
            "/{*file}",
            get(static_assets).with_state(PathBuf::from(&args.view)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        error!("Internal server error: {error}");
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    info!(
        "Server started on '{}' with frontend at '{}' and Database at '{}'",
        args.host, args.view, args.db
    );

    let handle = axum_server::Handle::new();
    let shut = util::shutdown_signal();

    let addr = args.host.to_socket_addrs().unwrap().next().unwrap();
    let tls = RustlsConfig::from_pem_file(&args.cert, &args.key)
        .await
        .unwrap();

    let server = axum_server::bind_rustls(addr, tls)
        .handle(handle.clone())
        .serve(app.into_make_service());

    tokio::select! {
        () = shut =>
            handle.graceful_shutdown(Some(Duration::from_secs(10))),
        res = server => res.unwrap(),
    }

    let db = ws_state.database.read().await;
    if let Err(e) = db.save_to_file(&args.db) {
        error!("Error saving count '{}' to file: {}", db.count, e);
    } else {
        info!("Count '{}' saved successfully to {}", db.count, &args.db);
    }
}

async fn static_index(State(path): State<PathBuf>, req: Request<Body>) -> impl IntoResponse {
    ServeFile::new(path.join("index.html"))
        .oneshot(req)
        .await
        .unwrap()
        .into_response()
}

async fn static_assets(
    State(path): State<PathBuf>,
    Path(file): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    if !file.contains('.') {
        ServeFile::new(path.join(file).with_extension("html"))
            .oneshot(req)
            .await
            .unwrap()
            .into_response()
    } else {
        ServeDir::new(path.clone())
            .oneshot(req)
            .await
            .unwrap()
            .into_response()
    }
}
