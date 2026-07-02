# sortcrab

Organize your files into categorized, semester-dated folders.

sortcrab scans a source directory, classifies each file by its extension, and moves
it into a structured destination tree: `{category}/{subcategory}/{semester}/{filename}`.

## What it looks like

Before — a chaotic downloads folder:

```text
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

After — organized by category, subcategory, and semester:

```text
~/Downloads
├── Documents/
│   ├── PDF/
│   │   ├── 2025-II/
│   │   │   └── report-final.pdf
│   │   └── 2026-I/
│   │       └── homework_math.pdf
│   ├── Word/
│   │   └── 2026-I/
│   │       └── class_notes.docx
│   └── Spreadsheets/
│       └── 2025-II/
│           └── budget.xlsx
├── Media/
│   ├── Images/
│   │   ├── 2025-II/
│   │   │   ├── DSC_001.jpg
│   │   │   └── DSC_002.jpg
│   │   └── 2026-I/
│   │       └── screenshot.png
│   ├── Audio/
│   │   └── 2026-I/
│   │       └── song.mp3
│   └── Videos/
│       └── 2026-I/
│           └── vacation.mp4
├── Archives/
│   └── Archives/
│       └── 2026-II/
│           ├── archive.zip
│           └── node_modules.zip
└── Development/
    └── Rust/
        └── 2025-II/
            └── main.rs
```

Pass `--no-semester` to sort without semester subdirectories:

```text
~/Downloads
├── Documents/
│   ├── PDF/report-final.pdf
│   └── Word/class_notes.docx
├── Media/
│   ├── Images/DSC_001.jpg
│   ├── Audio/song.mp3
│   └── Videos/vacation.mp4
├── Archives/archive.zip
└── Development/Rust/main.rs
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

**Option 3 — Direct download** (auto-detects OS and architecture)

```bash
curl --proto '=https' --tlsv1.2 -fsSL https://github.com/meyer-pidiache/sortcrab/releases/latest/download/install.sh | sh
```

The script auto-detects your OS, architecture, and libc, downloads the correct binary
from the latest GitHub release, verifies the SHA-256 checksum, and installs it to
`~/.local/bin/`. No `sudo` required.


Flags:

| Flag | Description |
|------|-------------|
| `--version <ver>` | Install a specific version (e.g. `--version 1.2.0`) |
| `--dry-run` | Preview what would happen without writing files |
| `--no-modify-path` | Skip adding `~/.local/bin/` to `PATH` in shell config |

> On Windows, use **Git Bash** or **WSL** — the script supports MINGW64/MSYS2
> environments. PowerShell users see the [install.ps1](https://github.com/meyer-pidiache/sortcrab/releases/latest/download/install.ps1) option.

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

# Zsh (macOS — Homebrew path)
sortcrab completions zsh > /usr/local/share/zsh/site-functions/_sortcrab
# Zsh (Linux — system path)
# sortcrab completions zsh > /usr/share/zsh/site-functions/_sortcrab

# Fish
sortcrab completions fish > ~/.config/fish/completions/sortcrab.fish

# PowerShell
sortcrab completions powershell >> $PROFILE
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

- `--dry-run` — preview which files would be moved without actually moving them
- `--no-semester` — disable semester-based subdirectory grouping
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
to create it with all built-in rules, then edit to customize.

```toml
version = "1"

[semester]
enabled = true
months_per_period = 6
folder_format = "{year}-{roman}"

[rules]
# ~80 built-in extension rules (truncated)
pdf = { category = "Documents", subcategory = "PDF" }
mp3 = { category = "Media", subcategory = "Audio" }
rs = { category = "Development", subcategory = "Rust" }
# ...
```

Rule overrides take precedence over built-in defaults on a per-extension basis —
you can edit any value or add new extensions directly in the file. Extensions not
listed in the file still get their built-in classification thanks to the
merge-at-load behaviour.

## License

sortcrab is distributed under the PolyForm Noncommercial License 1.0.0. See
[LICENSE.md](LICENSE.md) for the full text.

In short: you may use, modify, and distribute this software for noncommercial
purposes. Commercial use requires explicit permission from the author.

Copyright Meyer Pidiache (https://github.com/meyer-pidiache)
