// sortcrab — file classification

use crate::error::SortcrabError;
use crate::mover::Classification;
use crate::rules::{Rule, RulesConfig};
use std::path::Path;

/// Look up a file extension in the rules table.
///
/// Normalises the extension to lowercase and strips any leading dot before
/// performing the lookup.
pub fn classify_extension<'a>(
    rules: &'a RulesConfig,
    extension: &str,
) -> Option<&'a Rule> {
    let normalized = extension.trim().trim_start_matches('.').to_lowercase();
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
pub fn classify_file(
    rules: &RulesConfig,
    path: &Path,
) -> Result<Classification, SortcrabError> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| SortcrabError::UnknownExtension(
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("<unknown>")
                .to_string(),
        ))?;

    match classify_extension(rules, ext) {
        Some(rule) => Ok(Classification {
            category: rule.category.clone(),
            subcategory: rule.subcategory.clone(),
        }),
        None => Err(SortcrabError::UnknownExtension(ext.to_string())),
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
    fn classify_file_no_extension() {
        let rules = test_rules();
        match classify_file(&rules, Path::new("Makefile")) {
            Err(SortcrabError::UnknownExtension(ref name)) => {
                assert_eq!(name, "Makefile");
            }
            _ => panic!("expected UnknownExtension for file without extension"),
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
}
