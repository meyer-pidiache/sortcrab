---
description: Expert in Rust development for sortcrab - CLI tooling, clap, serde, thiserror, synchronous filesystem operations
name: rust
---

# sortcrab — Rust Development Guidelines

sortcrab is a **synchronous CLI** written in Rust. No async runtime. No Tokio.
Focus: correct filesystem operations, ergonomic CLI via clap, robust error handling with thiserror, and extensive testing.

## Stack

| Layer | Crate | Notes |
|-------|-------|-------|
| CLI framework | `clap` (derive API) | `#[derive(Parser)]`, `#[derive(Subcommand)]`, `ValueEnum` |
| Serialization | `serde` + `serde_json` + `toml` | Config files (TOML), JSON output |
| Error handling | `thiserror` | Custom error enum `SortError` |
| Logging | `log` + `env_logger` | Synchronous, no tracing |
| Testing | built-in `#[test]` + `assert_cmd` + `pretty_assertions` | 111+ tests, integration + unit |
| Filesystem | `std::fs` + `walkdir` | No tokio::fs, no async |

## Core Architecture

### Entry Point
```rust
// src/main.rs — thin wrapper, delegates to lib.rs
fn main() -> Result<(), SortError> {
    sortcrab::run()
}
```

### Clap CLI Structure
```rust
#[derive(Parser)]
#[command(name = "sortcrab", ...)]
struct Cli {
    #[arg(short, long)]
    source: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Config { ... },
    Completions { shell: Shell },
}
```

### Sort Engine (no async, no Tokio)
- `sortcrab::run()` is synchronous
- Filesystem operations use `std::fs` (rename, copy, create_dir_all)
- Cross-filesystem moves: copy + delete fallback
- Collision resolution: incrementing suffixes (`file-1.pdf`)

### Configuration
- TOML config file at `~/.config/sortcrab/config.toml`
- `RulesConfig` struct with `HashMap<String, Rule>` deserialized from TOML
- Built-in defaults merged with user overrides at load time
- Schema-less merge: user file only needs overrides, built-ins fill the rest

### Module Layout
```
src/
├── main.rs           # Thin entry point → lib::run()
├── lib.rs            # Public API, orchestration
├── cli.rs            # Clap derive structs
├── config.rs         # Config loading, merging, init
├── classifier.rs     # Extension → category mapping
├── semester.rs       # Date-based semester computation
├── organiser.rs      # File move/copy orchestration
├── display.rs        # Terminal output formatting
└── error.rs          # SortError enum (thiserror)
```

## Mandatory Patterns

### Error Handling
- Use `SortError` (thiserror) for all fallible operations
- `?` operator for propagation — no `.unwrap()` or `.expect()` in production paths
- Error messages: lowercase, no trailing punctuation
- Use `.context()` from anyhow-style if adding chain context, otherwise let thiserror derive messages

### CLI Argument Design
- `ValueEnum` for enum arguments (e.g., `Shell`)
- `PathBuf` for filesystem paths
- `Option<T>` for truly optional args
- Global flags (`--verbose`, `--dry-run`) on the top-level `Cli`, not on subcommands
- Short flags: single char where unambiguous (`-s`, `-t`, `-v`, `-q`)

### Filesystem Operations
- Always check `path.exists()` or use `TryExists` before operations
- `create_dir_all` before writing to nested paths
- `std::fs::rename` for same-filesystem moves, fall back to copy + remove for cross-filesystem
- Detect cross-filesystem via device ID comparison or attempt-based fallback
- Collision handling: append `-N` suffix before extension

### Testing
- Unit tests in `#[cfg(test)] mod tests` within each module
- Integration tests in `tests/` using `assert_cmd` + `assert_fs`
- `pretty_assertions` for readable test failures
- Snapshot-style: expected directory structures compared after sort operations
- Property-based: random file extensions, varied directory layouts

### Ownership & Borrowing
- `&str` over `&String`, `&[T]` over `&Vec<T>`
- `Cow<'_, str>` where conditional ownership is needed
- `impl AsRef<Path>` for path-accepting functions

### Serde Conventions
- `#[serde(rename_all = "snake_case")]` for all config structs
- `#[serde(default)]` for optional fields
- `#[serde(deny_unknown_fields)]` on top-level config structs
- `#[serde(skip_serializing_if = "Option::is_none")]` where appropriate

### What NOT to use
- No Tokio, no async/await, no `tokio::test`
- No `tracing` — use `log` + `env_logger`
- No `anyhow` — use `thiserror` + `SortError`
- No `unsafe` blocks
- No `Box<dyn Trait>` where `impl Trait` or enums suffice
