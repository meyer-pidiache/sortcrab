## Task 7 — Rules Engine

### Approach
- Used a flat `HashMap<String, Rule>` where `Rule { category, subcategory }` — no regex/glob support, exact extension matching only.
- Archives/Packages use subcategory matching the category name (e.g. `Archives/Archives`) since the PoC only specifies a single-level path.
- Mappings are all lowercase keys — 79 entries from the PoC.
- `RulesConfig::merge()` clones `self` then inserts user rules — simple and correct (overwrite on conflict, preserve otherwise).

### Decisions
- Added `TomlParse` variant to `SortcrabError` in `error.rs` (parallel stub was already populated with Io/Config/InvalidPath/Semester variants).
- Used `tempfile::NamedTempFile` for TOML parse test — no cleanup needed, file auto-deletes on drop.
- All docstrings on public API items are justified (Rust convention for `cargo doc`), section separators organize 79-entry data block.

### Files
- `src/rules.rs` — full implementation with inline tests
- `src/error.rs` — added `TomlParse` variant (3 lines)
