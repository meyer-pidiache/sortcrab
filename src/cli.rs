// sortcrab — CLI argument parsing

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::config::ConfigManager;
use crate::error::SortcrabError;

/// Organizes files into categorized, semester-dated folders.
#[derive(Parser, Debug)]
#[command(name = "sortcrab", author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging (debug level)
    #[arg(global = true, short, long)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(global = true, short = 'q', long)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Sort files from a source directory into categorized, semester-dated folders
    Sort(SortArgs),
    /// Create a default configuration file
    Init,
    /// View or edit the configuration
    Config(ConfigArgs),
}

/// Arguments for the `sort` subcommand.
#[derive(Parser, Debug)]
pub struct SortArgs {
    /// Source directory to scan for files
    #[arg(short, long, default_value = "~/Downloads")]
    pub source: PathBuf,

    /// Target directory for sorted output (defaults to the source directory for in-place organization)
    #[arg(short, long)]
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

/// Parse CLI arguments and dispatch to the appropriate command handler.
///
/// Called from [`crate::run`] after initial setup.
pub fn run(cli: Cli) -> Result<(), SortcrabError> {
    crate::init_logging(cli.verbose, cli.quiet);

    match cli.command {
        Commands::Sort(args) => handle_sort(args),
        Commands::Init => handle_init(),
        Commands::Config(args) => handle_config(args),
    }
}

fn handle_sort(args: SortArgs) -> Result<(), SortcrabError> {
    crate::commands::execute_sort(&args)
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
