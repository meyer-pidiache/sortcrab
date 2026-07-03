//! CLI argument parsing using [`clap`] derive API.
//!
//! Defines the top-level [`Cli`] struct, the [`Commands`] enum for
//! subcommands, and supporting argument structs.

use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

/// Organises files into categorised, semester-dated folders.
///
/// sortcrab scans a source directory, classifies each file by its extension,
/// and moves it into a structured tree:
/// `{target}/{category}/{subcategory}/{semester}/{filename}`.
#[derive(Parser, Debug)]
#[command(name = "sortcrab", author, version, about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    /// Source directory to scan for files
    #[arg(short, long, default_value = "~/Downloads")]
    pub source: PathBuf,

    /// Target directory for sorted output (defaults to the source directory for in-place organisation)
    #[arg(short, long)]
    pub target: Option<PathBuf>,

    /// Enable verbose logging (debug level)
    #[arg(global = true, short, long)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(global = true, short = 'q', long)]
    pub quiet: bool,

    /// Perform a dry run without moving any files
    #[arg(long)]
    pub dry_run: bool,

    /// Recursively descend into subdirectories
    #[arg(short = 'r', long)]
    pub recursive: bool,

    /// Disable semester-based subdirectory grouping
    #[arg(long)]
    pub no_semester: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// sortcrab subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a default configuration file at `~/.config/sortcrab/config.toml`
    Init,
    /// View or edit the sortcrab configuration
    Config(ConfigArgs),
    /// Generate shell completion scripts
    Completions(CompletionArgs),
}

/// Arguments for the `config` subcommand.
#[derive(Parser, Debug)]
pub struct ConfigArgs {
    /// Print the current configuration to stdout
    #[arg(long)]
    pub show: bool,

    /// Open the configuration file in the system's default editor
    #[arg(long)]
    pub edit: bool,
}

/// Arguments for the `completions` subcommand.
#[derive(Parser, Debug)]
pub struct CompletionArgs {
    /// Shell to generate completions for (bash, zsh, fish, powershell)
    pub shell: Shell,
}
