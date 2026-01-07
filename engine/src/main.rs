//! ## paiOS Engine Runtime
//!
//! This is the main entry point for the paiOS Engine daemon.
//! It handles CLI argument parsing, logging initialization, and boots the core engine library.
//!
//! ## Usage
//!
//! ```bash
//! ./pai-engine --config path/to/config.toml --debug
//! ```

use anyhow::Result;
use clap::Parser;
use pai_engine::PaiEngine;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

/// The command line arguments for the paiOS Engine.
///
/// # Fields
///
/// - `config`: The path to the configuration file.
/// - `debug`: Enable verbose logging.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    debug: bool,
}

/// The main function for the paiOS Engine.
///
/// # Errors
///
/// This function can return the following errors:
/// - `Result<()>`: A result indicating success or failure.
/// - `EngineError(String)`: An error with a message.
#[tokio::main]
async fn main() -> Result<()> {
    // 1. Parse Command Line Arguments
    let args = Args::parse();

    // 2. Initialize Logging (Tracing)
    let log_level = if args.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // 3. Start the Engine
    info!("Booting paiOS Engine...");

    let engine = PaiEngine::new(args.config);

    if let Err(e) = engine.start().await {
        error!("Engine failed to start: {}", e);
        std::process::exit(1);
    }

    // Keep the process running (simulation)
    // In a real daemon, you would wait for a shutdown signal here (Ctrl+C)
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received. Exiting.");

    Ok(())
}
