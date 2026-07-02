//! Core domain logic: classification, file moving, semester computation, and orchestration.
//!
//! The main entry point is [`sort_files`], which ties together the
//! classification, semester, and moving subsystems.

pub mod classify;
pub mod moving;
pub mod semester;

use std::collections::HashSet;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::core::moving::Classification;

use crate::config::SemesterConfig;
use crate::config::rules::RulesConfig;
use crate::core::classify::classify_file;
use crate::core::moving::{MoveOptions, move_file, resolve_collision};
use crate::core::semester::semester_label;
use crate::error::SortcrabError;

/// A recorded move action, used for tree display rendering.
#[derive(Debug, Clone)]
pub struct MoveRecord {
    /// The destination path relative to the target directory.
    pub dest_relative: PathBuf,
    /// Whether this was a dry-run prediction rather than an actual move.
    pub dry_run: bool,
}

/// Statistics collected during a sort operation.
///
/// # Example
///
/// ```rust
/// use sortcrab::core::SortReport;
///
/// let report = SortReport {
///     total: 10,
///     moved: 8,
///     skipped: 1,
///     errors: 1,
///     moves: vec![],
/// };
/// assert_eq!(report.moved + report.skipped + report.errors, report.total);
/// ```
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
    /// Individual move records for tree display rendering.
    pub moves: Vec<MoveRecord>,
}

/// Scan `source`, classify each file by extension, compute the semester from
/// the file's modification time, and move it into
/// `{target}/{category}/{subcategory}/{semester}/{filename}`.
///
/// When `semester_cfg.enabled` is `false`, the semester directory component is
/// omitted from the destination path.  [`SemesterConfig`] also controls the
/// period length and folder-name template.
///
/// When `dry_run` is `true`, no files are actually moved — the intended
/// destinations are logged via `log::info!`.
///
/// Directories are silently skipped and do **not** count toward the total.
/// Per-file errors are collected in the returned [`SortReport`] — the function
/// never fails on individual items. If the source path is not a directory an
/// [`Err`] is returned immediately.
///
/// # Errors
/// Returns [`SortcrabError::InvalidPath`] if `source` is not a directory.
///
/// # Example
///
/// ```rust,no_run
/// use sortcrab::core::sort_files;
/// use sortcrab::config::rules::RulesConfig;
/// use sortcrab::config::SemesterConfig;
/// use std::path::Path;
///
/// let rules = RulesConfig::default();
/// let semester = SemesterConfig::default();
/// let report = sort_files(Path::new("/tmp/source"), Path::new("/tmp/target"), &rules, false, &semester)?;
/// println!("Moved {} files", report.moved);
/// # Ok::<_, sortcrab::error::SortcrabError>(())
/// ```
pub fn sort_files(
    source: &Path,
    target: &Path,
    rules: &RulesConfig,
    dry_run: bool,
    semester_cfg: &SemesterConfig,
) -> Result<SortReport, SortcrabError> {
    if !source.is_dir() {
        return Err(SortcrabError::InvalidPath(source.to_path_buf()));
    }

    let mut report = SortReport::default();

    // Track predicted destinations during dry-run so collision resolution
    // produces correct -1, -2, … suffixes for same-batch conflicts.
    let mut predicted: HashSet<PathBuf> = HashSet::new();

    let entries = fs::read_dir(source)?;

    for entry in entries {
        let Some(path) = resolve_entry(entry, &mut report) else {
            continue;
        };

        if path.is_dir() {
            log::debug!("Skipping directory: {}", path.display());
            continue;
        }

        report.total += 1;

        process_single_file(
            &path,
            target,
            rules,
            dry_run,
            semester_cfg,
            &mut report,
            &mut predicted,
        );
    }

    Ok(report)
}

fn resolve_entry(entry: Result<DirEntry, io::Error>, report: &mut SortReport) -> Option<PathBuf> {
    match entry {
        Ok(e) => Some(e.path()),
        Err(e) => {
            log::warn!("Failed to read directory entry: {e}");
            report.errors += 1;
            report.total += 1;
            None
        }
    }
}

fn process_single_file(
    path: &Path,
    target: &Path,
    rules: &RulesConfig,
    dry_run: bool,
    semester_cfg: &SemesterConfig,
    report: &mut SortReport,
    predicted: &mut HashSet<PathBuf>,
) {
    let Some(filename) = extract_filename(path, report) else {
        return;
    };

    if filename.starts_with('.') {
        log::debug!("Skipping dotfile: {}", path.display());
        report.skipped += 1;
        return;
    }

    let Some(classification) = classify_or_skip(path, rules, report) else {
        return;
    };

    let Some(modified) = read_modified_time(path, report) else {
        return;
    };

    let semester = if semester_cfg.enabled {
        semester_label(
            &modified,
            semester_cfg.months_per_period,
            &semester_cfg.folder_format,
        )
    } else {
        String::new()
    };

    if check_already_organised(path, target, &classification, &semester) {
        log::debug!("Already organised: {}", path.display());
        report.skipped += 1;
        return;
    }

    execute_move(
        path,
        target,
        &classification,
        &semester,
        &filename,
        dry_run,
        report,
        predicted,
    );
}

fn extract_filename(path: &Path, report: &mut SortReport) -> Option<String> {
    match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => Some(name.to_string()),
        None => {
            log::warn!("Could not extract filename from: {}", path.display());
            report.errors += 1;
            None
        }
    }
}

fn classify_or_skip(
    path: &Path,
    rules: &RulesConfig,
    report: &mut SortReport,
) -> Option<Classification> {
    let classification = match classify_file(rules, path) {
        Ok(c) => c,
        Err(e) => {
            log::warn!("Could not classify {}: {e}", path.display());
            report.errors += 1;
            return None;
        }
    };

    // fs::metadata follows symlinks and would error on a broken
    // symlink whose target was already moved in this pass.
    if let Ok(meta) = fs::symlink_metadata(path)
        && meta.is_symlink()
    {
        log::debug!("Skipping symlink: {}", path.display());
        report.skipped += 1;
        return None;
    }

    Some(classification)
}

fn read_modified_time(path: &Path, report: &mut SortReport) -> Option<SystemTime> {
    match fs::metadata(path) {
        Ok(meta) => match meta.modified() {
            Ok(t) => Some(t),
            Err(e) => {
                log::warn!("Could not read modified time for {}: {e}", path.display());
                report.errors += 1;
                None
            }
        },
        Err(e) => {
            log::warn!("Could not read metadata for {}: {e}", path.display());
            report.errors += 1;
            None
        }
    }
}

fn check_already_organised(
    path: &Path,
    target: &Path,
    classification: &Classification,
    semester: &str,
) -> bool {
    let dest_dir = target
        .join(&classification.category)
        .join(&classification.subcategory)
        .join(semester);
    let Ok(dest_canonical) = std::fs::canonicalize(&dest_dir) else {
        return false;
    };
    let Ok(source_canonical) = std::fs::canonicalize(path) else {
        return false;
    };
    source_canonical.starts_with(&dest_canonical)
}

#[allow(clippy::too_many_arguments)]
fn execute_move(
    path: &Path,
    target: &Path,
    classification: &Classification,
    semester: &str,
    filename: &str,
    dry_run: bool,
    report: &mut SortReport,
    predicted: &mut HashSet<PathBuf>,
) {
    let dest_dir = target
        .join(&classification.category)
        .join(&classification.subcategory)
        .join(semester);

    if dry_run {
        let dest = resolve_collision(&dest_dir, filename, predicted);
        predicted.insert(dest.clone());
        report.moves.push(MoveRecord {
            dest_relative: dest.strip_prefix(target).unwrap_or(&dest).to_path_buf(),
            dry_run: true,
        });
        report.moved += 1;
    } else {
        let opts = MoveOptions {
            source: path,
            target,
            classification,
            semester,
            filename,
        };

        match move_file(&opts) {
            Ok(dest) => {
                report.moves.push(MoveRecord {
                    dest_relative: dest.strip_prefix(target).unwrap_or(&dest).to_path_buf(),
                    dry_run: false,
                });
                report.moved += 1;
            }
            Err(SortcrabError::Skipped(reason)) => {
                log::debug!("Skipped {}: {reason}", path.display());
                report.skipped += 1;
            }
            Err(e) => {
                log::error!("Failed to move {}: {e}", path.display());
                report.errors += 1;
            }
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SemesterConfig;
    use std::fs;
    use std::path::PathBuf;
    use std::time::SystemTime;
    use tempfile::tempdir;

    fn setup_source_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }

    fn default_semester() -> SemesterConfig {
        SemesterConfig::default()
    }

    fn disabled_semester() -> SemesterConfig {
        SemesterConfig {
            enabled: false,
            ..SemesterConfig::default()
        }
    }

    fn current_semester() -> String {
        semester_label(&SystemTime::now(), 6, "{year}-{roman}")
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
        let report =
            sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

        assert_eq!(report.total, 3);
        assert_eq!(report.moved, 3);
        assert_eq!(report.skipped, 0);
        assert_eq!(report.errors, 0);

        assert!(!src.path().join("report.pdf").exists());
        assert!(!src.path().join("song.mp3").exists());
        assert!(!src.path().join("main.rs").exists());

        assert!(has_file_under(
            &tgt.path().join("Documents/PDF"),
            "report.pdf"
        ));
        assert!(has_file_under(&tgt.path().join("Media/Audio"), "song.mp3"));
        assert!(has_file_under(
            &tgt.path().join("Development/Rust"),
            "main.rs"
        ));
    }

    #[test]
    fn test_basic_sort_exact_path() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), "report.pdf", b"pdf content");

        let rules = RulesConfig::default();
        let _report =
            sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

        let sem = current_semester();
        let expected = tgt.path().join(format!("Documents/PDF/{sem}/report.pdf"));
        assert!(expected.exists(), "Expected file at {}", expected.display());
    }

    #[test]
    fn test_sort_no_semester() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), "report.pdf", b"pdf content");

        let rules = RulesConfig::default();
        let report =
            sort_files(src.path(), tgt.path(), &rules, false, &disabled_semester()).unwrap();

        assert_eq!(report.total, 1);
        assert_eq!(report.moved, 1);

        // File should be directly under Documents/PDF without semester dir
        let expected = tgt.path().join("Documents/PDF/report.pdf");
        assert!(
            expected.exists(),
            "Expected file at {} (no semester dir)",
            expected.display()
        );
    }

    #[test]
    fn test_empty_directory() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        let rules = RulesConfig::default();
        let report =
            sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

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
        let report =
            sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

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
        let report =
            sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

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
        std::os::unix::fs::symlink(src.path().join("real.pdf"), src.path().join("link.pdf"))
            .unwrap();

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
        let report =
            sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

        // On unix the symlink is present so both entries are counted;
        // on non-unix only real.pdf exists.
        #[cfg(unix)]
        {
            assert_eq!(report.total, 2, "should count both real.pdf and link.pdf");
            assert_eq!(report.moved, 1);
            assert_eq!(report.skipped, 1, "link.pdf symlink should be skipped");
            assert_eq!(report.errors, 0, "no errors expected");
        }
        #[cfg(not(unix))]
        {
            assert_eq!(report.total, 1, "only real.pdf exists on this platform");
            assert_eq!(report.moved, 1);
            assert_eq!(report.skipped, 0);
            assert_eq!(report.errors, 0);
        }

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
        let result = sort_files(&file, src.path(), &rules, false, &default_semester());

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
        let report =
            sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

        assert_eq!(report.total, 4);
        assert_eq!(report.moved, 2); // doc.pdf, song.mp3
        assert_eq!(report.skipped, 1); // .hidden.txt
        assert_eq!(report.errors, 1); // unknown.xyz

        // Source directory should still exist (we never remove it).
        assert!(src.path().join("subdir").is_dir());
    }

    // ── Dry run ─────────────────────────────────────────────────────

    #[test]
    fn test_dry_run_basic() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), "doc.pdf", b"pdf");
        setup_source_file(src.path(), "song.mp3", b"mp3");

        let rules = RulesConfig::default();
        let report = sort_files(src.path(), tgt.path(), &rules, true, &default_semester()).unwrap();

        // All files counted, sources must still exist
        assert_eq!(report.total, 2);
        assert_eq!(report.moved, 2);
        assert_eq!(report.skipped, 0);
        assert_eq!(report.errors, 0);

        assert!(src.path().join("doc.pdf").exists());
        assert!(src.path().join("song.mp3").exists());
    }

    #[test]
    fn test_dry_run_respects_dotfiles() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), ".hidden.txt", b"secret");
        setup_source_file(src.path(), "visible.pdf", b"visible");

        let rules = RulesConfig::default();
        let report = sort_files(src.path(), tgt.path(), &rules, true, &default_semester()).unwrap();

        // Dotfile is skipped even in dry-run mode
        assert_eq!(report.total, 2);
        assert_eq!(report.moved, 1);
        assert_eq!(report.skipped, 1);
        assert_eq!(report.errors, 0);

        // Both files still exist
        assert!(src.path().join(".hidden.txt").exists());
        assert!(src.path().join("visible.pdf").exists());
    }

    #[test]
    fn test_dry_no_semester() {
        let src = tempdir().unwrap();
        let tgt = tempdir().unwrap();

        setup_source_file(src.path(), "doc.pdf", b"pdf");

        let rules = RulesConfig::default();
        let report =
            sort_files(src.path(), tgt.path(), &rules, true, &disabled_semester()).unwrap();

        assert_eq!(report.total, 1);
        assert_eq!(report.moved, 1);
        // No semester dir in dry-run output
        assert!(src.path().join("doc.pdf").exists());
    }

    #[test]
    fn test_already_organised_skipped() {
        let tgt = tempdir().unwrap();
        let sem = current_semester();

        // Source is inside the destination tree — file is already organised.
        let src = tgt.path().join("Documents/PDF").join(&sem);
        fs::create_dir_all(&src).unwrap();
        let file_path = src.join("report.pdf");
        fs::write(&file_path, b"content").unwrap();

        let rules = RulesConfig::default();
        let report = sort_files(&src, tgt.path(), &rules, false, &default_semester()).unwrap();

        assert_eq!(report.total, 1);
        assert_eq!(report.skipped, 1);
        assert_eq!(report.moved, 0);
    }
}
