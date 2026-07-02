//! sortcrab — file organization library.
//!
//! sortcrab scans a source directory, classifies each file by its extension,
//! computes an academic semester from the file's modification time, and moves
//! it into a structured destination tree:
//! `{target}/{category}/{subcategory}/{semester}/{filename}`.
//!
//! # Example
//!
//! ```rust,no_run
//! use sortcrab::core::sort_files;
//! use sortcrab::config::rules::RulesConfig;
//! use sortcrab::config::SemesterConfig;
//! use std::path::Path;
//!
//! let rules = RulesConfig::default();
//! let semester = SemesterConfig::default();
//! let report = sort_files(
//!     Path::new("/path/to/source"),
//!     Path::new("/path/to/target"),
//!     &rules,
//!     false,
//!     &semester,
//! )?;
//! println!("Moved {} files", report.moved);
//! # Ok::<_, sortcrab::error::SortcrabError>(())
//! ```

pub mod cli;
pub mod config;
pub mod core;
pub mod error;

use clap::Parser;

/// Initialize logging based on verbosity level.
///
/// - Default: `info` level
/// - `--verbose`: `debug` level
/// - `--quiet`: `error` level
///
/// Uses `try_init()` so calling it twice is harmless (second call is a no-op).
pub fn init_logging(verbose: bool, quiet: bool) {
    let mut builder = env_logger::Builder::from_default_env();

    builder.format_target(true).format_line_number(true);

    if quiet {
        builder.filter_level(log::LevelFilter::Error);
    } else if verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }

    let _ = builder.try_init();
}

/// Run the sortcrab CLI.
///
/// Parses command-line arguments via [`clap`] and dispatches to the
/// appropriate command handler. This is the main entry point used by
/// `main.rs`.
///
/// # Errors
/// Returns a boxed error if CLI argument parsing or command execution fails.
///
/// # Example
///
/// ```rust,no_run
/// sortcrab::run().unwrap();
/// ```
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::args::Cli::parse();
    cli::run(cli).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
