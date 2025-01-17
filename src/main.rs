pub mod counter;
pub mod util;
pub mod ws;

use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    extract::{Path, State},
    http::{HeaderValue, Request, StatusCode},
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use counter::Counter;
use std::{net::TcpListener, path::PathBuf};
use tokio::{sync::broadcast, time::Duration};
use tower::{BoxError, ServiceBuilder, ServiceExt};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
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
    view: PathBuf,

    /// Path to the persistent counter file
    #[arg(short, default_value = "counter.json")]
    counter: PathBuf,

    /// Path to the SSL certificate
    #[arg(
        long,
        default_value = "/etc/letsencrypt/live/one-googol.nwrenger.dev/fullchain.pem"
    )]
    cert: PathBuf,

    /// Path to the SSL private key
    #[arg(
        long,
        default_value = "/etc/letsencrypt/live/one-googol.nwrenger.dev/privkey.pem"
    )]
    key: PathBuf,
}

#[tokio::main]
async fn main() {
    util::logging();

    let args = Args::parse();

    if !PathBuf::from(&args.view).exists() {
        error!("The path for view content {:?} is invalid!", args.view);
        std::process::exit(1);
    }

    if !PathBuf::from(&args.cert).exists() {
        error!("The SSL certificate path {:?} does not exist!", args.cert);
        std::process::exit(1);
    }

    if !PathBuf::from(&args.key).exists() {
        error!("The SSL key path {:?} does not exist!", args.key);
        std::process::exit(1);
    }

    let mut counter = Counter::new();
    counter.load_from_file(&args.counter);

    let (sender, _) = broadcast::channel(100);

    let ws_state = WebSocketState::new(counter, sender);

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
                .layer(
                    CorsLayer::new()
                        .allow_origin(args.host.parse::<HeaderValue>().unwrap())
                        .allow_methods(Any),
                )
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

    let handle = axum_server::Handle::new();
    let shut = util::shutdown_signal();

    let tcp = TcpListener::bind(&args.host).unwrap();
    let tls = RustlsConfig::from_pem_file(&args.cert, &args.key)
        .await
        .unwrap();

    info!("Server started on \"{}\"", args.host);

    let server = axum_server::from_tcp_rustls(tcp, tls)
        .handle(handle.clone())
        .serve(app.into_make_service());

    tokio::select! {
        () = shut =>
            handle.graceful_shutdown(Some(Duration::from_secs(10))),
        res = server => res.unwrap(),
    }

    let counter = ws_state.counter.read().await;
    if let Err(e) = counter.save_to_file(&args.counter) {
        error!("Error saving \"{:?}\" to file: {}", counter, e);
    } else {
        info!(
            "\"{:?}\" saved successfully to {:?}",
            counter, &args.counter
        );
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
