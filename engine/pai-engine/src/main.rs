use anyhow::Result;
use clap::{ArgAction, Parser};
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

    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Parse command line arguments
    let args = Args::parse();

    // 2. Initialize logging (tracing) based on verbosity
    let log_level = match args.verbose {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

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
