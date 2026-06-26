// sortcrab — CLI entry point

fn main() {
    if let Err(e) = sortcrab::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
