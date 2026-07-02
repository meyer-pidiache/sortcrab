# demo.tape
#
# Records the sortcrab demo GIF.
#
# Prerequisites:
#   1. Backup and clear your ~/Downloads first — files MUST be empty
#      or existing files will be mixed with the demo files.
#   2. Run ./scripts/prepare-demo.sh (creates 9 demo files in ~/Downloads).
#   3. sortcrab installed, vhs installed.
#
# Usage:
#     cd <project-root>
#     vhs < scripts/demo.tape
#
# Output: assets/sortcrab-demo.gif

Output assets/sortcrab-demo.gif

Type "ls ~/Downloads"
Sleep 200ms
Enter
Sleep 2s

Ctrl+L
Sleep 300ms

Type "sortcrab | more"
Sleep 300ms
Enter
Sleep 2s

Space
Sleep 2s

