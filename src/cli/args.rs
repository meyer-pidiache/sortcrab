// sortcrab — CLI argument parsing

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Organizes files into categorized, semester-dated folders.
#[derive(Parser, Debug)]
#[command(name = "sortcrab", author, version, about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    /// Source directory to scan for files
    #[arg(short, long, default_value = "~/Downloads")]
    pub source: PathBuf,

    /// Target directory for sorted output (defaults to the source directory for in-place organization)
    #[arg(short, long)]
    pub target: Option<PathBuf>,

    /// Enable verbose logging (debug level)
    #[arg(global = true, short, long)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(global = true, short = 'q', long)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a default configuration file
    Init,
    /// View or edit the configuration
    Config(ConfigArgs),
}

/// Arguments for the sort operation.
#[derive(Parser, Debug)]
pub struct SortArgs {
    /// Source directory to scan for files
    pub source: PathBuf,

    /// Target directory for sorted output (defaults to the source directory for in-place organization)
    pub target: Option<PathBuf>,
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
