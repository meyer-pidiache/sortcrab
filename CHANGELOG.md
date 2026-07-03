# Changelog

All notable changes to this project will be documented in this file.

### Bug Fixes

- exclude assets, scripts, and agent config from crate package
- address PR review inline comments
- make test_recursive_skip_already_organised order-independent

### Documentation

- use absolute URL for demo GIF to fix crates.io rendering

### Features

- *(ci)* build static musl binaries for Linux releases
- *(cli)* add --recursive / -r flag to CLI args
- *(core)* implement recursive directory traversal for sort_files
- *(cli)* add shell completions subcommand
- *(config)* add missing extension rules

### Refactor

- *(skills)* replace generic rust skill with project-specific skills
- *(lib)* change run() return type from Box<dyn Error> to SortcrabError

### Testing

- update integration tests for recursive param in sort_files
- *(cli)* add integration tests for shell completions subcommand

### Bug Fixes

- *(release)* publish Homebrew formula to tap on every release
- *(cli)* dry-run tracing summary says 'would move' instead of 'moved'
- *(core)* track predicted destinations during dry-run collision resolution
- *(cli)* use 'would move' instead of 'would sort' in dry-run summary
- *(ci)* override RUSTFLAGS for SonarQube scan and remove unsupported coverage report
- *(ci)* remove deprecated sonar-project.properties and unreferenced clippy report path
- *(ci)* extract inline python script and derive version from Cargo.toml
- suppress clippy::too_many_arguments lint on execute_move
- remove legacy semester_from_time() that ignores configured months_per_period
- address PR review feedback

### CI/CD

- pass release tag via env to avoid template injection

### Documentation

- improve issue/PR templates per review feedback
- add Related Issues section to PR template
- sync README config example with new inline format
- fix PR template merge strategy description (squash -> rebase)
- replace ASCII demo trees with animated demo GIF

### Features

- *(community)* add GitHub issue and PR templates
- *(output)* replace per-file tracing logs with user-friendly tree display
- *(config)* use inline TOML format for sortcrab init config
- *(ci)* add SonarQube Cloud OSS analysis
- *(ci)* add coverage data to SonarQube Cloud scan
- *(install)* replace Direct download with auto-detecting install script
- *(taxonomy)* replace redundant Archives/Archives and Packages/Packages with meaningful subcategories

### Refactor

- *(output)* remove redundant header, add target path as tree root
- rename mover module to moving
- replace tracing with log + env_logger
- address PR #42 review feedback
- add unit tests to raise new-code coverage (Refs #35)
- reduce cognitive complexity of sort_files from 30 to 15 (Refs #41)
- decompose process_single_file into 5 small helpers
- use shellexpand crate instead of manual tilde expansion
- remove redundant bare touch before touch -t

### Testing

- *(display)* add unit tests for tree rendering logic

### Bug Fixes

- publish releases directly (not draft)
- use safer git reset in hooks verification example
- *(docs)* update doctest for sort_files semester parameter
- *(test)* update from_toml_parses test to match transparent RulesConfig
- *(ci)* add missing Parser import in doctest and update Cargo.lock
- *(code-review)* address CodeRabbit findings
- skip all dotfiles before classification and sync README config example with generated format

### CI/CD

- auto-release on push to main when version changes

### Chores

- remove sortcrab-conventions skill

### Documentation

- add git hooks setup and ignore .atl/
- add PR template and contributing workflow docs
- add installation instructions to README for all platforms
- add before/after example to README showing sortcrab in action
- improve rustdoc API documentation for docs.rs
- address code review comments
- use latest release URLs and remove stale not-published note
- fix before/after examples with correct semester nesting and add CI badge
- update demo example

### Features

- add technical AI agent skills for Rust stack
- publish Homebrew formula to homebrew-sortcrab tap (#17)
- *(config)* load built-in rules from default.toml via include_str!
- *(cli)* add --dry-run flag to preview file moves
- *(config)* add --no-semester flag and SemesterConfig
- *(config)* semester config with months_per_period and folder_format + bugfixes

### Refactor

- *(logging)* remove .with_file(true) from init_logging
- *(cli)* remove SortArgs and handle_sort

### CI/CD

- add git hooks (fmt+clippy, commit-msg, tests) and PR title validation

### Chores

- bump version to 0.1.1
- update Cargo.lock

### Features

- *(release)* trigger via workflow_dispatch, read version from Cargo.toml

### Refactor

- restructure src/ into domain-organized modules

### Style

- fix import ordering in cli/mod.rs, core/mod.rs, proptest.rs

### Bug Fixes

- address Oracle review issues (EXDEV, config test, clippy), add .sisyphus to gitignore
- default --source resolves to ~/Downloads, not home dir
- CI failures - unused var on Windows, flaky symlink test under coverage
- deny.toml schema, test_init_command XDG_CONFIG_HOME, sortcrab_binary CARGO_BIN_EXE
- cross-platform CI — deny MPL-2.0, cfg-gate config_dir/symlink tests
- parse config path from init stdout instead of hardcoding platform-specific path
- cfg-gate test_sort_all_skip_conditions assertions for non-unix
- make test_config_dir_ends_with_sortcrab cross-platform
- single-line matrix JSON in release plan job — GITHUB_OUTPUT no acepta leading whitespace
- add toolchain: stable to dtolnay/rust-toolchain@v1 steps
- add contents:write permission to Release workflow

### CI/CD

- add GitHub Actions workflow with lint, test, coverage, audit, deny jobs
- fix fmt, deny, and flaky symlink-under-coverage test

### Chores

- scaffold sortcrab project structure
- add dependencies, git hooks, release workflow, changelog config
- bump actions to Node 24 versions + add aarch64 cross-compilation setup
- remove unused anyhow dep, add --all-targets to cargo udeps
- update Cargo.lock after removing anyhow
- remove unused dev-dependencies detected by cargo udeps
- add CODEOWNERS with meyer-pidiache as default reviewer

### Documentation

- remove 'once published to crates.io' caveat — publishing now

### Features

- implement rules engine, semester calculator, file mover, and error types
- implement file classifier and CLI argument parsing
- implement sort, init, config commands and config manager
- wire main entry point in lib.rs run()
- change default sort to in-place + fix CI workflow
- release pipeline — crate metadata, CI tags trigger, README install, release.yml rewrite
- *(ci)* professional CI improvements
- *(ci)* professional CI improvements

### Refactor

- *(cli)* remove 'sort' subcommand, make sort the default

### Testing

- add integration tests, property-based tests, and project docs
<!-- generated by git-cliff -->
