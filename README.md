# sortcrab

Organize your files into categorized, semester-dated folders.

sortcrab scans a source directory, classifies each file by its extension, and moves
it into a structured destination tree: `{category}/{subcategory}/{semester}/{filename}`.

## Quick start

### Install

**Option 1 — Homebrew (macOS / Linux)**
```bash
brew install meyer-pidiache/sortcrab/sortcrab
```

**Option 2 — Pre-built binary**
Download the latest tarball for your platform from
[GitHub Releases](https://github.com/meyer-pidiache/sortcrab/releases/latest),
then extract and place `sortcrab` in your `$PATH`.

**Option 3 — Cargo** (once published to crates.io)
```bash
cargo install sortcrab
```

**Option 4 — Build from source**
```bash
git clone https://github.com/meyer-pidiache/sortcrab.git
cd sortcrab
cargo install --path .
```

### Usage

```bash
# Sort your Downloads folder in-place (default)
sortcrab

# Sort another folder in-place
sortcrab --source ~/Documents

# Sort to a different target directory
sortcrab --source ~/Downloads --target ~/Other

# Initialize default configuration
sortcrab init

# View current configuration
sortcrab config --show
```

## Usage

| Command | Description |
|---------|-------------|
| `sortcrab` | Sort your Downloads folder in-place (default). |
| `sortcrab -s ~/Documents` | Sort another folder in-place. |
| `sortcrab -s ~/Downloads -t ~/Other` | Sort to a dedicated target directory instead of in-place. |
| `sortcrab init` | Create the default configuration file at `~/.config/sortcrab/config.toml`. |
| `sortcrab config --show` | Print the current configuration to stdout. |
| `sortcrab config --edit` | Open the configuration file in `$EDITOR` (falls back to `vi`). |

### Flags

- `--verbose` / `-v` — enable debug-level logging
- `--quiet` / `-q` — suppress all output except errors

## How it works

sortcrab processes files in four steps:

1. **Scan** — read all entries in the source directory (subdirectories are skipped)
2. **Classify** — look up each file's extension in a rules table to determine its
   category and subcategory (e.g. `.pdf` → `Documents / PDF`)
3. **Semester** — compute the semester from the file's modification time.
   January–June becomes `{year}-I`, July–December becomes `{year}-II`
4. **Move** — build the destination path `{target}/{category}/{subcategory}/{semester}/{filename}`
   and perform the move. Collisions are resolved with incrementing suffixes
   (`file-1.pdf`, `file-2.pdf`, ...). Cross-filesystem moves fall back to copy + delete.

The following are **skipped** and left in place:
- Dotfiles (names starting with `.`)
- Symbolic links
- Files already at their organized destination

## Default categories

| Category | Subcategories | Example extensions |
|----------|---------------|-------------------|
| Documents | PDF, Word, Text, LaTeX, Presentations, Spreadsheets, Data | `.pdf`, `.docx`, `.txt`, `.tex`, `.pptx`, `.xlsx`, `.csv` |
| Media | Images, Images/Vectors, Images/Photoshop, Images/Illustrator, Images/InDesign, Audio, Videos | `.jpg`, `.png`, `.svg`, `.psd`, `.mp3`, `.mp4` |
| Archives | Archives | `.zip`, `.rar`, `.7z`, `.tar.gz` |
| Packages | Packages | `.deb`, `.rpm`, `.exe`, `.dmg`, `.AppImage` |
| Development | Scripts, Python, JavaScript, TypeScript, Java, C, C++, C#, Go, Rust, Web, Databases, Docker | `.py`, `.js`, `.ts`, `.rs`, `.go`, `.html`, `.sql` |
| Other | Torrents, DiskImages, VirtualMachines | `.torrent`, `.iso`, `.vdi` |

sortcrab ships with approximately 80 built-in extension-to-category mappings. You can
override or extend any of them in the configuration file.

## Configuration

The configuration file lives at `~/.config/sortcrab/config.toml`. Run `sortcrab init`
to create it with defaults, then edit to customize.

```toml
version = "1"

[rules]
"pdf" = { category = "Documents", subcategory = "PDF" }
"mp3" = { category = "Media", subcategory = "Audio" }
# Add your own overrides below
```

User rules take precedence over built-in defaults on a per-extension basis. Extensions
not listed in your config keep their default mappings.

## Status

- **Build status**: passing (placeholder)
- **Test coverage**: growing (placeholder)
- **License**: [PolyForm Noncommercial 1.0.0](LICENSE.md)

## License

sortcrab is distributed under the PolyForm Noncommercial License 1.0.0. See
[LICENSE.md](LICENSE.md) for the full text.

In short: you may use, modify, and distribute this software for noncommercial
purposes. Commercial use requires explicit permission from the author.

Copyright Meyer Pidiache (https://github.com/meyer-pidiache)
