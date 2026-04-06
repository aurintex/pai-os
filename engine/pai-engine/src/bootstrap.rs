//! Composition root helpers: domain crate visibility, config load, shutdown signals.
//!
//! Concrete adapters for vision/audio/api/… are feature-gated in their crates; this module
//! ensures each domain crate is part of the engine binary and emits structured bootstrap logs.

use anyhow::{Context, Result};
use std::path::Path;
use tracing::info;

/// Load optional TOML/JSON config from disk when the path exists; otherwise continue with defaults.
pub fn load_config(path: Option<&Path>) -> Result<()> {
    match path {
        Some(p) if p.exists() => {
            let _cfg = config::Config::builder()
                .add_source(config::File::from(p))
                .build()
                .with_context(|| format!("failed to load config from {}", p.display()))?;
            info!(
                target: "pai_engine::config",
                "loaded configuration from {}",
                p.display()
            );
        }
        Some(p) => {
            info!(
                target: "pai_engine::config",
                "config path {} not found; using defaults",
                p.display()
            );
        }
        None => {
            info!(
                target: "pai_engine::config",
                "no configuration file; using built-in defaults"
            );
        }
    }
    Ok(())
}

/// Emit structured logs and resolve optional domain crate dependencies for the active feature set.
pub fn log_domain_stack_init() {
    use common as _;

    info!(
        target: "pai_engine::bootstrap",
        "common: shared domain types and ports (linked)"
    );
    info!(
        target: "pai_engine::bootstrap",
        "pai-core: SessionManager, EventBus, FlowRunner (active)"
    );

    #[cfg(any(
        feature = "vision_mock",
        feature = "vision_v4l2",
        feature = "vision_rga",
        feature = "vision_image"
    ))]
    {
        use vision as _;
        info!(target: "pai_engine::bootstrap", "vision: crate linked");
    }

    #[cfg(any(
        feature = "audio_mock",
        feature = "audio_cpal",
        feature = "audio_webrtc"
    ))]
    {
        use audio as _;
        info!(
            target: "pai_engine::bootstrap",
            "audio: crate linked"
        );
    }

    #[cfg(any(
        feature = "infer_mock",
        feature = "infer_llamacpp_cpu",
        feature = "infer_rknn",
        feature = "infer_rkllm",
        feature = "infer_sherpa",
        feature = "infer_hailo",
        feature = "infer_mcp_client"
    ))]
    {
        use inference as _;
        info!(
            target: "pai_engine::bootstrap",
            "inference: crate linked"
        );
    }

    #[cfg(any(
        feature = "api_mock",
        feature = "api_grpc_uds",
        feature = "api_grpc_tcp",
        feature = "api_http",
        feature = "api_mcp_server"
    ))]
    {
        use api as _;
        info!(target: "pai_engine::bootstrap", "api: crate linked");
    }

    #[cfg(any(
        feature = "periph_mock",
        feature = "periph_desktop",
        feature = "periph_desktop_hid",
        feature = "periph_gpio",
        feature = "periph_evdev",
        feature = "periph_usb_hid"
    ))]
    {
        use peripherals as _;
        info!(
            target: "pai_engine::bootstrap",
            "peripherals: crate linked"
        );
    }
}

/// Block until Ctrl-C or SIGTERM (Unix). Used as the engine main-loop shutdown trigger.
pub async fn wait_for_shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("register SIGTERM");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.expect("listen for ctrl-c");
    }
}
