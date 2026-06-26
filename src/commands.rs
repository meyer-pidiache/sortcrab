// sortcrab — sort command implementation

use std::fs;
use std::path::{Path, PathBuf};

use crate::classify::classify_file;
use crate::cli::SortArgs;
use crate::error::SortcrabError;
use crate::mover::{move_file, MoveOptions};
use crate::rules::RulesConfig;
use crate::semester::semester_from_time;

/// Statistics collected during a sort operation.
#[derive(Debug, Clone, Default)]
pub struct SortReport {
    /// Total number of files processed (excludes directories).
    pub total: usize,
    /// Files successfully moved to their organised destination.
    pub moved: usize,
    /// Files deliberately skipped (dotfiles, symlinks, already-organised).
    pub skipped: usize,
    /// Files that encountered an error (unknown extension, I/O failure, etc.).
    pub errors: usize,
}

/// Scan `source`, classify each file by extension, compute the semester from
/// the file's modification time, and move it into
/// `{target}/{category}/{subcategory}/{semester}/{filename}`.
///
/// Directories are silently skipped and do **not** count toward the total.
/// Per-file errors are collected in the returned [`SortReport`] — the function
/// never fails on individual items. If the source path is not a directory an
/// [`Err`] is returned immediately.
pub fn sort_files(
    source: &Path,
    target: &Path,
    rules: &RulesConfig,
) -> Result<SortReport, SortcrabError> {
    if !source.is_dir() {
        return Err(SortcrabError::InvalidPath(source.to_path_buf()));
    }

    let mut report = SortReport::default();

    let entries = fs::read_dir(source)?;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!("Failed to read directory entry: {e}");
                report.errors += 1;
                report.total += 1;
                continue;
            }
        };

        let path = entry.path();

        if path.is_dir() {
            tracing::debug!("Skipping directory: {}", path.display());
            continue;
        }

        report.total += 1;

        let filename = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => {
                tracing::warn!("Could not extract filename from: {}", path.display());
                report.errors += 1;
                continue;
            }
        };

        let classification = match classify_file(rules, &path) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Could not classify {}: {e}", path.display());
                report.errors += 1;
                continue;
            }
        };

        let modified = match fs::metadata(&path) {
            Ok(meta) => match meta.modified() {
                Ok(t) => t,
                Err(e) => {
                    tracing::warn!("Could not read modified time for {}: {e}", path.display());
                    report.errors += 1;
                    continue;
                }
            },
            Err(e) => {
                tracing::warn!("Could not read metadata for {}: {e}", path.display());
                report.errors += 1;
                continue;
            }
        };

        let semester = semester_from_time(&modified);

        let opts = MoveOptions {
            source: &path,
            target,
            classification: &classification,
            semester: &semester,
            filename: &filename,
        };

        match move_file(&opts) {
            Ok(dest) => {
                tracing::info!("Moved {} -> {}", path.display(), dest.display());
                report.moved += 1;
            }
            Err(SortcrabError::Skipped(reason)) => {
                tracing::debug!("Skipped {}: {reason}", path.display());
                report.skipped += 1;
            }
            Err(e) => {
                tracing::error!("Failed to move {}: {e}", path.display());
                report.errors += 1;
            }
        }
    }

    Ok(report)
}

/// Expand a leading tilde `~` in a path to the user's home directory.
///
/// This is needed because clap's `default_value = "~"` does not go through
/// shell expansion — the literal tilde must be resolved programmatically.
fn resolve_home(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    if let Some(stripped) = s.strip_prefix("~") {
        if let Ok(home) = std::env::var("HOME") {
            let after = stripped.trim_start_matches('/');
            if after.is_empty() {
                PathBuf::from(home)
            } else {
                PathBuf::from(home).join(after)
            }
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    }
}

/// Execute the `sort` subcommand.
///
/// Resolves the target directory (defaulting to the source directory for
/// in-place organization), loads the rules configuration, calls [`sort_files`],
/// and prints a human-readable summary.
pub fn execute_sort(args: &SortArgs) -> Result<(), SortcrabError> {
    let source = resolve_home(&args.source);
    let target: PathBuf = args
        .target
        .clone()
        .unwrap_or_else(|| source.clone());

    tracing::debug!("Sort source: {:?}, target: {:?}", source, target);

    let rules = RulesConfig::default();

    let report = sort_files(&source, &target, &rules)?;

    println!(
        "Sorted {} files, skipped {}, {} errors",
        report.moved, report.skipped, report.errors
    );

    tracing::info!(
        "Sort complete — total: {}, moved: {}, skipped: {}, errors: {}",
        report.total,
        report.moved,
        report.skipped,
        report.errors,
    );

    Ok(())
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::SystemTime;
    use tempfile::tempdir;

    fn setup_source_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }

    fn current_semester() -> String {
        semester_from_time(&SystemTime::now())
    }

    fn has_file_under(base: &Path, name: &str) -> bool {
        if !base.is_dir() {
            return false;
        }
        match fs::read_dir(base) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .any(|e| e.path().is_dir() && e.path().join(name).exists()),
            Err(_) => false,
        }
    }

    // ── sort_files tests ────────────────────────────────────────────

    #[test]
    fn test_basic_sort() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), "report.pdf", b"pdf content");
        setup_source_file(src.path(), "song.mp3", b"mp3 content");
        setup_source_file(src.path(), "main.rs", b"fn main() {}");

        let rules = RulesConfig::default();
        let report = sort_files(src.path(), tgt.path(), &rules).unwrap();

        assert_eq!(report.total, 3);
        assert_eq!(report.moved, 3);
        assert_eq!(report.skipped, 0);
        assert_eq!(report.errors, 0);

        assert!(!src.path().join("report.pdf").exists());
        assert!(!src.path().join("song.mp3").exists());
        assert!(!src.path().join("main.rs").exists());

        assert!(has_file_under(&tgt.path().join("Documents/PDF"), "report.pdf"));
        assert!(has_file_under(&tgt.path().join("Media/Audio"), "song.mp3"));
        assert!(has_file_under(&tgt.path().join("Development/Rust"), "main.rs"));
    }

    #[test]
    fn test_basic_sort_exact_path() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), "report.pdf", b"pdf content");

        let rules = RulesConfig::default();
        let _report = sort_files(src.path(), tgt.path(), &rules).unwrap();

        let sem = current_semester();
        let expected = tgt.path().join(format!("Documents/PDF/{sem}/report.pdf"));
        assert!(
            expected.exists(),
            "Expected file at {}",
            expected.display()
        );
    }

    #[test]
    fn test_empty_directory() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        let rules = RulesConfig::default();
        let report = sort_files(src.path(), tgt.path(), &rules).unwrap();

        assert_eq!(report.total, 0);
        assert_eq!(report.moved, 0);
        assert_eq!(report.skipped, 0);
        assert_eq!(report.errors, 0);
    }

    #[test]
    fn test_dotfile_skipped() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), ".hidden.txt", b"secret");
        setup_source_file(src.path(), "visible.pdf", b"visible");

        let rules = RulesConfig::default();
        let report = sort_files(src.path(), tgt.path(), &rules).unwrap();

        assert_eq!(report.total, 2);
        assert_eq!(report.moved, 1);
        assert_eq!(report.skipped, 1);
        assert_eq!(report.errors, 0);

        assert!(src.path().join(".hidden.txt").exists());
    }

    #[test]
    fn test_unknown_extension_counts_as_error() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), "data.xyz123", b"unknown");

        let rules = RulesConfig::default();
        let report = sort_files(src.path(), tgt.path(), &rules).unwrap();

        assert_eq!(report.total, 1);
        assert_eq!(report.moved, 0);
        assert_eq!(report.skipped, 0);
        assert_eq!(report.errors, 1);
    }

    #[test]
    fn test_symlink_skipped() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), "real.pdf", b"pdf");
        #[cfg(unix)]
        std::os::unix::fs::symlink(src.path().join("real.pdf"), src.path().join("link.pdf")).unwrap();

        // Debug: list source contents before sort
        let src_entries: Vec<_> = std::fs::read_dir(src.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        eprintln!("DEBUG [test_symlink_skipped] source before sort: {src_entries:?}");

        // Verify preconditions
        assert!(
            src.path().join("real.pdf").exists(),
            "precondition failed: real.pdf must exist before sort"
        );

        let rules = RulesConfig::default();
        let report = sort_files(src.path(), tgt.path(), &rules).unwrap();

        // Both entries are counted (neither is a directory), but the symlink
        // is rejected by move_file.
        assert_eq!(report.total, 2, "should count both real.pdf and link.pdf");
        assert_eq!(
            report.moved, 1,
            "real.pdf should be moved (got {}); skipped={}, errors={}",
            report.moved, report.skipped, report.errors
        );
        assert_eq!(report.skipped, 1, "link.pdf symlink should be skipped");
        assert_eq!(report.errors, 0, "no errors expected");

        // Intermediate state: real.pdf must have been moved
        assert!(
            !src.path().join("real.pdf").exists(),
            "real.pdf should no longer be in source after move"
        );
        assert!(
            has_file_under(&tgt.path().join("Documents/PDF"), "real.pdf"),
            "real.pdf should exist in target Documents/PDF; src still contains: {:?}",
            std::fs::read_dir(src.path())
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_source_not_a_directory() {
        let src = tempdir().unwrap();
        let file = setup_source_file(src.path(), "file.pdf", b"x");

        let rules = RulesConfig::default();
        let result = sort_files(&file, src.path(), &rules);

        assert!(result.is_err());
        match result.unwrap_err() {
            SortcrabError::InvalidPath(p) => assert_eq!(p, file),
            other => panic!("expected InvalidPath, got: {other:?}"),
        }
    }

    #[test]
    fn test_mixed_scenario() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        // Subdirectory — should be skipped entirely, not counted.
        fs::create_dir(src.path().join("subdir")).unwrap();

        setup_source_file(src.path(), "doc.pdf", b"pdf");
        setup_source_file(src.path(), ".hidden.txt", b"secret");
        setup_source_file(src.path(), "unknown.xyz", b"???");
        setup_source_file(src.path(), "song.mp3", b"mp3");

        let rules = RulesConfig::default();
        let report = sort_files(src.path(), tgt.path(), &rules).unwrap();

        assert_eq!(report.total, 4);
        assert_eq!(report.moved, 2); // doc.pdf, song.mp3
        assert_eq!(report.skipped, 1); // .hidden.txt
        assert_eq!(report.errors, 1); // unknown.xyz

        // Source directory should still exist (we never remove it).
        assert!(src.path().join("subdir").is_dir());
    }
}
