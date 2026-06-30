# Contributing

Thanks for your interest in sortcrab. Here is how to get started.

## Prerequisites

- **Rust** 1.85 or later (edition 2024). Install via [rustup](https://rustup.rs/).

## Setup

```bash
# Clone your fork
git clone https://github.com/<your-username>/sortcrab.git
cd sortcrab

# Verify it builds
cargo build

# Run the tests
cargo test
```

## Git hooks

This project ships custom git hooks under `.githooks/` that run on every commit
and push:

| Hook | What it does |
|------|-------------|
| `pre-commit` | `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` |
| `commit-msg` | Validates the commit message follows Conventional Commits |
| `pre-push` | `cargo test --all-targets` |

Enable them with:

```bash
git config core.hooksPath .githooks
```

Run the hooks once to verify they work:

```bash
# Trigger pre-commit and commit-msg hooks
git commit --allow-empty -m "test: verify hooks" --no-edit
# Undo the test commit
git reset --hard HEAD~1
```

## Running tests

```bash
# All tests
cargo test

# A specific test
cargo test test_basic_sort

# With output
cargo test -- --nocapture
```

## Linting

sortcrab uses `clippy` with deny-level warnings:

```bash
cargo clippy --all-targets -- -D warnings
```

## Commit conventions

This project uses [Conventional Commits](https://www.conventionalcommits.org/).

```
feat: add dry-run mode for sort command
fix: handle files with no extension gracefully
docs: update README with semester format details
chore: bump clap to 4.5
test: add property tests for collision resolution
```

Prefixes: `feat:`, `fix:`, `docs:`, `test:`, `chore:`, `refactor:`, `style:`.

Keep commits focused on a single change and write the body (if needed) in
imperative mood. For example: "Add dry-run mode" not "Added dry-run mode".

## Pull request process

1. Fork the repository on GitHub.
2. Create a feature branch from `main`.
3. Make your changes and commit them.
4. Run `cargo test` and `cargo clippy --all-targets -- -D warnings` — both must
   pass cleanly.
5. Push your branch and open a PR against `main`.
6. Describe what the PR does and why. Link related issues if applicable.

## Pull request checklist

Before opening a PR, verify:

- [ ] `cargo test` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)
- [ ] Changes are scoped to a single logical unit

A PR template is available at `.github/PULL_REQUEST_TEMPLATE.md` — it will be
pre-filled when you open a pull request on GitHub.

## Issue labels

| Label | Description |
|-------|-------------|
| `bug` | Something isn't working |
| `documentation` | Improvements or additions to documentation |
| `enhancement` | New feature or improvement |
| `type:refactor` | Code change that neither fixes a bug nor adds a feature |

## What to work on

Check the issue tracker for open bugs and feature requests. If you want to work
on something that is not yet tracked, open an issue first to discuss the approach.
