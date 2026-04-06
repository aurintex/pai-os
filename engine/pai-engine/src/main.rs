mod bootstrap;

use anyhow::Result;
use bootstrap::{load_config, log_domain_stack_init, wait_for_shutdown_signal};
use clap::{ArgAction, Parser};
use pai_core::adapters::HardcodedFlowRunner;
use pai_core::domain::{EventBus, SessionManager};
use pai_core::ports::{InferenceError, InferencePort};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;
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
    let args = Args::parse();

    let log_level = match args.verbose {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    load_config(args.config.as_deref())?;

    info!(target: "pai_engine::bootstrap", "pai-engine composition root starting");

    log_domain_stack_init();

    #[derive(Debug)]
    struct StubInference;

    impl InferencePort for StubInference {
        fn complete(&self, prompt: &str) -> Result<String, InferenceError> {
            Ok(format!("[stub] {prompt}"))
        }
    }

    let (event_bus, _event_rx) = EventBus::channel(64);
    let flow_runner = Arc::new(HardcodedFlowRunner::new(
        Arc::new(StubInference),
        event_bus.clone(),
    ));
    let session = SessionManager::new(flow_runner, event_bus);
    info!(
        target: "pai_engine::bootstrap",
        "session orchestration ready (state: {:?})",
        session.state_machine().state()
    );

    info!(
        target: "pai_engine::bootstrap",
        "engine main loop running (waiting for shutdown signal)"
    );

    wait_for_shutdown_signal().await;

    info!(
        target: "pai_engine::bootstrap",
        "shutdown complete — exiting pai-engine"
    );

    Ok(())
}
