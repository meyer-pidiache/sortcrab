// sortcrab — file moving logic

use crate::error::SortcrabError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Represents a file classification result.
pub struct Classification {
    pub category: String,
    pub subcategory: String,
}

/// Options for moving a file to its organized destination.
pub struct MoveOptions<'a> {
    pub source: &'a Path,
    pub target: &'a Path,
    pub classification: &'a Classification,
    pub semester: &'a str,
    pub filename: &'a str,
}

/// Move a file to its organized destination.
///
/// Builds the destination path `{target}/{category}/{subcategory}/{semester}/{filename}`,
/// handles collisions with incrementing suffixes (`file-1.pdf`, `file-2.pdf`, …),
/// and performs cross-filesystem moves (copy + delete when `rename` returns EXDEV).
///
/// # Skip conditions
/// - Dotfiles (names starting with `'.'`)
/// - Symbolic links
/// - Files already at the destination path (idempotency)
///
/// # Errors
/// Returns `SortcrabError::Skipped` for files that are deliberately not moved.
/// Returns `SortcrabError::Io` for filesystem errors.
pub fn move_file(opts: &MoveOptions<'_>) -> Result<PathBuf, SortcrabError> {
    // ── Skip dotfiles ──────────────────────────────────────────────
    if opts.filename.starts_with('.') {
        return Err(SortcrabError::Skipped(format!(
            "dotfile: {}",
            opts.filename
        )));
    }

    // ── Skip symlinks ──────────────────────────────────────────────
    let metadata = fs::symlink_metadata(opts.source)?;
    if metadata.is_symlink() {
        return Err(SortcrabError::Skipped(format!(
            "symlink: {}",
            opts.filename
        )));
    }

    // ── Build destination path ─────────────────────────────────────
    let dest_dir = opts
        .target
        .join(&opts.classification.category)
        .join(&opts.classification.subcategory)
        .join(opts.semester);

    // ── Idempotency check ──────────────────────────────────────────
    // If the destination directory already exists and the source file
    // is already inside it, the file is already organised — skip.
    if let Ok(dest_canonical) = fs::canonicalize(&dest_dir)
        && let Ok(source_canonical) = fs::canonicalize(opts.source)
        && source_canonical.starts_with(&dest_canonical)
    {
        return Ok(source_canonical);
    }

    // ── Create destination directory ───────────────────────────────
    fs::create_dir_all(&dest_dir)?;

    // ── Resolve filename collisions ────────────────────────────────
    let dest = resolve_collision(&dest_dir, opts.filename);

    // ── Perform the move ───────────────────────────────────────────
    // Try a fast rename first.  If the source and destination live on
    // different mount points, rename fails with EXDEV → copy + delete.
    match fs::rename(opts.source, &dest) {
        Ok(()) => {}
        Err(e) if is_crosses_devices(&e) => {
            fs::copy(opts.source, &dest)?;
            fs::remove_file(opts.source)?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(dest)
}

/// Resolve a filename collision by appending an incrementing counter.
///
/// If `filename` already exists under `dest_dir`, tries
/// `file-1.ext`, `file-2.ext`, … until a free name is found.
fn resolve_collision(dest_dir: &Path, filename: &str) -> PathBuf {
    let path = dest_dir.join(filename);
    if !path.exists() {
        return path;
    }

    let p = Path::new(filename);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or(filename);

    let ext = match p.extension().and_then(|s| s.to_str()) {
        Some(e) if !e.is_empty() => format!(".{}", e),
        _ => String::new(),
    };

    let mut counter = 1;
    loop {
        let name = format!("{}-{}{}", stem, counter, ext);
        let candidate = dest_dir.join(&name);
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

/// Check whether an [`io::Error`] signals a cross-filesystem move.
///
/// This happens when `fs::rename()` cannot move a file across mount
/// points.  On Linux the raw OS error is `EXDEV` (18); Rust 1.75+
/// stabilised `io::ErrorKind::CrossesDevices` for portable checks.
fn is_crosses_devices(e: &io::Error) -> bool {
    e.kind() == io::ErrorKind::CrossesDevices || e.raw_os_error() == Some(18) // EXDEV (Linux)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // ── Helper to build a minimal Classification ───────────────────
    fn classify() -> Classification {
        Classification {
            category: "academic".into(),
            subcategory: "math".into(),
        }
    }

    // ── test_basic_move ───────────────────────────────────────────
    #[test]
    fn mover_basic_move() {
        let src_dir = tempdir().unwrap();
        let tgt_dir = tempdir().unwrap();

        // Create a source file
        let source_file = src_dir.path().join("notes.pdf");
        fs::write(&source_file, "hello world").unwrap();

        let class = classify();
        let semester = "2025-I";

        let opts = MoveOptions {
            source: &source_file,
            target: tgt_dir.path(),
            classification: &class,
            semester,
            filename: "notes.pdf",
        };

        let dest = move_file(&opts).unwrap();

        // Destination should exist
        assert!(dest.exists(), "destination file should exist");
        // Source should be gone (rename)
        assert!(
            !source_file.exists(),
            "source file should be gone after move"
        );
        // Dest should be at the expected path
        let expected = tgt_dir.path().join("academic/math/2025-I/notes.pdf");
        assert_eq!(dest, expected);
    }

    // ── test_collision_numbering ───────────────────────────────────
    #[test]
    fn mover_collision_numbering() {
        let src_dir = tempdir().unwrap();
        let tgt_dir = tempdir().unwrap();

        let class = classify();
        let semester = "2025-I";

        // Create three identically-named source files (different content)
        let files = ["report.pdf", "report.pdf", "report.pdf"];
        let dests: Vec<PathBuf> = files
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let src = src_dir.path().join(format!("src-{}-{}", i, name));
                fs::write(&src, format!("content-{}", i)).unwrap();

                let opts = MoveOptions {
                    source: &src,
                    target: tgt_dir.path(),
                    classification: &class,
                    semester,
                    filename: name,
                };
                move_file(&opts).unwrap()
            })
            .collect();

        // Should have three distinct destination paths
        let mut unique: std::collections::HashSet<&PathBuf> = std::collections::HashSet::new();
        for d in &dests {
            unique.insert(d);
        }
        assert_eq!(unique.len(), 3, "all three dests should be unique");

        // Check specific names exist
        let dir = tgt_dir.path().join("academic/math/2025-I");
        assert!(dir.join("report.pdf").exists(), "report.pdf should exist");
        assert!(
            dir.join("report-1.pdf").exists(),
            "report-1.pdf should exist"
        );
        assert!(
            dir.join("report-2.pdf").exists(),
            "report-2.pdf should exist"
        );
    }

    // ── test_skip_dotfiles ────────────────────────────────────────
    #[test]
    fn mover_skip_dotfiles() {
        let src_dir = tempdir().unwrap();
        let tgt_dir = tempdir().unwrap();

        let source_file = src_dir.path().join(".hidden.txt");
        fs::write(&source_file, "secret").unwrap();

        let class = classify();
        let opts = MoveOptions {
            source: &source_file,
            target: tgt_dir.path(),
            classification: &class,
            semester: "2025-I",
            filename: ".hidden.txt",
        };

        let result = move_file(&opts);
        assert!(result.is_err(), "dotfile should be rejected");
        let err = result.unwrap_err();
        match err {
            SortcrabError::Skipped(ref msg) => {
                assert!(
                    msg.contains("dotfile"),
                    "error should mention dotfile: {}",
                    msg
                );
            }
            _ => panic!("expected Skipped error, got: {:?}", err),
        }

        // Source should still exist (was not moved)
        assert!(
            source_file.exists(),
            "dotfile source should remain untouched"
        );
    }

    // ── test_skip_symlink ─────────────────────────────────────────
    #[test]
    fn mover_skip_symlink() {
        let src_dir = tempdir().unwrap();
        let tgt_dir = tempdir().unwrap();

        // Create a real file, then a symlink pointing to it
        let real_file = src_dir.path().join("real.txt");
        fs::write(&real_file, "real content").unwrap();

        let link = src_dir.path().join("link.txt");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&real_file, &link).unwrap();
        #[cfg(not(unix))]
        std::os::windows::fs::symlink_file(&real_file, &link).unwrap();

        let class = classify();
        let opts = MoveOptions {
            source: &link,
            target: tgt_dir.path(),
            classification: &class,
            semester: "2025-I",
            filename: "link.txt",
        };

        let result = move_file(&opts);
        assert!(result.is_err(), "symlink should be rejected");
        let err = result.unwrap_err();
        match err {
            SortcrabError::Skipped(ref msg) => {
                assert!(
                    msg.contains("symlink"),
                    "error should mention symlink: {}",
                    msg
                );
            }
            _ => panic!("expected Skipped error, got: {:?}", err),
        }

        // Symlink should still exist (was not moved)
        assert!(link.exists(), "symlink should remain untouched");
    }

    // ── test_idempotent_skip ──────────────────────────────────────
    #[test]
    fn mover_idempotent_skip() {
        let tgt_dir = tempdir().unwrap();

        // Create a file already at its "organised" destination
        let dest = tgt_dir.path().join("academic/math/2025-I/already.pdf");
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::write(&dest, "already organised").unwrap();

        let class = classify();
        let opts = MoveOptions {
            source: &dest,
            target: tgt_dir.path(),
            classification: &class,
            semester: "2025-I",
            filename: "already.pdf",
        };

        // Should succeed (idempotent — returns the same path)
        let result = move_file(&opts).unwrap();
        assert_eq!(result, fs::canonicalize(&dest).unwrap());
        // File should still exist at the same place
        assert!(dest.exists(), "already-organised file should remain");
    }
}
