//! File classification by extension.
//!
//! Maps file extensions to category/subcategory pairs using the rules
//! configuration. The main entry point is [`classify_file`].

use crate::config::rules::{Rule, RulesConfig};
use crate::core::moving::Classification;
use crate::error::SortcrabError;
use std::path::Path;

/// Fallback category for files with unknown or missing extensions.
pub const FALLBACK_CATEGORY: &str = "Other";
/// Fallback subcategory for files with unknown or missing extensions.
pub const FALLBACK_SUBCATEGORY: &str = "Unknown";

/// Look up a file extension in the rules table.
///
/// Normalises the extension to lowercase and strips any leading dot before
/// performing the lookup.
///
/// # Example
///
/// ```rust
/// use sortcrab::config::rules::RulesConfig;
/// use sortcrab::core::classify::classify_extension;
///
/// let rules = RulesConfig::default();
/// let rule = classify_extension(&rules, "pdf").unwrap();
/// assert_eq!(rule.category, "Documents");
/// ```
pub fn classify_extension<'a>(rules: &'a RulesConfig, extension: &str) -> Option<&'a Rule> {
    let normalized = extension
        .trim()
        .trim_start_matches('.')
        .trim()
        .to_lowercase();
    rules.rules.get(&normalized)
}

/// Classify a file at `path` by its extension.
///
/// Extracts the extension from the path (using [`Path::extension`]), looks it up
/// via [`classify_extension`], and returns a [`Classification`] on success.
///
/// # Errors
/// Returns [`SortcrabError::UnknownExtension`] if:
/// - The file has no extension (e.g. `"Makefile"`)
/// - The extension is empty after normalisation
/// - The extension is not in the rules table
///
/// # Example
///
/// ```rust,no_run
/// use sortcrab::config::rules::RulesConfig;
/// use sortcrab::core::classify::classify_file;
/// use std::path::Path;
///
/// let rules = RulesConfig::default();
/// let class = classify_file(&rules, Path::new("report.pdf")).unwrap();
/// assert_eq!(class.category, "Documents");
/// ```
pub fn classify_file(rules: &RulesConfig, path: &Path) -> Result<Classification, SortcrabError> {
    // First try by extension (e.g. ".pdf" → "pdf").
    // If the file has no extension, fall back to its name (e.g. "Dockerfile").
    let lookup_key = path
        .extension()
        .and_then(|s| s.to_str())
        .or_else(|| path.file_name().and_then(|n| n.to_str()))
        .ok_or_else(|| SortcrabError::UnknownExtension("<unknown>".to_string()))?;

    match classify_extension(rules, lookup_key) {
        Some(rule) => Ok(Classification {
            category: rule.category.clone(),
            subcategory: rule.subcategory.clone(),
        }),
        None => Err(SortcrabError::UnknownExtension(lookup_key.to_string())),
    }
}

/// Classify a file, falling back to `Other/Unknown` for unknown extensions.
///
/// Unlike [`classify_file`], this function does not return
/// [`SortcrabError::UnknownExtension`]. Instead, files with unrecognized
/// extensions or no extension are classified under the fallback category
/// (`Other/Unknown`), ensuring every visible file gets moved somewhere
/// rather than being left behind.
///
/// # Example
///
/// ```rust
/// use sortcrab::config::rules::RulesConfig;
/// use sortcrab::core::classify::classify_or_fallback;
/// use std::path::Path;
///
/// let rules = RulesConfig::default();
/// let class = classify_or_fallback(&rules, Path::new("data.xyz123"));
/// assert_eq!(class.category, "Other");
/// assert_eq!(class.subcategory, "Unknown");
/// ```
pub fn classify_or_fallback(rules: &RulesConfig, path: &Path) -> Classification {
    match classify_file(rules, path) {
        Ok(c) => c,
        // classify_file currently only fails with UnknownExtension (pure
        // string lookups, no I/O), so any Err routes to the fallback.
        Err(_) => Classification {
            category: FALLBACK_CATEGORY.to_string(),
            subcategory: FALLBACK_SUBCATEGORY.to_string(),
        },
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rules() -> RulesConfig {
        RulesConfig::default()
    }

    // ── classify_extension ──────────────────────────────────────────

    #[test]
    fn known_extension() {
        let rules = test_rules();
        let rule = classify_extension(&rules, "pdf").unwrap();
        assert_eq!(rule.category, "Documents");
        assert_eq!(rule.subcategory, "PDF");
    }

    #[test]
    fn unknown_extension() {
        let rules = test_rules();
        assert!(classify_extension(&rules, "xyz123").is_none());
    }

    #[test]
    fn uppercase_extension() {
        let rules = test_rules();
        let rule = classify_extension(&rules, "PDF").unwrap();
        assert_eq!(rule.category, "Documents");
        assert_eq!(rule.subcategory, "PDF");
    }

    #[test]
    fn mixed_case_extension() {
        let rules = test_rules();
        let rule = classify_extension(&rules, "Mp3").unwrap();
        assert_eq!(rule.category, "Media");
        assert_eq!(rule.subcategory, "Audio");
    }

    #[test]
    fn empty_extension() {
        let rules = test_rules();
        assert!(classify_extension(&rules, "").is_none());
    }

    #[test]
    fn extension_with_dot_prefix() {
        let rules = test_rules();
        let rule = classify_extension(&rules, ".pdf").unwrap();
        assert_eq!(rule.category, "Documents");
        assert_eq!(rule.subcategory, "PDF");
    }

    #[test]
    fn extension_with_leading_and_trailing_whitespace() {
        let rules = test_rules();
        let rule = classify_extension(&rules, "  pdf  ").unwrap();
        assert_eq!(rule.category, "Documents");
        assert_eq!(rule.subcategory, "PDF");
    }

    // ── classify_file ───────────────────────────────────────────────

    #[test]
    fn classify_file_known() {
        let rules = test_rules();
        let class = classify_file(&rules, Path::new("report.pdf")).unwrap();
        assert_eq!(class.category, "Documents");
        assert_eq!(class.subcategory, "PDF");
    }

    #[test]
    fn classify_file_dockerfile() {
        let rules = test_rules();
        let class = classify_file(&rules, Path::new("Dockerfile")).unwrap();
        assert_eq!(class.category, "Development");
        assert_eq!(class.subcategory, "Docker");
    }

    #[test]
    fn classify_file_makefile_unknown() {
        let rules = test_rules();
        match classify_file(&rules, Path::new("Makefile")) {
            Err(SortcrabError::UnknownExtension(ref name)) => {
                assert_eq!(name, "Makefile");
            }
            _ => panic!("expected UnknownExtension for Makefile"),
        }
    }

    #[test]
    fn classify_file_unknown_extension() {
        let rules = test_rules();
        match classify_file(&rules, Path::new("data.xyz123")) {
            Err(SortcrabError::UnknownExtension(ref ext)) => {
                assert_eq!(ext, "xyz123");
            }
            _ => panic!("expected UnknownExtension for unknown extension"),
        }
    }

    #[test]
    fn classify_file_uppercase() {
        let rules = test_rules();
        match classify_file(&rules, Path::new("report.PDF")) {
            Ok(class) => {
                assert_eq!(class.category, "Documents");
                assert_eq!(class.subcategory, "PDF");
            }
            _ => panic!("expected Ok for uppercase .PDF"),
        }
    }

    #[test]
    fn classify_file_dotfile() {
        let rules = test_rules();
        match classify_file(&rules, Path::new(".gitignore")) {
            Err(SortcrabError::UnknownExtension(_)) => {}
            _ => panic!("expected UnknownExtension for dotfile"),
        }
    }

    // ── classify_or_fallback ───────────────────────────────────────

    #[test]
    fn test_classify_or_fallback_known() {
        let rules = test_rules();
        let class = classify_or_fallback(&rules, Path::new("report.pdf"));
        assert_eq!(class.category, "Documents");
        assert_eq!(class.subcategory, "PDF");
    }

    #[test]
    fn test_classify_or_fallback_unknown_ext() {
        let rules = test_rules();
        let class = classify_or_fallback(&rules, Path::new("data.xyz123"));
        assert_eq!(class.category, "Other");
        assert_eq!(class.subcategory, "Unknown");
    }

    #[test]
    fn test_classify_or_fallback_no_extension() {
        let rules = test_rules();
        let class = classify_or_fallback(&rules, Path::new("Makefile"));
        assert_eq!(class.category, "Other");
        assert_eq!(class.subcategory, "Unknown");
    }
}
