//! File sorting rules and extension-to-category mappings.
//!
//! [`RulesConfig`] holds the built-in ~80 extension mappings and supports
//! loading overrides from a TOML file via [`RulesConfig::from_toml`] and
//! merging via [`RulesConfig::merge`].

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// A single file-type rule specifying the destination category and subcategory.
///
/// # Example
///
/// ```rust
/// use sortcrab::config::rules::Rule;
///
/// let rule = Rule {
///     category: "Documents".into(),
///     subcategory: "PDF".into(),
/// };
/// assert_eq!(rule.category, "Documents");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub category: String,
    pub subcategory: String,
}

/// Configuration mapping file extensions to sorting rules.
///
/// The [`Default`] implementation loads approximately 80 built-in
/// extension-to-category mappings from an embedded TOML file covering
/// documents, media, archives, packages, development files, and other
/// common types.
///
/// # Example
///
/// ```rust
/// use sortcrab::config::rules::RulesConfig;
///
/// let config = RulesConfig::default();
/// assert!(config.rules.len() >= 45);
/// assert_eq!(config.rules["pdf"].category, "Documents");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RulesConfig {
    pub rules: HashMap<String, Rule>,
}

impl Default for RulesConfig {
    /// Returns the built-in default rules (~79 extension mappings).
    ///
    /// Loaded from the embedded `default.toml` at compile time via
    /// [`include_str!`].
    fn default() -> Self {
        let toml_str = include_str!("default.toml");
        toml::from_str(toml_str).expect("Built-in default.toml must be valid TOML")
    }
}

impl RulesConfig {
    /// Parse a `RulesConfig` from a TOML file at the given path.
    ///
    /// Expected TOML format:
    /// ```toml
    /// [rules]
    /// "pdf" = { category = "Documents", subcategory = "PDF" }
    /// ```
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self, crate::error::SortcrabError> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let config: RulesConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Merge another `RulesConfig` into this one.
    ///
    /// The user's rules take precedence over defaults on a per-extension basis.
    /// Any extension **not** present in the user config keeps its default value.
    pub fn merge(self, user: RulesConfig) -> RulesConfig {
        let mut merged = self;
        for (ext, rule) in user.rules {
            merged.rules.insert(ext, rule);
        }
        merged
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_default_rules_count() {
        let config = RulesConfig::default();
        assert!(
            config.rules.len() >= 45,
            "Expected >= 45 rules, got {}",
            config.rules.len()
        );
    }

    #[test]
    fn test_default_rules_have_expected_extensions() {
        let config = RulesConfig::default();
        assert_eq!(config.rules.get("pdf").unwrap().category, "Documents");
        assert_eq!(config.rules.get("pdf").unwrap().subcategory, "PDF");
        assert_eq!(config.rules.get("mp3").unwrap().category, "Media");
        assert_eq!(config.rules.get("mp3").unwrap().subcategory, "Audio");
        assert_eq!(config.rules.get("rs").unwrap().category, "Development");
        assert_eq!(config.rules.get("rs").unwrap().subcategory, "Rust");
        assert_eq!(config.rules.get("zip").unwrap().category, "Archives");
    }

    #[test]
    fn test_from_toml_parses() {
        let toml_str = r#"
[rules]
"pdf" = { category = "Documents", subcategory = "PDF" }
"mp3" = { category = "Media", subcategory = "Audio" }
"#;

        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "{}", toml_str).unwrap();
        let path = tmp.path();

        let config = RulesConfig::from_toml(path).unwrap();
        assert_eq!(config.rules.len(), 2);
        assert_eq!(config.rules["pdf"].category, "Documents");
        assert_eq!(config.rules["mp3"].subcategory, "Audio");
    }

    #[test]
    fn test_from_toml_file_not_found() {
        let result = RulesConfig::from_toml("/tmp/sortcrab_nonexistent_XXXX.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_overrides() {
        let defaults = RulesConfig::default();
        let mut user_rules = HashMap::new();
        user_rules.insert(
            "pdf".into(),
            Rule {
                category: "MyDocs".into(),
                subcategory: "PDFs".into(),
            },
        );
        let user_config = RulesConfig { rules: user_rules };

        let merged = defaults.merge(user_config);

        assert_eq!(merged.rules["pdf"].category, "MyDocs");
        assert_eq!(merged.rules["pdf"].subcategory, "PDFs");
        assert_eq!(merged.rules["mp3"].category, "Media");
        assert_eq!(merged.rules["mp3"].subcategory, "Audio");
    }

    #[test]
    fn test_merge_preserves_defaults() {
        let defaults = RulesConfig::default();
        let user_config = RulesConfig {
            rules: HashMap::new(),
        };

        let merged = defaults.merge(user_config);

        assert!(merged.rules.len() >= 45);
        assert_eq!(merged.rules["pdf"].category, "Documents");
        assert_eq!(merged.rules["rs"].subcategory, "Rust");
    }
}
