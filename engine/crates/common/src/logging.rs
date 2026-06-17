//! Centralised tracing initialisation for the paiOS engine.
//!
//! All crates use [`tracing`] macros for structured logging. The subscriber is
//! installed once at startup by the binary (`pai-engine`) via [`try_init`].
//! Log level is controlled at runtime by the `RUST_LOG` environment variable
//! (e.g. `RUST_LOG=info`, `RUST_LOG=pai_engine=debug`). A caller-supplied
//! default directive is used as fallback when `RUST_LOG` is not set.
//!
//! # Example
//!
//! ```rust,ignore
//! // This is illustrative; the real call happens in pai-engine/src/main.rs.
//! common::logging::try_init("info")?;
//! ```

use tracing_subscriber::{fmt, EnvFilter};

/// Build an [`EnvFilter`] from `RUST_LOG`, falling back to `default_directive`,
/// then to `"info"`.
///
/// Kept separate so it can be tested without touching global state.
fn env_filter(default_directive: &str) -> EnvFilter {
    EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default_directive))
        .unwrap_or_else(|_| EnvFilter::new("info"))
}

/// Install a `fmt` subscriber with [`EnvFilter`] as the global default.
///
/// Returns an error if a global subscriber has already been set (e.g. in tests
/// that call this more than once). Callers should propagate the error or
/// ignore it with `.ok()` in test harnesses.
///
/// # Errors
///
/// Returns a boxed error if a global subscriber is already set. In `pai-engine`
/// the error propagates via `?` into `anyhow::Error`.
pub fn try_init(
    default_directive: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    fmt()
        .with_env_filter(env_filter(default_directive))
        .try_init()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_filter_builds_without_rust_log() {
        // RUST_LOG is unset (or set to something valid); either way the filter
        // must be constructible without panicking.
        let _filter = env_filter("info");
    }

    #[test]
    fn env_filter_falls_back_to_default() {
        // Force fallback by using an invalid RUST_LOG value — the function
        // must still return a valid filter (the default directive).
        std::env::remove_var("RUST_LOG");
        let _filter = env_filter("warn");
    }
}
