use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

/// Command line arguments for the paiOS engine.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Parse command line arguments
    let args = Args::parse();

    // 2. Initialize logging (tracing)
    let log_level = if args.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_env_filter("info")
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // 3. Bootstrap engine (placeholder for now)
    info!("Booting paiOS engine workspace...");

    if let Some(path) = args.config {
        info!("Using configuration from: {}", path.display());
    } else {
        info!("No configuration file provided, using defaults.");
    }

    // TODO: In future steps, construct adapters and wire domain crates via `core`.

    // 4. Wait for shutdown signal to keep the process alive
    if let Err(e) = tokio::signal::ctrl_c().await {
        error!("Failed to listen for shutdown signal: {e}");
    }

    info!("Shutdown signal received. Exiting paiOS engine.");

    Ok(())
}
