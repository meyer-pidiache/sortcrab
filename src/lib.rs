// sortcrab — library root

use clap::Parser;

pub mod cli;
pub mod commands;
pub mod config;
pub mod rules;
pub mod classify;
pub mod semester;
pub mod mover;
pub mod error;

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
        .with_file(true)
        .with_line_number(true)
        .try_init();
}

/// Entry point for the sortcrab CLI.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    cli::run(cli).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

