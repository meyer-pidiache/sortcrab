//! CLI argument parsing and command dispatch.
//!
//! This module defines the CLI command structure via [`clap`] (in [`args`])
//! and implements the handler functions that run each command.

pub mod args;
mod display;

use std::path::{Path, PathBuf};

use crate::cli::args::{Cli, Commands, ConfigArgs};
use crate::config::{ConfigManager, SemesterConfig};
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
        None => execute_sort(&cli),
        Some(Commands::Init) => handle_init(),
        Some(Commands::Config(args)) => handle_config(args),
    }
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
/// in-place organisation), loads the configuration, and calls [`sort_files`].
/// Prints a human-readable summary on completion.
///
/// # Errors
/// Returns [`SortcrabError::InvalidPath`] if the source directory does not exist.
///
/// # Example
///
/// ```rust,no_run
/// use clap::Parser;
/// use sortcrab::cli::args::Cli;
/// use sortcrab::cli::execute_sort;
///
/// let cli = Cli::parse_from(["sortcrab", "--dry-run"]);
/// execute_sort(&cli).unwrap();
/// ```
pub fn execute_sort(cli: &Cli) -> Result<(), SortcrabError> {
    let source = resolve_home(&cli.source);
    let target: PathBuf = cli
        .target
        .as_ref()
        .map(|t| resolve_home(t))
        .unwrap_or_else(|| source.clone());

    let config = ConfigManager::load()?;
    let dry_run = cli.dry_run;

    // Disable semester grouping when the CLI flag overrides the config
    let semester_cfg = if cli.no_semester {
        SemesterConfig {
            enabled: false,
            ..config.semester.clone()
        }
    } else {
        config.semester.clone()
    };

    tracing::debug!(
        "Sort source: {:?}, target: {:?}, semester: {}{}",
        source,
        target,
        if semester_cfg.enabled { "on" } else { "off" },
        if dry_run { " (dry run)" } else { "" }
    );

    let report = sort_files(&source, &target, &config.rules, dry_run, &semester_cfg)?;

    if !cli.quiet {
        let action = if dry_run { "Would sort" } else { "Sorted" };
        println!("\n{action} {} files:", report.moves.len());

        display::print_move_tree(&report.moves);

        if dry_run {
            println!(
                "\nDry run: would sort {} files, skip {}, {} errors",
                report.moved, report.skipped, report.errors
            );
        } else {
            println!(
                "\nSorted {} files, skipped {}, {} errors",
                report.moved, report.skipped, report.errors
            );
        }
    }

    Ok(())
}
