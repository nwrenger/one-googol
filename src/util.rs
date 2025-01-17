use num_bigint::BigInt;
use serde::{Deserialize, Deserializer, Serializer};
use tokio::signal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const ONE_GOOGOL: &str = "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
pub const UPDATE_PERIOD_MS: u64 = 250;

/// Initialize tracing
pub fn logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Waits for a shutdown signal (`ctrl` + `c`)
pub async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
}

/// Convert a BigInt to its decimal string representation
pub fn serialize_bigint<S>(bigint: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&bigint.to_str_radix(10))
}

/// Parse a string back to a BigInt
pub fn deserialize_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    BigInt::parse_bytes(s.as_bytes(), 10)
        .ok_or_else(|| serde::de::Error::custom("Failed to parse BigInt from string"))
}
