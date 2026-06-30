//! sortcrab — CLI binary entry point.
//!
//! This binary calls [`sortcrab::run`] and exits with a non-zero status code
//! on error.

fn main() {
    if let Err(e) = sortcrab::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
