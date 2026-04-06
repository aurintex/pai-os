//! Filesystem-backed configuration using TOML, YAML, or JSON.

use crate::domain::ConfigError;
use crate::ports::ConfigProvider;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

/// On-disk format for a configuration file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigFileFormat {
    Toml,
    Yaml,
    Json,
}

/// Loads configuration from a file, supporting TOML, YAML, and JSON.
///
/// Format resolution:
/// 1. If constructed with [`FileConfigAdapter::with_format`], that format is always used.
/// 2. Otherwise, the file extension is used (`.toml`, `.yaml`, `.yml`, `.json`).
/// 3. If the extension is missing or unrecognized, the content is inspected (see [`sniff_format`]).
#[derive(Clone, Copy, Debug, Default)]
pub struct FileConfigAdapter {
    format_override: Option<ConfigFileFormat>,
}

impl FileConfigAdapter {
    pub fn new() -> Self {
        Self {
            format_override: None,
        }
    }

    /// Force a single format for every [`ConfigProvider::load`] call (ignores extension and sniffing).
    pub fn with_format(format: ConfigFileFormat) -> Self {
        Self {
            format_override: Some(format),
        }
    }

    fn resolve_format(&self, path: &Path, content: &str) -> ConfigFileFormat {
        if let Some(fmt) = self.format_override {
            return fmt;
        }
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_ascii_lowercase().as_str() {
                "toml" => return ConfigFileFormat::Toml,
                "yaml" | "yml" => return ConfigFileFormat::Yaml,
                "json" => return ConfigFileFormat::Json,
                _ => {}
            }
        }
        sniff_format(content)
    }
}

impl ConfigProvider for FileConfigAdapter {
    fn load<T: DeserializeOwned>(&self, path: &Path) -> Result<T, ConfigError> {
        let content = fs::read_to_string(path).map_err(|e| ConfigError::io(path, e))?;
        let fmt = self.resolve_format(path, &content);
        parse_config(&content, fmt, path)
    }
}

fn parse_config<T: DeserializeOwned>(
    content: &str,
    fmt: ConfigFileFormat,
    path: &Path,
) -> Result<T, ConfigError> {
    match fmt {
        ConfigFileFormat::Toml => {
            toml::from_str(content).map_err(|e| ConfigError::parse(path, "TOML", e))
        }
        ConfigFileFormat::Yaml => {
            serde_yaml::from_str(content).map_err(|e| ConfigError::parse(path, "YAML", e))
        }
        ConfigFileFormat::Json => {
            serde_json::from_str(content).map_err(|e| ConfigError::parse(path, "JSON", e))
        }
    }
}

/// Infer format from leading content when extension is missing or unknown.
///
/// Heuristics: JSON objects start with `{`; TOML array-of-tables with `[[`; a single TOML table
/// `[name]` is distinguished from a JSON array; YAML documents may start with `---` or `%YAML`.
pub fn sniff_format(content: &str) -> ConfigFileFormat {
    let t = content.trim_start();
    if t.is_empty() {
        return ConfigFileFormat::Toml;
    }
    if t.starts_with('{') {
        return ConfigFileFormat::Json;
    }
    if t.starts_with("[[") {
        return ConfigFileFormat::Toml;
    }
    if let Some(after_bracket) = t.strip_prefix('[') {
        for c in after_bracket.chars() {
            if c.is_whitespace() {
                continue;
            }
            if c.is_ascii_alphabetic() || c == '_' {
                return ConfigFileFormat::Toml;
            }
            break;
        }
        return ConfigFileFormat::Json;
    }
    if t.starts_with("---") || t.starts_with("%YAML") {
        return ConfigFileFormat::Yaml;
    }
    ConfigFileFormat::Toml
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::io::Write;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Sample {
        name: String,
        count: u32,
    }

    #[test]
    fn extension_toml() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("cfg.toml");
        fs::write(
            &path,
            r#"
name = "pai"
count = 2
"#,
        )
        .unwrap();
        let adapter = FileConfigAdapter::new();
        let v: Sample = adapter.load(&path).expect("load");
        assert_eq!(
            v,
            Sample {
                name: "pai".into(),
                count: 2
            }
        );
    }

    #[test]
    fn extension_yaml() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("cfg.yaml");
        fs::write(
            &path,
            "name: pai\ncount: 2\n",
        )
        .unwrap();
        let adapter = FileConfigAdapter::new();
        let v: Sample = adapter.load(&path).expect("load");
        assert_eq!(
            v,
            Sample {
                name: "pai".into(),
                count: 2
            }
        );
    }

    #[test]
    fn extension_json() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("cfg.json");
        fs::write(
            &path,
            r#"{"name": "pai", "count": 2}"#,
        )
        .unwrap();
        let adapter = FileConfigAdapter::new();
        let v: Sample = adapter.load(&path).expect("load");
        assert_eq!(
            v,
            Sample {
                name: "pai".into(),
                count: 2
            }
        );
    }

    #[test]
    fn with_format_overrides_extension() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("cfg.txt");
        fs::write(&path, r#"{"name": "pai", "count": 2}"#).unwrap();
        let adapter = FileConfigAdapter::with_format(ConfigFileFormat::Json);
        let v: Sample = adapter.load(&path).expect("load");
        assert_eq!(v.count, 2);
    }

    #[test]
    fn sniff_json_without_extension() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("noext");
        let mut f = fs::File::create(&path).unwrap();
        write!(f, r#"{{"name": "x", "count": 1}}"#).unwrap();
        drop(f);
        let adapter = FileConfigAdapter::new();
        let v: Sample = adapter.load(&path).expect("load");
        assert_eq!(v.name, "x");
    }

    #[test]
    fn unknown_extension_sniffs_toml() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("cfg.conf");
        fs::write(&path, "name = \"p\"\ncount = 1\n").unwrap();
        let adapter = FileConfigAdapter::new();
        let v: Sample = adapter.load(&path).expect("load");
        assert_eq!(v.name, "p");
    }

    #[test]
    fn sniff_format_cases() {
        assert_eq!(sniff_format(r#"{"a":1}"#), ConfigFileFormat::Json);
        assert_eq!(sniff_format("[[t]]\na=1"), ConfigFileFormat::Toml);
        assert_eq!(sniff_format("[table]\na=1"), ConfigFileFormat::Toml);
        assert_eq!(sniff_format("[1,2]"), ConfigFileFormat::Json);
        assert_eq!(sniff_format("---\nname: x"), ConfigFileFormat::Yaml);
    }
}
