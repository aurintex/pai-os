mod bootstrap;

use anyhow::Result;
use bootstrap::{load_config, log_domain_stack_init, wait_for_shutdown_signal};
use clap::{ArgAction, Parser};
use pai_core::adapters::HardcodedFlowRunner;
use pai_core::domain::{EventBus, SessionManager};
use pai_core::ports::{InferenceError, InferencePort, InferenceRequest};
use pai_core::types::Token;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};

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

#[derive(Debug)]
struct StubInference;

impl InferencePort for StubInference {
    async fn infer(
        &self,
        req: InferenceRequest,
    ) -> Result<mpsc::Receiver<Result<Token, InferenceError>>, InferenceError> {
        let (tx, rx) = mpsc::channel(1);
        let content = format!("[stub] {}", req.prompt);
        // Spawn a task to honour the cancellation token and send the stub token.
        tokio::spawn(async move {
            tokio::select! {
                _ = req.cancellation.cancelled() => {}
                res = tx.send(Ok(Token { content })) => {
                    let _ = res;
                }
            }
        });
        Ok(rx)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // `RUST_LOG` overrides the verbosity flag; -v / -vv / -vvv set the fallback.
    let default_level = match args.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    common::logging::try_init(default_level)
        .map_err(|e| anyhow::anyhow!("tracing subscriber init failed: {e}"))?;

    load_config(args.config.as_deref())?;

    info!(target: "pai_engine::bootstrap", "pai-engine composition root starting");

    log_domain_stack_init();

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
        "shutdown complete; exiting pai-engine"
    );

    Ok(())
}
