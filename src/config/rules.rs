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
/// The [`Default`] implementation provides approximately 80 built-in
/// extension-to-category mappings covering documents, media, archives,
/// packages, development files, and other common types.
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
pub struct RulesConfig {
    pub rules: HashMap<String, Rule>,
}

impl Default for RulesConfig {
    /// Returns the built-in default rules (~79 extension mappings).
    fn default() -> Self {
        let mut rules = HashMap::new();

        // ── Documents ──────────────────────────────────────────────
        rules.insert(
            "pdf".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "PDF".into(),
            },
        );
        rules.insert(
            "doc".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Word".into(),
            },
        );
        rules.insert(
            "docx".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Word".into(),
            },
        );
        rules.insert(
            "odt".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Word".into(),
            },
        );
        rules.insert(
            "txt".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Text".into(),
            },
        );
        rules.insert(
            "md".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Text".into(),
            },
        );
        rules.insert(
            "rtf".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Text".into(),
            },
        );
        rules.insert(
            "tex".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Latex".into(),
            },
        );
        rules.insert(
            "ppt".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Presentations".into(),
            },
        );
        rules.insert(
            "pptx".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Presentations".into(),
            },
        );
        rules.insert(
            "odp".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Presentations".into(),
            },
        );
        rules.insert(
            "xls".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Spreadsheets".into(),
            },
        );
        rules.insert(
            "xlsx".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Spreadsheets".into(),
            },
        );
        rules.insert(
            "ods".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Spreadsheets".into(),
            },
        );
        rules.insert(
            "csv".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Data".into(),
            },
        );
        rules.insert(
            "json".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Data".into(),
            },
        );
        rules.insert(
            "xml".into(),
            Rule {
                category: "Documents".into(),
                subcategory: "Data".into(),
            },
        );

        // ── Media / Images ─────────────────────────────────────────
        rules.insert(
            "jpg".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images".into(),
            },
        );
        rules.insert(
            "jpeg".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images".into(),
            },
        );
        rules.insert(
            "png".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images".into(),
            },
        );
        rules.insert(
            "gif".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images".into(),
            },
        );
        rules.insert(
            "bmp".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images".into(),
            },
        );
        rules.insert(
            "svg".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images/Vectors".into(),
            },
        );
        rules.insert(
            "webp".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images".into(),
            },
        );
        rules.insert(
            "heic".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images".into(),
            },
        );
        rules.insert(
            "psd".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images/Photoshop".into(),
            },
        );
        rules.insert(
            "ai".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images/Illustrator".into(),
            },
        );
        rules.insert(
            "indd".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Images/InDesign".into(),
            },
        );

        // ── Media / Audio ──────────────────────────────────────────
        rules.insert(
            "mp3".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Audio".into(),
            },
        );
        rules.insert(
            "wav".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Audio".into(),
            },
        );
        rules.insert(
            "flac".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Audio".into(),
            },
        );
        rules.insert(
            "aac".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Audio".into(),
            },
        );
        rules.insert(
            "ogg".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Audio".into(),
            },
        );
        rules.insert(
            "m4a".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Audio".into(),
            },
        );

        // ── Media / Videos ─────────────────────────────────────────
        rules.insert(
            "mp4".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Videos".into(),
            },
        );
        rules.insert(
            "mov".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Videos".into(),
            },
        );
        rules.insert(
            "avi".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Videos".into(),
            },
        );
        rules.insert(
            "mkv".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Videos".into(),
            },
        );
        rules.insert(
            "flv".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Videos".into(),
            },
        );
        rules.insert(
            "wmv".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Videos".into(),
            },
        );
        rules.insert(
            "webm".into(),
            Rule {
                category: "Media".into(),
                subcategory: "Videos".into(),
            },
        );

        // ── Archives ───────────────────────────────────────────────
        rules.insert(
            "zip".into(),
            Rule {
                category: "Archives".into(),
                subcategory: "Archives".into(),
            },
        );
        rules.insert(
            "rar".into(),
            Rule {
                category: "Archives".into(),
                subcategory: "Archives".into(),
            },
        );
        rules.insert(
            "7z".into(),
            Rule {
                category: "Archives".into(),
                subcategory: "Archives".into(),
            },
        );
        rules.insert(
            "tar".into(),
            Rule {
                category: "Archives".into(),
                subcategory: "Archives".into(),
            },
        );
        rules.insert(
            "gz".into(),
            Rule {
                category: "Archives".into(),
                subcategory: "Archives".into(),
            },
        );
        rules.insert(
            "bz2".into(),
            Rule {
                category: "Archives".into(),
                subcategory: "Archives".into(),
            },
        );
        rules.insert(
            "xz".into(),
            Rule {
                category: "Archives".into(),
                subcategory: "Archives".into(),
            },
        );

        // ── Packages ───────────────────────────────────────────────
        rules.insert(
            "deb".into(),
            Rule {
                category: "Packages".into(),
                subcategory: "Packages".into(),
            },
        );
        rules.insert(
            "rpm".into(),
            Rule {
                category: "Packages".into(),
                subcategory: "Packages".into(),
            },
        );
        rules.insert(
            "exe".into(),
            Rule {
                category: "Packages".into(),
                subcategory: "Packages".into(),
            },
        );
        rules.insert(
            "msi".into(),
            Rule {
                category: "Packages".into(),
                subcategory: "Packages".into(),
            },
        );
        rules.insert(
            "dmg".into(),
            Rule {
                category: "Packages".into(),
                subcategory: "Packages".into(),
            },
        );
        rules.insert(
            "pkg".into(),
            Rule {
                category: "Packages".into(),
                subcategory: "Packages".into(),
            },
        );
        rules.insert(
            "appimage".into(),
            Rule {
                category: "Packages".into(),
                subcategory: "Packages".into(),
            },
        );

        // ── Development ────────────────────────────────────────────
        rules.insert(
            "sh".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Scripts".into(),
            },
        );
        rules.insert(
            "bash".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Scripts".into(),
            },
        );
        rules.insert(
            "py".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Python".into(),
            },
        );
        rules.insert(
            "js".into(),
            Rule {
                category: "Development".into(),
                subcategory: "JavaScript".into(),
            },
        );
        rules.insert(
            "ts".into(),
            Rule {
                category: "Development".into(),
                subcategory: "TypeScript".into(),
            },
        );
        rules.insert(
            "java".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Java".into(),
            },
        );
        rules.insert(
            "c".into(),
            Rule {
                category: "Development".into(),
                subcategory: "C".into(),
            },
        );
        rules.insert(
            "cpp".into(),
            Rule {
                category: "Development".into(),
                subcategory: "C++".into(),
            },
        );
        rules.insert(
            "cs".into(),
            Rule {
                category: "Development".into(),
                subcategory: "CSharp".into(),
            },
        );
        rules.insert(
            "go".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Go".into(),
            },
        );
        rules.insert(
            "rs".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Rust".into(),
            },
        );
        rules.insert(
            "php".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Web".into(),
            },
        );
        rules.insert(
            "html".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Web".into(),
            },
        );
        rules.insert(
            "css".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Web".into(),
            },
        );
        rules.insert(
            "scss".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Web".into(),
            },
        );
        rules.insert(
            "sql".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Databases".into(),
            },
        );
        rules.insert(
            "db".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Databases".into(),
            },
        );
        rules.insert(
            "sqlite".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Databases".into(),
            },
        );
        rules.insert(
            "dockerfile".into(),
            Rule {
                category: "Development".into(),
                subcategory: "Docker".into(),
            },
        );

        // ── Other ──────────────────────────────────────────────────
        rules.insert(
            "torrent".into(),
            Rule {
                category: "Other".into(),
                subcategory: "Torrents".into(),
            },
        );
        rules.insert(
            "iso".into(),
            Rule {
                category: "Other".into(),
                subcategory: "DiskImages".into(),
            },
        );
        rules.insert(
            "img".into(),
            Rule {
                category: "Other".into(),
                subcategory: "DiskImages".into(),
            },
        );
        rules.insert(
            "vdi".into(),
            Rule {
                category: "Other".into(),
                subcategory: "VirtualMachines".into(),
            },
        );
        rules.insert(
            "vbox".into(),
            Rule {
                category: "Other".into(),
                subcategory: "VirtualMachines".into(),
            },
        );

        RulesConfig { rules }
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
        // PoC has ~79 entries — require at least 45 as per spec
        assert!(
            config.rules.len() >= 45,
            "Expected >= 45 rules, got {}",
            config.rules.len()
        );
    }

    #[test]
    fn test_default_rules_have_expected_extensions() {
        let config = RulesConfig::default();
        // Spot-check a few well-known mappings
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

        // pdf should be overridden
        assert_eq!(merged.rules["pdf"].category, "MyDocs");
        assert_eq!(merged.rules["pdf"].subcategory, "PDFs");

        // mp3 should still be the default
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

        // All default rules should be there unchanged
        assert!(merged.rules.len() >= 45);
        assert_eq!(merged.rules["pdf"].category, "Documents");
        assert_eq!(merged.rules["rs"].subcategory, "Rust");
    }
}
