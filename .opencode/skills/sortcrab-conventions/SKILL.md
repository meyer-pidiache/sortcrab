---
name: sortcrab-conventions
description: Project-specific conventions for sortcrab, a Rust CLI file organizer. Module structure, crate usage, testing patterns, and commit conventions.
metadata:
  audience: contributors
---

## Project Structure

sortcrab follows a domain-module layout under `src/`:

- `main.rs` — binary entry point, calls `sortcrab::run()`
- `lib.rs` — library root, re-exports modules, logging setup
- `cli/` — CLI argument parsing (`args.rs`) and command dispatch (`mod.rs`)
- `config/` — TOML config loading/saving (`mod.rs`) and rules table (`rules.rs`)
- `core/` — domain logic: classification (`classify.rs`), moving (`mover.rs`), semester calculation (`semester.rs`)
- `error.rs` — unified error type via `thiserror`

## Crate Conventions

- **clap** (derive API) — CLI parsing with `#[derive(Parser, Subcommand)]`
- **thiserror** — `SortcrabError` enum with `#[derive(Error)]` and `#[from]`
- **serde** + **toml** — config serialization with `#[derive(Serialize, Deserialize)]`
- **tracing** + **tracing-subscriber** — structured logging with `EnvFilter`
- **chrono** — timezone-aware date handling for semester computation
- **tempfile** — temporary directories in tests
- **directories** — platform-specific config paths via `ProjectDirs`

## Rust Edition & Idioms

- Edition 2024
- `let_chains` for multi-condition patterns (`if let Ok(meta) = ... && meta.is_symlink()`)
- `impl Default` for config types with sensible defaults
- Associated functions (no `self`) on stateless managers like `ConfigManager`
- `PathBuf` for owned paths, `&Path` for borrowed references
- Match ergonomics: `Err(e) =>` rather than `Err(ref e) =>`

## Testing Conventions

- Tests are inline (`#[cfg(test)] mod tests`) in each source file
- Use `tempfile::tempdir()` for isolated filesystem tests
- Property testing via proptest is preferred for edge-case coverage
- Run with `cargo nextest` for parallel test execution and better failure reporting
- Test function names are snake_case and describe the scenario
- Use helper functions (`setup_source_file`, `classify`, `current_semester`) to reduce boilerplate

## Module Organization

- Each module has a `mod.rs` that re-exports public items
- Domain types are defined close to where they are used
- `SortReport` is defined in `core/mod.rs` alongside `sort_files`
- `MoveOptions` is a builder-like struct passed to `move_file`
- Errors use `thiserror` with `#[error("...")]` for display formatting
