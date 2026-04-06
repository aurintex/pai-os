use anyhow::Result;
use clap::{ArgAction, Parser};
use pai_core::adapters::HardcodedFlowRunner;
use pai_core::domain::{EventBus, SessionManager};
use pai_core::ports::{InferenceError, InferencePort};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, error, info};
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

    // 3. Bootstrap engine — core session orchestration (stub inference until real adapters wire in)
    #[derive(Debug)]
    struct StubInference;

    impl InferencePort for StubInference {
        fn complete(&self, prompt: &str) -> Result<String, InferenceError> {
            Ok(format!("[stub] {prompt}"))
        }
    }

    let (event_bus, event_rx) = EventBus::channel(64);
    let flow_runner = Arc::new(HardcodedFlowRunner::new(
        Arc::new(StubInference),
        event_bus.clone(),
    ));
    let session = SessionManager::new(flow_runner, event_bus);

    // Keep the sole consumer alive so the mpsc channel stays open; drain events so publishes never
    // fail with Closed/Full during normal operation.
    tokio::spawn(async move {
        let mut event_rx = event_rx;
        while let Some(ev) = event_rx.recv().await {
            debug!(target: "pai_engine::event_bus", ?ev, "domain event");
        }
    });
    info!(
        "Booting paiOS engine workspace (session state: {:?})...",
        session.state_machine().state()
    );

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
