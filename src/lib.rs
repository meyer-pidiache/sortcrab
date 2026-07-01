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
//! use std::path::Path;
//!
//! let rules = RulesConfig::default();
//! let report = sort_files(
//!     Path::new("/path/to/source"),
//!     Path::new("/path/to/target"),
//!     &rules,
//!     false,
//!     true,
//! )?;
//! println!("Moved {} files", report.moved);
//! # Ok::<_, sortcrab::error::SortcrabError>(())
//! ```

pub mod cli;
pub mod config;
pub mod core;
pub mod error;

use clap::Parser;
use tracing_subscriber::EnvFilter;

/// Initialize logging based on verbosity level.
///
/// - Default: `info` level
/// - `--verbose`: `debug` level
/// - `--quiet`: `error` level
///
/// Uses `try_init()` so calling it twice is harmless (second call is a no-op).
pub fn init_logging(verbose: bool, quiet: bool) {
    let filter = if quiet {
        EnvFilter::new("error")
    } else if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_line_number(true)
        .try_init();
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
