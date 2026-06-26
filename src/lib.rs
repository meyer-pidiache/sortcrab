// sortcrab — library root

pub mod cli;
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
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
