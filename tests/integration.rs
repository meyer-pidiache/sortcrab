// sortcrab — integration tests

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use sortcrab::config::SemesterConfig;
use sortcrab::config::rules::RulesConfig;
use sortcrab::core::semester::semester_label;
use sortcrab::core::sort_files;
use tempfile::tempdir;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn create_file(dir: &Path, name: &str, content: &[u8]) {
    fs::write(dir.join(name), content).unwrap();
}

fn current_semester() -> String {
    semester_label(&SystemTime::now(), 6, "{year}-{roman}")
}

fn default_semester() -> SemesterConfig {
    SemesterConfig::default()
}

fn sortcrab_binary() -> PathBuf {
    // CARGO_BIN_EXE_<name> is set by Cargo during test compilation and
    // points to the correct binary regardless of --target-dir (works
    // with cargo llvm-cov, cargo nextest, etc.).
    PathBuf::from(env!("CARGO_BIN_EXE_sortcrab"))
}

// ── Full pipeline: multiple file types → correct category/subcategory/semester ──

#[test]
fn test_full_sort_pipeline() {
    let src = tempdir().unwrap();
    let tgt = tempdir().unwrap();

    create_file(src.path(), "report.pdf", b"pdf content");
    create_file(src.path(), "song.mp3", b"mp3 content");
    create_file(src.path(), "main.rs", b"fn main() {}");
    create_file(src.path(), "photo.jpg", b"jpg content");
    create_file(src.path(), "notes.txt", b"notes content");

    let rules = RulesConfig::default();
    let report = sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

    assert_eq!(report.total, 5);
    assert_eq!(report.moved, 5);
    assert_eq!(report.skipped, 0);
    assert_eq!(report.errors, 0);

    let sem = current_semester();

    assert!(
        tgt.path()
            .join(format!("Documents/PDF/{sem}/report.pdf"))
            .exists()
    );
    assert!(
        tgt.path()
            .join(format!("Media/Audio/{sem}/song.mp3"))
            .exists()
    );
    assert!(
        tgt.path()
            .join(format!("Development/Rust/{sem}/main.rs"))
            .exists()
    );
    assert!(
        tgt.path()
            .join(format!("Media/Images/{sem}/photo.jpg"))
            .exists()
    );
    assert!(
        tgt.path()
            .join(format!("Documents/Text/{sem}/notes.txt"))
            .exists()
    );

    // Original files should be gone (moved, not copied)
    assert!(!src.path().join("report.pdf").exists());
    assert!(!src.path().join("song.mp3").exists());
    assert!(!src.path().join("main.rs").exists());
    assert!(!src.path().join("photo.jpg").exists());
    assert!(!src.path().join("notes.txt").exists());
}

// ── Collision resolution ────────────────────────────────────────────────────

#[test]
fn test_sort_with_collisions() {
    let src = tempdir().unwrap();
    let tgt = tempdir().unwrap();
    let rules = RulesConfig::default();
    let sem = current_semester();
    let dest_dir = tgt.path().join(format!("Documents/PDF/{sem}"));

    // First file → report.pdf
    create_file(src.path(), "report.pdf", b"first content");
    let r1 = sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();
    assert_eq!(r1.moved, 1);
    assert!(dest_dir.join("report.pdf").exists());

    // Second file with same name → report-1.pdf
    create_file(src.path(), "report.pdf", b"second content");
    let r2 = sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();
    assert_eq!(r2.moved, 1);
    assert!(dest_dir.join("report-1.pdf").exists());
    assert!(!src.path().join("report.pdf").exists());

    // Third file with same name → report-2.pdf
    create_file(src.path(), "report.pdf", b"third content");
    let r3 = sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();
    assert_eq!(r3.moved, 1);
    assert!(dest_dir.join("report-2.pdf").exists());
    assert!(!src.path().join("report.pdf").exists());
}

// ── Empty directory ─────────────────────────────────────────────────────────

#[test]
fn test_sort_empty_dir() {
    let src = tempdir().unwrap();
    let tgt = tempdir().unwrap();

    let rules = RulesConfig::default();
    let report = sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

    assert_eq!(report.total, 0);
    assert_eq!(report.moved, 0);
    assert_eq!(report.skipped, 0);
    assert_eq!(report.errors, 0);
}

// ── Skip conditions: dotfiles + symlinks + already-organised ─────────────────

#[test]
fn test_sort_all_skip_conditions() {
    let src = tempdir().unwrap();
    let tgt = tempdir().unwrap();

    // Dotfile
    create_file(src.path(), ".hidden.txt", b"secret");
    // Regular file
    create_file(src.path(), "visible.pdf", b"visible content");
    // Symlink to the regular file
    #[cfg(unix)]
    std::os::unix::fs::symlink(src.path().join("visible.pdf"), src.path().join("link.pdf"))
        .unwrap();

    let rules = RulesConfig::default();
    let report = sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

    #[cfg(unix)]
    {
        assert_eq!(report.total, 3, "dotfile + visible.pdf + link.pdf");
        assert_eq!(report.moved, 1);
        assert_eq!(report.skipped, 2, "dotfile + symlink skipped");
    }
    #[cfg(not(unix))]
    {
        assert_eq!(report.total, 2, "no symlink on this platform");
        assert_eq!(report.moved, 1);
        assert_eq!(report.skipped, 1, "only dotfile skipped");
    }
    assert_eq!(report.errors, 0);

    // Dotfile stays untouched
    assert!(src.path().join(".hidden.txt").exists());
    // Symlink stays untouched (use is_symlink, not exists — dangling symlink
    // returns false for exists())
    #[cfg(unix)]
    assert!(src.path().join("link.pdf").is_symlink());
    // Real file was moved
    assert!(!src.path().join("visible.pdf").exists());

    let sem = current_semester();
    assert!(
        tgt.path()
            .join(format!("Documents/PDF/{sem}/visible.pdf"))
            .exists()
    );
}

// ── Mixed known and unknown extensions ───────────────────────────────────────

#[test]
fn test_sort_mixed_known_and_unknown() {
    let src = tempdir().unwrap();
    let tgt = tempdir().unwrap();

    create_file(src.path(), "known.pdf", b"pdf");
    create_file(src.path(), "known.mp3", b"mp3");
    create_file(src.path(), "unknown.xyz", b"???");
    create_file(src.path(), "also_unknown.qwerty", b"???");

    let rules = RulesConfig::default();
    let report = sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

    assert_eq!(report.total, 4);
    assert_eq!(report.moved, 2);
    assert_eq!(report.skipped, 0);
    assert_eq!(report.errors, 2);
}

// ── Nested directories inside source are skipped ─────────────────────────────

#[test]
fn test_sort_nested_directories() {
    let src = tempdir().unwrap();
    let tgt = tempdir().unwrap();

    // Subdirectory with a file inside
    fs::create_dir(src.path().join("subdir")).unwrap();
    create_file(
        src.path().join("subdir").as_ref(),
        "inner.pdf",
        b"inner content",
    );
    // File at root level
    create_file(src.path(), "root.pdf", b"root content");

    let rules = RulesConfig::default();
    let report = sort_files(src.path(), tgt.path(), &rules, false, &default_semester()).unwrap();

    // Only the root-level file is processed
    assert_eq!(report.total, 1);
    assert_eq!(report.moved, 1);
    assert_eq!(report.skipped, 0);
    assert_eq!(report.errors, 0);

    // Subdirectory and its contents remain in source
    assert!(src.path().join("subdir").is_dir());
    assert!(src.path().join("subdir/inner.pdf").exists());

    let sem = current_semester();
    assert!(
        tgt.path()
            .join(format!("Documents/PDF/{sem}/root.pdf"))
            .exists()
    );
}

// ── Init command via CLI subprocess with isolated HOME ─────────────────────
//
// We spawn `sortcrab init` in a subprocess with a temp HOME so the real
// config directory is never touched.  This avoids unsafe env-var
// manipulation (edition 2024) and parallel-test flakiness.

#[test]
fn test_init_command() {
    let tmp_home = tempdir().unwrap();

    let binary = sortcrab_binary();
    let output = std::process::Command::new(&binary)
        .arg("init")
        .env("HOME", tmp_home.path())
        .env_remove("XDG_CONFIG_HOME")
        .output()
        .expect("failed to run sortcrab init");

    assert!(
        output.status.success(),
        "sortcrab init should exit 0; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Parse the config path from init's output (last non‑empty line).
    // tracing info lines may also appear on stdout, so we can't just
    // strip_prefix from the head.
    let stdout = String::from_utf8_lossy(&output.stdout);
    let config_path = stdout
        .lines()
        .rfind(|l| !l.is_empty())
        .and_then(|line| {
            line.trim()
                .strip_prefix("Created default configuration at ")
                .and_then(|s| {
                    let trimmed = s.trim().trim_matches('"');
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(trimmed))
                    }
                })
        })
        .expect("init should print 'Created default configuration at ...'");

    assert!(
        config_path.exists(),
        "config file should exist after init at {config_path:?}"
    );

    // Verify the config is valid TOML with rules
    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(
        content.contains("version = \"1\""),
        "config should contain version"
    );
    assert!(
        content.contains("category = \"Documents\""),
        "config should contain a rule with category"
    );
    assert!(
        content.contains("subcategory = \"PDF\""),
        "config should contain a rule with subcategory"
    );
}

// ── CLI --version ────────────────────────────────────────────────────────────

#[test]
fn test_cli_version() {
    let binary = sortcrab_binary();
    let output = std::process::Command::new(&binary)
        .arg("--version")
        .output()
        .expect("failed to run sortcrab --version");

    assert!(output.status.success(), "sortcrab --version should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("sortcrab"),
        "version output should contain 'sortcrab'"
    );
}

// ── CLI invalid subcommand ───────────────────────────────────────────────────

#[test]
fn test_cli_invalid_subcommand() {
    let binary = sortcrab_binary();
    let output = std::process::Command::new(&binary)
        .arg("nonexistent-subcommand")
        .output()
        .expect("failed to run sortcrab with invalid subcommand");

    assert!(
        !output.status.success(),
        "invalid subcommand should exit with error"
    );
}
