# sortcrab

Organize your files into categorized, semester-dated folders.

sortcrab scans a source directory, classifies each file by its extension, and moves
it into a structured destination tree: `{category}/{subcategory}/{semester}/{filename}`.

## What it looks like

Before — a chaotic downloads folder:

```
~/Downloads
├── report-final.pdf
├── DSC_001.jpg
├── DSC_002.jpg
├── homework_math.pdf
├── class_notes.docx
├── vacation.mp4
├── screenshot.png
├── song.mp3
├── budget.xlsx
├── archive.zip
├── main.rs
└── node_modules.zip
```

After — organized by category and subcategory:

```
~/Downloads
├── Documents/
│   ├── PDF/
│   │   ├── report-final.pdf
│   │   └── homework_math.pdf
│   ├── Word/
│   │   └── class_notes.docx
│   └── Spreadsheets/
│       └── budget.xlsx
├── Media/
│   ├── Images/
│   │   ├── DSC_001.jpg
│   │   ├── DSC_002.jpg
│   │   └── screenshot.png
│   ├── Audio/
│   │   └── song.mp3
│   └── Videos/
│       └── vacation.mp4
├── Archives/
│   ├── archive.zip
│   └── node_modules.zip
└── Development/
    └── Rust/
        └── main.rs
```

In semester mode, files are additionally grouped by academic semester
based on their modification date:

```
~/Downloads
├── 2025-2/
│   ├── Documents/PDF/report-final.pdf
│   ├── Media/Images/DSC_001.jpg
│   └── Development/Rust/main.rs
├── 2026-1/
│   ├── Documents/Word/class_notes.docx
│   ├── Media/Audio/song.mp3
│   └── Media/Videos/vacation.mp4
└── 2026-2/
    └── Archives/archive.zip
```

## Quick start

### Install

**Option 1 — Homebrew (macOS / Linux)**
```bash
brew install meyer-pidiache/sortcrab/sortcrab
```

**Option 2 — Cargo**
```bash
cargo install sortcrab
```

**Option 3 — Direct download**

Linux (x86_64):
```bash
curl -L https://github.com/meyer-pidiache/sortcrab/releases/download/v0.1.1/sortcrab-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv sortcrab /usr/local/bin/
```

Linux (aarch64):
```bash
curl -L https://github.com/meyer-pidiache/sortcrab/releases/download/v0.1.1/sortcrab-aarch64-unknown-linux-gnu.tar.gz | tar xz
sudo mv sortcrab /usr/local/bin/
```

macOS (Apple Silicon):
```bash
curl -L https://github.com/meyer-pidiache/sortcrab/releases/download/v0.1.1/sortcrab-aarch64-apple-darwin.tar.gz | tar xz
sudo mv sortcrab /usr/local/bin/
```

macOS (Intel):
```bash
curl -L https://github.com/meyer-pidiache/sortcrab/releases/download/v0.1.1/sortcrab-x86_64-apple-darwin.tar.gz | tar xz
sudo mv sortcrab /usr/local/bin/
```

Windows (PowerShell):
```powershell
curl.exe -LO https://github.com/meyer-pidiache/sortcrab/releases/download/v0.1.1/sortcrab-x86_64-pc-windows-msvc.tar.gz
tar -xf sortcrab-x86_64-pc-windows-msvc.tar.gz
move sortcrab.exe C:\Windows\System32\
```

**Option 4 — Build from source**
```bash
git clone https://github.com/meyer-pidiache/sortcrab.git
cd sortcrab
cargo install --path .
```

**Shell completions**

sortcrab can generate shell completion scripts for bash, zsh, fish, and
PowerShell:

```bash
# Bash
sortcrab completions bash > ~/.local/share/bash-completion/completions/sortcrab

# Zsh
sortcrab completions zsh > /usr/local/share/zsh/site-functions/_sortcrab

# Fish
sortcrab completions fish > ~/.config/fish/completions/sortcrab.fish

# PowerShell
sortcrab completions powershell >> $PROFILE
```

> **Note:** sortcrab is not yet published to crates.io or Homebrew core.
> The formulas and packages above refer to the project's GitHub Releases
> and custom Homebrew tap.

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
