//! CLI argument parsing and command dispatch.
//!
//! This module defines the CLI command structure via [`clap`] (in [`args`])
//! and implements the handler functions that run each command.

pub mod args;

use std::path::{Path, PathBuf};

use crate::cli::args::{Cli, Commands, ConfigArgs, SortArgs};
use crate::config::ConfigManager;
use crate::config::rules::RulesConfig;
use crate::core::sort_files;
use crate::error::SortcrabError;

/// Parse CLI arguments and dispatch to the appropriate command handler.
///
/// Called from [`crate::run`] after logging initialisation.
///
/// # Errors
/// Returns a [`SortcrabError`] if the requested command fails.
pub fn run(cli: Cli) -> Result<(), SortcrabError> {
    crate::init_logging(cli.verbose, cli.quiet);

    match cli.command {
        None => {
            // Default: run sort
            let args = SortArgs {
                source: cli.source,
                target: cli.target,
            };
            handle_sort(args)
        }
        Some(Commands::Init) => handle_init(),
        Some(Commands::Config(args)) => handle_config(args),
    }
}

fn handle_sort(args: SortArgs) -> Result<(), SortcrabError> {
    execute_sort(&args)
}

fn handle_init() -> Result<(), SortcrabError> {
    tracing::debug!("Initializing default configuration");
    ConfigManager::create_default()?;
    println!(
        "Created default configuration at {:?}",
        ConfigManager::config_path()?
    );
    Ok(())
}

fn handle_config(args: ConfigArgs) -> Result<(), SortcrabError> {
    tracing::debug!("Config show={}, edit={}", args.show, args.edit);
    if args.show {
        ConfigManager::print()?;
    } else if args.edit {
        ConfigManager::edit()?;
    }
    Ok(())
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

/// Execute the sort command.
///
/// Resolves the target directory (defaulting to the source directory for
/// in-place organisation), loads the default rules configuration, calls
/// [`sort_files`], and prints a human-readable summary.
///
/// # Errors
/// Returns [`SortcrabError::InvalidPath`] if the source directory does not exist.
///
/// # Example
///
/// ```rust,no_run
/// use sortcrab::cli::args::SortArgs;
/// use sortcrab::cli::execute_sort;
/// use std::path::PathBuf;
///
/// let args = SortArgs {
///     source: PathBuf::from("~/Downloads"),
///     target: None,
/// };
/// execute_sort(&args).unwrap();
/// ```
pub fn execute_sort(args: &SortArgs) -> Result<(), SortcrabError> {
    let source = resolve_home(&args.source);
    let target: PathBuf = args.target.clone().unwrap_or_else(|| source.clone());

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
