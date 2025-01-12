pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const ONE_GOOGOL: &str = "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
pub const UPDATE_PERIOD_MS: u64 = 250;

/// Initialize tracing
pub fn logging() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Wait for a shutdown signal (`ctrl` + `c`)
pub async fn shutdown_signal() {
    use tokio::signal;
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
}
