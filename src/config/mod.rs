//! Configuration loading, saving, and path resolution.
//!
//! sortcrab uses a TOML configuration file at `~/.config/sortcrab/config.toml`.
//! The [`ConfigManager`] type provides stateless associated functions for
//! loading, creating, printing, and editing the config file.

pub mod rules;

#[cfg(test)]
use std::cell::RefCell;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::SortcrabError;

#[cfg(test)]
thread_local! {
    /// Override for the config directory during tests.
    /// When set, [`config_path`](ConfigManager::config_path) and
    /// [`config_dir`](ConfigManager::config_dir) return paths under this
    /// directory instead of the real user config directory.
    static TEST_CONFIG_DIR: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

/// Controls semester-based subdirectory organisation.
///
/// When enabled, files are grouped by configurable periods
/// (`2025-I`, `2025-II`, …) under their category/subcategory tree.
///
/// `months_per_period` controls how many months fall in each period
/// (6 = semesters, 4 = trimesters, 3 = quarters, 12 = yearly).
///
/// `folder_format` is a template string that supports `{year}`,
/// `{period}` (1-based numeric), and `{roman}` (Roman numeral).
///
/// # Example TOML
///
/// ```toml
/// [semester]
/// enabled = true
/// months_per_period = 3
/// folder_format = "{year}Q{period}"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemesterConfig {
    /// Whether to group files by semester.
    #[serde(default = "default_semester_enabled")]
    pub enabled: bool,

    /// Number of months per academic period (default: 6).
    #[serde(default = "default_months_per_period")]
    pub months_per_period: u32,

    /// Folder-name template. Variables: {year}, {period}, {roman}.
    #[serde(default = "default_folder_format")]
    pub folder_format: String,
}

fn default_semester_enabled() -> bool {
    true
}

fn default_months_per_period() -> u32 {
    6
}

fn default_folder_format() -> String {
    "{year}-{roman}".to_string()
}

impl Default for SemesterConfig {
    fn default() -> Self {
        SemesterConfig {
            enabled: default_semester_enabled(),
            months_per_period: default_months_per_period(),
            folder_format: default_folder_format(),
        }
    }
}

/// Top-level sortcrab configuration persisted as TOML.
///
/// Fields are ordered so that `version` and `[semester]` appear first in the
/// serialised file, before the long `[rules.*]` section.
///
/// # Example TOML
///
/// ```toml
/// version = "1"
///
/// [semester]
/// enabled = true
///
/// [rules]
/// "pdf" = { category = "Documents", subcategory = "PDF" }
/// ```
///
/// # Example
///
/// ```rust
/// use sortcrab::config::SortcrabConfig;
///
/// let config = SortcrabConfig::default();
/// assert_eq!(config.version, "1");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortcrabConfig {
    /// Schema version for future migration support.
    #[serde(default = "default_version")]
    pub version: String,

    /// Controls semester-based subdirectory grouping.
    #[serde(default)]
    pub semester: SemesterConfig,

    /// Extension-to-category mapping rules.
    #[serde(default)]
    pub rules: rules::RulesConfig,
}

fn default_version() -> String {
    "1".to_string()
}

impl Default for SortcrabConfig {
    fn default() -> Self {
        SortcrabConfig {
            version: default_version(),
            semester: SemesterConfig::default(),
            rules: rules::RulesConfig::default(),
        }
    }
}

/// Stateless config-file manager.
///
/// All methods are associated functions operating on the user's config dir
/// discovered via [`directories::ProjectDirs`].
pub struct ConfigManager;

impl ConfigManager {
    #[cfg(test)]
    pub fn set_test_config_dir(path: PathBuf) {
        TEST_CONFIG_DIR.with(|d| *d.borrow_mut() = Some(path));
    }

    #[cfg(test)]
    pub fn clear_test_config_dir() {
        TEST_CONFIG_DIR.with(|d| *d.borrow_mut() = None);
    }

    /// Returns `~/.config/sortcrab/config.toml`.
    pub fn config_path() -> Result<PathBuf, SortcrabError> {
        #[cfg(test)]
        if let Some(dir) = TEST_CONFIG_DIR.with(|d| d.borrow().clone()) {
            return Ok(dir.join("config.toml"));
        }
        let proj_dirs = directories::ProjectDirs::from("com", "", "sortcrab")
            .ok_or_else(|| SortcrabError::Config("could not determine config directory".into()))?;
        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    /// Returns `~/.config/sortcrab/`.
    pub fn config_dir() -> Result<PathBuf, SortcrabError> {
        #[cfg(test)]
        if let Some(dir) = TEST_CONFIG_DIR.with(|d| d.borrow().clone()) {
            return Ok(dir);
        }
        let proj_dirs = directories::ProjectDirs::from("com", "", "sortcrab")
            .ok_or_else(|| SortcrabError::Config("could not determine config directory".into()))?;
        Ok(proj_dirs.config_dir().to_path_buf())
    }

    /// Load configuration from disk, or return defaults if the file doesn't exist.
    ///
    /// User-specified rules are merged with built-in defaults so unlisted
    /// extensions keep their default classifications. The `[semester]` section
    /// and `version` come entirely from the user file.
    pub fn load() -> Result<SortcrabConfig, SortcrabError> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let user_config: SortcrabConfig = toml::from_str(&content)?;
            let defaults = SortcrabConfig::default();
            let merged_rules = defaults.rules.merge(user_config.rules);
            Ok(SortcrabConfig {
                rules: merged_rules,
                semester: user_config.semester,
                version: user_config.version,
            })
        } else {
            Ok(SortcrabConfig::default())
        }
    }

    /// Create the config directory and write the default configuration file
    /// with all built-in extension rules in inline `[rules]` format.
    ///
    /// Inline format (`ext = { category = "...", subcategory = "..." }`) is
    /// used instead of the expanded `[rules.ext]` table format so the file is
    /// more compact and easier to scan.
    pub fn create_default() -> Result<(), SortcrabError> {
        let path = Self::config_path()?;
        let dir = Self::config_dir()?;

        std::fs::create_dir_all(&dir)?;

        let config = SortcrabConfig::default();
        let mut toml = String::new();

        toml.push_str(&format!("version = \"{}\"\n", config.version));
        toml.push_str("\n[semester]\n");
        toml.push_str(&format!("enabled = {}\n", config.semester.enabled));
        toml.push_str(&format!(
            "months_per_period = {}\n",
            config.semester.months_per_period
        ));
        toml.push_str(&format!(
            "folder_format = \"{}\"\n",
            config.semester.folder_format
        ));
        toml.push_str("\n[rules]\n");
        let mut extensions: Vec<&String> = config.rules.rules.keys().collect();
        extensions.sort();
        for ext in extensions {
            let rule = &config.rules.rules[ext];
            toml.push_str(&format!(
                "{ext} = {{ category = \"{}\", subcategory = \"{}\" }}\n",
                rule.category, rule.subcategory,
            ));
        }

        std::fs::write(&path, toml)?;

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

        #[cfg(target_os = "linux")]
        assert!(path.ends_with("sortcrab"), "expected ~/.config/sortcrab");

        #[cfg(target_os = "macos")]
        assert!(
            path.ends_with("com.sortcrab"),
            "expected ~/Library/Application Support/com.sortcrab"
        );

        #[cfg(target_os = "windows")]
        assert!(
            path.parent().unwrap().ends_with("sortcrab"),
            "expected %APPDATA%/sortcrab/config"
        );
    }

    #[test]
    fn test_sortcrab_config_default() {
        let config = SortcrabConfig::default();
        assert!(
            !config.rules.rules.is_empty(),
            "default rules should not be empty"
        );
        assert_eq!(config.version, "1");
    }

    #[test]
    fn test_sortcrab_config_toml_roundtrip() {
        let config = SortcrabConfig::default();
        let toml_str = toml::to_string(&config).expect("serialization should succeed");
        let parsed: SortcrabConfig =
            toml::from_str(&toml_str).expect("deserialization should succeed");
        assert_eq!(parsed.rules.rules.len(), config.rules.rules.len());
        assert_eq!(parsed.version, "1");
    }

    #[test]
    fn test_load_returns_defaults_when_config_missing() {
        let tmp = tempfile::tempdir().unwrap();
        ConfigManager::set_test_config_dir(tmp.path().to_path_buf());

        let config = ConfigManager::load().unwrap();
        assert!(
            !config.rules.rules.is_empty(),
            "fallback config should have rules"
        );
        assert_eq!(config.version, "1");

        ConfigManager::clear_test_config_dir();
    }

    #[test]
    fn test_create_default_and_load() {
        let tmp = tempfile::tempdir().unwrap();
        ConfigManager::set_test_config_dir(tmp.path().to_path_buf());

        let path = ConfigManager::config_path().unwrap();

        ConfigManager::create_default().unwrap();
        assert!(
            path.exists(),
            "config file should exist after create_default"
        );

        let config = ConfigManager::load().unwrap();
        assert!(
            !config.rules.rules.is_empty(),
            "loaded config should have rules"
        );
        assert_eq!(config.version, "1");

        ConfigManager::clear_test_config_dir();
    }
}
