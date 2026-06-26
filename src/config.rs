// sortcrab — configuration loading, saving, and path resolution

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::SortcrabError;
use crate::rules::RulesConfig;

/// Top-level sortcrab configuration persisted as TOML.
///
/// ```toml
/// version = "1"
///
/// [rules]
/// "pdf" = { category = "Documents", subcategory = "PDF" }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortcrabConfig {
    pub rules: RulesConfig,

    /// Schema version for future migration support.
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_version() -> String {
    "1".to_string()
}

impl Default for SortcrabConfig {
    fn default() -> Self {
        SortcrabConfig {
            rules: RulesConfig::default(),
            version: default_version(),
        }
    }
}

/// Stateless config-file manager.
///
/// All methods are associated functions operating on the user's config dir
/// discovered via [`directories::ProjectDirs`].
pub struct ConfigManager;

impl ConfigManager {
    /// Returns `~/.config/sortcrab/config.toml`.
    pub fn config_path() -> Result<PathBuf, SortcrabError> {
        let proj_dirs = directories::ProjectDirs::from("com", "", "sortcrab")
            .ok_or_else(|| SortcrabError::Config("could not determine config directory".into()))?;
        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    /// Returns `~/.config/sortcrab/`.
    pub fn config_dir() -> Result<PathBuf, SortcrabError> {
        let proj_dirs = directories::ProjectDirs::from("com", "", "sortcrab")
            .ok_or_else(|| SortcrabError::Config("could not determine config directory".into()))?;
        Ok(proj_dirs.config_dir().to_path_buf())
    }

    /// Load configuration from disk, or return defaults if the file doesn't exist.
    pub fn load() -> Result<SortcrabConfig, SortcrabError> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let config: SortcrabConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(SortcrabConfig::default())
        }
    }

    /// Create the config directory and write the default configuration file.
    pub fn create_default() -> Result<(), SortcrabError> {
        let path = Self::config_path()?;
        let dir = Self::config_dir()?;

        std::fs::create_dir_all(&dir)?;

        let config = SortcrabConfig::default();
        let toml_str = toml::to_string_pretty(&config)
            .map_err(|e| SortcrabError::Config(e.to_string()))?;
        std::fs::write(&path, toml_str)?;

        tracing::info!("Created default config at {:?}", path);
        Ok(())
    }

    /// Read the config file and print its contents to stdout.
    pub fn print() -> Result<(), SortcrabError> {
        let path = Self::config_path()?;
        let content = std::fs::read_to_string(&path)?;
        println!("{}", content);
        Ok(())
    }

    /// Open the config file in `$EDITOR` (falls back to `vi`).
    pub fn edit() -> Result<(), SortcrabError> {
        let path = Self::config_path()?;
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());

        let status = std::process::Command::new(&editor)
            .arg(&path)
            .status()
            .map_err(|e| {
                SortcrabError::Config(format!("failed to launch editor '{}': {}", editor, e))
            })?;

        if !status.success() {
            return Err(SortcrabError::Config(format!(
                "editor '{}' exited with error",
                editor
            )));
        }

        Ok(())
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path_ends_with_config_toml() {
        let path = ConfigManager::config_path().unwrap();
        assert!(path.ends_with("config.toml"));
    }

    #[test]
    fn test_config_dir_ends_with_sortcrab() {
        let path = ConfigManager::config_dir().unwrap();
        assert!(path.ends_with("sortcrab"));
    }

    #[test]
    fn test_sortcrab_config_default() {
        let config = SortcrabConfig::default();
        assert!(!config.rules.rules.is_empty(), "default rules should not be empty");
        assert_eq!(config.version, "1");
    }

    #[test]
    fn test_sortcrab_config_toml_roundtrip() {
        let config = SortcrabConfig::default();
        let toml_str = toml::to_string_pretty(&config)
            .expect("serialization should succeed");
        let parsed: SortcrabConfig = toml::from_str(&toml_str)
            .expect("deserialization should succeed");
        assert_eq!(parsed.rules.rules.len(), config.rules.rules.len());
        assert_eq!(parsed.version, "1");
    }

    #[test]
    fn test_load_returns_defaults_when_config_missing() {
        let path = ConfigManager::config_path().unwrap();
        let existed = path.exists();
        if existed {
            std::fs::remove_file(&path).unwrap();
        }

        let config = ConfigManager::load().unwrap();
        assert!(!config.rules.rules.is_empty(), "fallback config should have rules");
        assert_eq!(config.version, "1");

        if existed {
            ConfigManager::create_default().unwrap();
        }
    }

    #[test]
    fn test_create_default_and_load() {
        let path = ConfigManager::config_path().unwrap();

        let _ = std::fs::remove_file(&path);

        ConfigManager::create_default().unwrap();
        assert!(path.exists(), "config file should exist after create_default");

        let config = ConfigManager::load().unwrap();
        assert!(!config.rules.rules.is_empty(), "loaded config should have rules");
        assert_eq!(config.version, "1");

        std::fs::remove_file(&path).unwrap();
    }
}
