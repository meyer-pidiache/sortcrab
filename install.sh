#!/bin/sh
#
# install.sh — sortcrab installer
#
# Auto-detects OS, architecture, and libc, then downloads and installs
# the correct sortcrab binary from GitHub Releases.
#
# Usage:
#   curl -fsSL https://github.com/meyer-pidiache/sortcrab/releases/latest/download/install.sh | sh
#   curl -fsSL ... | sh -s -- --version 1.2.0
#   curl -fsSL ... | sh -s -- --dry-run
#   curl -fsSL ... | sh -s -- --no-modify-path
#
# Options:
#   --version <ver>    Install a specific version (default: latest)
#   --dry-run          Show what would be done without writing files
#   --no-modify-path   Skip adding install directory to PATH in shell config
#   --help             Show this help message and exit
#
# Licensed under PolyForm Noncommercial License 1.0.0.
# See https://github.com/meyer-pidiache/sortcrab/blob/main/LICENSE.md

set -eu

REPO="meyer-pidiache/sortcrab"
PROJECT="sortcrab"
DEFAULT_INSTALL_DIR="${HOME}/.local/bin"

echo_info() { printf '\033[32m%s\033[0m %s\n' "INFO:" "$1"; }
echo_warn() { printf '\033[33m%s\033[0m %s\n' "WARN:" "$1"; }
echo_err() { printf '\033[31m%s\033[0m %s\n' "ERROR:" "$1" >&2; }

die() {
    echo_err "$@"
    exit 1
}

_tmpdir=""
_cleanup() {
    if [ -n "$_tmpdir" ] && [ -d "$_tmpdir" ]; then
        rm -rf "$_tmpdir"
    fi
}
trap _cleanup EXIT INT TERM HUP

_mktmpdir() {
    if command -v mktemp >/dev/null 2>&1; then
        _tmpdir="$(mktemp -d 2>/dev/null)" || _tmpdir="$(mktemp -d -t sortcrab 2>/dev/null)"
    fi
    if [ -z "$_tmpdir" ]; then
        _tmpdir="/tmp/sortcrab-install-$$"
        mkdir -p "$_tmpdir"
    fi
}

show_help() {
    cat <<'HELP_EOF'
sortcrab installer — auto-detects OS/architecture and installs the correct binary.

Usage:
  curl -fsSL https://github.com/meyer-pidiache/sortcrab/releases/latest/download/install.sh | sh
  curl -fsSL ... | sh -s -- --version 1.2.0
  curl -fsSL ... | sh -s -- --dry-run
  curl -fsSL ... | sh -s -- --no-modify-path

Options:
  --version <ver>    Install a specific version (default: latest)
  --dry-run          Show what would be done without writing files
  --no-modify-path   Skip adding install directory to PATH in shell config
  --help             Show this help message and exit
HELP_EOF
}

VERSION=""
DRY_RUN=0
MODIFY_PATH=1

parse_args() {
    while [ $# -gt 0 ]; do
        case "$1" in
            --help|-h)
                show_help
                exit 0
                ;;
            --version)
                if [ $# -lt 2 ]; then
                    die "--version requires an argument (e.g., --version 1.2.0)"
                fi
                VERSION="$2"
                shift 2
                ;;
            --dry-run)
                DRY_RUN=1
                shift
                ;;
            --no-modify-path)
                MODIFY_PATH=0
                shift
                ;;
            *)
                die "Unknown option: $1\nRun with --help to see available options."
                ;;
        esac
    done
}

OS=""
ARCH=""
LIBC="gnu"
TARGET=""
BINARY=""

detect_platform() {
    case "$(uname -s)" in
        Linux)        OS="linux" ;;
        Darwin)       OS="macos" ;;
        MINGW*|MSYS*) OS="windows" ;;
        CYGWIN*)      OS="windows" ;;
        *)            die "Unsupported OS: $(uname -s). sortcrab supports Linux, macOS, and Windows." ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64) ARCH="x86_64" ;;
        aarch64|arm64) ARCH="aarch64" ;;
        *)            die "Unsupported architecture: $(uname -m). sortcrab supports x86_64 and aarch64." ;;
    esac

    if [ "$OS" = "linux" ]; then
        if ldd --version 2>/dev/null | head -1 | grep -qi musl; then
            LIBC="musl"
        fi
    fi

    case "$OS" in
        linux)   TARGET="${ARCH}-unknown-linux-${LIBC}" ;;
        macos)   TARGET="${ARCH}-apple-darwin" ;;
        windows) TARGET="${ARCH}-pc-windows-msvc" ;;
    esac

    BINARY="sortcrab"
    if [ "$OS" = "windows" ]; then
        BINARY="sortcrab.exe"
    fi

    echo_info "Detected: ${OS} / ${ARCH} (${LIBC}) → ${TARGET}"
}

ARCHIVE_URL=""
SHA_URL=""

determine_urls() {
    BASE_URL="https://github.com/${REPO}"
    ARCHIVE_NAME="sortcrab-${TARGET}.tar.gz"
    SHA_FILE="sortcrab-${TARGET}.sha256"

    if [ -z "$VERSION" ] || [ "$VERSION" = "latest" ]; then
        DOWNLOAD_BASE="${BASE_URL}/releases/latest/download"
    else
        VER="${VERSION#v}"
        DOWNLOAD_BASE="${BASE_URL}/releases/download/v${VER}"
    fi

    ARCHIVE_URL="${DOWNLOAD_BASE}/${ARCHIVE_NAME}"
    SHA_URL="${DOWNLOAD_BASE}/${SHA_FILE}"
}

verify_sha256() {
    echo_info "Verifying SHA-256 checksum..."

    sha_file="${_tmpdir}/${SHA_FILE}"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "$sha_file" "$SHA_URL" || die "Failed to download checksum: ${SHA_URL}"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "$sha_file" "$SHA_URL" || die "Failed to download checksum: ${SHA_URL}"
    else
        die "Neither curl nor wget found. Please install curl or wget and try again."
    fi

    stored_hash=""
    expected_hash=""

    if command -v sha256sum >/dev/null 2>&1; then
        stored_hash=$(cut -d' ' -f1 "$sha_file")
        expected_hash=$(sha256sum "${_tmpdir}/${ARCHIVE_NAME}" | cut -d' ' -f1)
    elif command -v shasum >/dev/null 2>&1; then
        stored_hash=$(cut -d' ' -f1 "$sha_file")
        expected_hash=$(shasum -a 256 "${_tmpdir}/${ARCHIVE_NAME}" | cut -d' ' -f1)
    else
        echo_warn "No SHA-256 tool found. Skipping checksum verification."
        return 0
    fi

    if [ "$expected_hash" != "$stored_hash" ]; then
        cat >&2 <<-EOF
			ERROR: SHA-256 mismatch!
			  Expected: ${stored_hash}
			  Got:      ${expected_hash}
			The download may be corrupted or tampered with.
			Please retry or download manually from:
			  https://github.com/${REPO}/releases
			EOF
        exit 1
    fi

    echo_info "Checksum verified"
}

download_archive() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "${_tmpdir}/${ARCHIVE_NAME}" "$ARCHIVE_URL"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "${_tmpdir}/${ARCHIVE_NAME}" "$ARCHIVE_URL"
    else
        die "Neither curl nor wget found. Please install curl or wget and try again."
    fi
}

get_install_dir() {
    if [ "$OS" = "windows" ] && [ -n "${LOCALAPPDATA:-}" ]; then
        printf '%s\\Programs\\sortcrab' "$LOCALAPPDATA"
    else
        printf '%s' "$DEFAULT_INSTALL_DIR"
    fi
}

install_binary() {
    install_dir=$(get_install_dir)

    if [ "$DRY_RUN" = 1 ]; then
        echo_info "[DRY-RUN] Would download: ${ARCHIVE_URL}"
        echo_info "[DRY-RUN] Would verify SHA-256 checksum"
        echo_info "[DRY-RUN] Would install ${BINARY} to: ${install_dir}/"
        if [ "$MODIFY_PATH" = 1 ]; then
            echo_info "[DRY-RUN] Would add ${install_dir} to PATH in shell config"
        fi
        return 0
    fi

    _mktmpdir

    echo_info "Downloading ${ARCHIVE_NAME}..."
    download_archive

    verify_sha256

    echo_info "Extracting archive..."
    tar xf "${_tmpdir}/${ARCHIVE_NAME}" -C "$_tmpdir" || die "Failed to extract archive (corrupted download?)."

    mkdir -p "$install_dir" || die "Failed to create install directory: ${install_dir}"

    echo_info "Installing ${BINARY} to ${install_dir}/"
    cp "${_tmpdir}/${BINARY}" "${install_dir}/${BINARY}" || die "Failed to copy binary to ${install_dir}/"
    chmod +x "${install_dir}/${BINARY}" 2>/dev/null || true

    echo_info "${PROJECT} installed successfully!"
}

modify_path() {
    [ "$MODIFY_PATH" = 0 ] && return 0

    install_dir=$(get_install_dir)

    case ":${PATH:-}:" in
        *:"${install_dir}":*)
            echo_info "${install_dir} is already in PATH."
            return 0
            ;;
    esac

    echo_info "Adding ${install_dir} to PATH..."

    _shell="${SHELL:-}"
    if [ -z "$_shell" ]; then
        _shell="$(ps -o comm= -p "$(ps -o ppid= -p "$$")" 2>/dev/null || true)"
    fi

    _rc=""
    _line=""
    case "$_shell" in
        */zsh)
            _rc="${HOME}/.zshrc"
            _line="export PATH=\"\${PATH}:${install_dir}\""
            ;;
        */bash)
            if [ "$OS" = "macos" ]; then
                _rc="${HOME}/.zshrc"
                _line="export PATH=\"\${PATH}:${install_dir}\""
                echo_warn "macOS default shell is zsh. Trying ~/.zshrc instead."
            else
                _rc="${HOME}/.bashrc"
                _line="export PATH=\"\${PATH}:${install_dir}\""
            fi
            ;;
        */fish)
            _rc="${HOME}/.config/fish/config.fish"
            _line="fish_add_path ${install_dir}"
            ;;
        */sh|*/dash)
            _rc="${HOME}/.profile"
            _line="export PATH=\"\${PATH}:${install_dir}\""
            ;;
        *)
            echo_warn "Unrecognized shell: ${_shell}. Please add ${install_dir} to your PATH manually."
            return 0
            ;;
    esac

    if [ ! -f "$_rc" ]; then
        touch "$_rc" 2>/dev/null || {
            echo_warn "Cannot create ${_rc}. Please add ${install_dir} to your PATH manually."
            return 0
        }
    fi

    if grep -qsF "$_line" "$_rc" 2>/dev/null; then
        echo_info "PATH entry already exists in ${_rc}."
        return 0
    fi

    {
        echo ""
        echo "# Added by sortcrab installer on $(date +%Y-%m-%d)"
        echo "$_line"
    } >> "$_rc" || {
        echo_warn "Failed to write to ${_rc}. Please add ${install_dir} to your PATH manually."
        return 0
    }

    echo_info "Updated ${_rc} — restart your shell or run: source ${_rc}"
}

print_next_steps() {
    [ "$DRY_RUN" = 1 ] && return 0

    echo ""
    echo_info "To get started, run: sortcrab --help"
    echo_info "Documentation: https://github.com/${REPO}#readme"
}

# Wrapping everything in main() prevents partial execution if the download
# is truncated mid-transfer — the function definition would be incomplete
# and the call at the bottom would fail with a syntax error.
main() {
    parse_args "$@"
    detect_platform
    determine_urls
    install_binary
    modify_path
    print_next_steps
}

main "$@"
