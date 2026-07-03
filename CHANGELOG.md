# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Bug Fixes

- Exclude assets, scripts, and agent config from crate package

- Make test_recursive_skip_already_organised order-independent


### Documentation

- Use absolute URL for demo GIF to fix crates.io rendering


### Features

- *(ci)* Build static musl binaries for Linux releases

- *(cli)* Add --recursive / -r flag to CLI args

- *(core)* Implement recursive directory traversal for sort_files

- *(cli)* Add shell completions subcommand

- *(config)* Add missing extension rules


### Refactor

- *(skills)* Replace generic rust skill with project-specific skills

- *(lib)* Change run() return type from Box<dyn Error> to SortcrabError


### Testing

- Update integration tests for recursive param in sort_files

- *(cli)* Add integration tests for shell completions subcommand


## [0.1.3] - 2026-07-02

### Bug Fixes

- *(release)* Publish Homebrew formula to tap on every release

- *(cli)* Dry-run tracing summary says 'would move' instead of 'moved'

- *(core)* Track predicted destinations during dry-run collision resolution

- *(cli)* Use 'would move' instead of 'would sort' in dry-run summary

- *(ci)* Override RUSTFLAGS for SonarQube scan and remove unsupported coverage report

- *(ci)* Remove deprecated sonar-project.properties and unreferenced clippy report path

- *(ci)* Extract inline python script and derive version from Cargo.toml

- Suppress clippy::too_many_arguments lint on execute_move

- Remove legacy semester_from_time() that ignores configured months_per_period


### CI/CD

- Pass release tag via env to avoid template injection


### Documentation

- Improve issue/PR templates per review feedback

- Add Related Issues section to PR template

- Sync README config example with new inline format

- Fix PR template merge strategy description (squash -> rebase)

- Replace ASCII demo trees with animated demo GIF


### Features

- *(community)* Add GitHub issue and PR templates

- *(output)* Replace per-file tracing logs with user-friendly tree display

- *(config)* Use inline TOML format for sortcrab init config

- *(ci)* Add SonarQube Cloud OSS analysis

- *(ci)* Add coverage data to SonarQube Cloud scan

- *(install)* Replace Direct download with auto-detecting install script

- *(taxonomy)* Replace redundant Archives/Archives and Packages/Packages with meaningful subcategories


### Refactor

- *(output)* Remove redundant header, add target path as tree root

- Rename mover module to moving

- Replace tracing with log + env_logger

- Address PR #42 review feedback

- Add unit tests to raise new-code coverage (Refs #35)

- Reduce cognitive complexity of sort_files from 30 to 15 (Refs #41)

- Decompose process_single_file into 5 small helpers

- Use shellexpand crate instead of manual tilde expansion

- Remove redundant bare touch before touch -t


### Testing

- *(display)* Add unit tests for tree rendering logic


## [0.1.2] - 2026-07-01

### Bug Fixes

- Publish releases directly (not draft)

- Use safer git reset in hooks verification example

- *(docs)* Update doctest for sort_files semester parameter

- *(test)* Update from_toml_parses test to match transparent RulesConfig

- *(ci)* Add missing Parser import in doctest and update Cargo.lock

- *(code-review)* Address CodeRabbit findings

- Skip all dotfiles before classification and sync README config example with generated format


### CI/CD

- Auto-release on push to main when version changes


### Chores

- Remove sortcrab-conventions skill


### Documentation

- Add git hooks setup and ignore .atl/

- Add PR template and contributing workflow docs

- Add installation instructions to README for all platforms

- Add before/after example to README showing sortcrab in action

- Improve rustdoc API documentation for docs.rs

- Address code review comments

- Use latest release URLs and remove stale not-published note

- Fix before/after examples with correct semester nesting and add CI badge

- Update demo example


### Features

- Add technical AI agent skills for Rust stack

- Publish Homebrew formula to homebrew-sortcrab tap (#17)

- *(config)* Load built-in rules from default.toml via include_str!

- *(cli)* Add --dry-run flag to preview file moves

- *(config)* Add --no-semester flag and SemesterConfig

- *(config)* Semester config with months_per_period and folder_format + bugfixes


### Refactor

- *(logging)* Remove .with_file(true) from init_logging

- *(cli)* Remove SortArgs and handle_sort


## [0.1.1] - 2026-06-26

### CI/CD

- Add git hooks (fmt+clippy, commit-msg, tests) and PR title validation


### Chores

- Bump version to 0.1.1


### Features

- *(release)* Trigger via workflow_dispatch, read version from Cargo.toml


### Refactor

- Restructure src/ into domain-organized modules


### Style

- Fix import ordering in cli/mod.rs, core/mod.rs, proptest.rs


## [0.1.0] - 2026-06-26

### Bug Fixes

- Address Oracle review issues (EXDEV, config test, clippy), add .sisyphus to gitignore

- Default --source resolves to ~/Downloads, not home dir

- CI failures - unused var on Windows, flaky symlink test under coverage

- Deny.toml schema, test_init_command XDG_CONFIG_HOME, sortcrab_binary CARGO_BIN_EXE

- Cross-platform CI — deny MPL-2.0, cfg-gate config_dir/symlink tests

- Parse config path from init stdout instead of hardcoding platform-specific path

- Cfg-gate test_sort_all_skip_conditions assertions for non-unix

- Make test_config_dir_ends_with_sortcrab cross-platform

- Single-line matrix JSON in release plan job — GITHUB_OUTPUT no acepta leading whitespace

- Add toolchain: stable to dtolnay/rust-toolchain@v1 steps

- Add contents:write permission to Release workflow


### CI/CD

- Add GitHub Actions workflow with lint, test, coverage, audit, deny jobs

- Fix fmt, deny, and flaky symlink-under-coverage test


### Chores

- Add dependencies, git hooks, release workflow, changelog config

- Bump actions to Node 24 versions + add aarch64 cross-compilation setup

- Add CODEOWNERS with meyer-pidiache as default reviewer


### Documentation

- Remove 'once published to crates.io' caveat — publishing now


### Features

- Implement rules engine, semester calculator, file mover, and error types

- Implement file classifier and CLI argument parsing

- Implement sort, init, config commands and config manager

- Wire main entry point in lib.rs run()

- Change default sort to in-place + fix CI workflow

- Release pipeline — crate metadata, CI tags trigger, README install, release.yml rewrite

- *(ci)* Professional CI improvements

- *(ci)* Professional CI improvements


### Refactor

- *(cli)* Remove 'sort' subcommand, make sort the default


### Testing

- Add integration tests, property-based tests, and project docs


<!-- generated by git-cliff -->
