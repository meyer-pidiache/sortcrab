#!/usr/bin/env bash
#
# prepare-demo.sh
#
# Creates the 9 demo files in ~/Downloads with modified dates set so that
# running `sortcrab` produces exactly:
#
# ~/Downloads
# ├── Documents/
# │   ├── PDF/
# │   │   ├── 2025-II/
# │   │   │   └── report-final.pdf
# │   │   └── 2026-I/
# │   │       └── homework_math.pdf
# │   ├── Spreadsheets/
# │   │   └── 2025-II/
# │   │       └── budget.xlsx
# │   └── Word/
# │       └── 2026-I/
# │           └── class_notes.docx
# ├── Media/
# │   ├── Audio/
# │   │   └── 2026-I/
# │   │       └── song.mp3
# │   ├── Images/
# │   │   ├── 2025-II/
# │   │   │   ├── DSC_001.jpg
# │   │   │   └── DSC_002.jpg
# │   │   └── 2026-I/
# │   │       └── screenshot.png
# │   └── Videos/
# │       └── 2026-I/
# │           └── vacation.mp4
#
# sortcrab uses the file modification time to compute the semester:
#   Jan–Jun → {year}-I
#   Jul–Dec → {year}-II

set -euo pipefail

DOWNLOADS="$HOME/Downloads"

echo "=== Setting up demo files in $DOWNLOADS ==="

# Clean up any leftover files from previous runs
rm -f "$DOWNLOADS"/report-final.pdf \
      "$DOWNLOADS"/homework_math.pdf \
      "$DOWNLOADS"/class_notes.docx \
      "$DOWNLOADS"/budget.xlsx \
      "$DOWNLOADS"/DSC_001.jpg \
      "$DOWNLOADS"/DSC_002.jpg \
      "$DOWNLOADS"/screenshot.png \
      "$DOWNLOADS"/song.mp3 \
      "$DOWNLOADS"/vacation.mp4

# ── Semester 2025-II (Jul–Dec 2025) ──────────────────────────────

# Documents/PDF/2025-II/report-final.pdf
touch -t 202510151200 "$DOWNLOADS"/report-final.pdf

# Documents/Spreadsheets/2025-II/budget.xlsx
touch -t 202508201200 "$DOWNLOADS"/budget.xlsx

# Media/Images/2025-II/DSC_001.jpg
touch -t 202509101200 "$DOWNLOADS"/DSC_001.jpg

# Media/Images/2025-II/DSC_002.jpg
touch -t 202509111200 "$DOWNLOADS"/DSC_002.jpg

# ── Semester 2026-I (Jan–Jun 2026) ────────────────────────────────

# Documents/PDF/2026-I/homework_math.pdf
touch -t 202603151200 "$DOWNLOADS"/homework_math.pdf

# Documents/Word/2026-I/class_notes.docx
touch -t 202604101200 "$DOWNLOADS"/class_notes.docx

# Media/Images/2026-I/screenshot.png
touch -t 202602201200 "$DOWNLOADS"/screenshot.png

# Media/Audio/2026-I/song.mp3
touch -t 202605011200 "$DOWNLOADS"/song.mp3

# Media/Videos/2026-I/vacation.mp4
touch -t 202606151200 "$DOWNLOADS"/vacation.mp4

echo ""
echo "=== Files created ==="
ls -lh "$DOWNLOADS"/report-final.pdf \
      "$DOWNLOADS"/homework_math.pdf \
      "$DOWNLOADS"/class_notes.docx \
      "$DOWNLOADS"/budget.xlsx \
      "$DOWNLOADS"/DSC_001.jpg \
      "$DOWNLOADS"/DSC_002.jpg \
      "$DOWNLOADS"/screenshot.png \
      "$DOWNLOADS"/song.mp3 \
      "$DOWNLOADS"/vacation.mp4

echo ""
echo "Ready. Now run: sortcrab"
echo ""
