//! Adapters implementing `common` ports (e.g., file-based config, SQLite permissions).

mod file_config;

pub use file_config::{ConfigFileFormat, FileConfigAdapter, sniff_format};
