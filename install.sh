#!/bin/bash
set -e

# Lark CLI Installer
# https://github.com/DreamCats/lark-cli

REPO="DreamCats/lark-cli"
BINARY_NAME="lark-cli"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS and architecture
detect_platform() {
    local os arch

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux*)
            case "$arch" in
                x86_64)  echo "x86_64-unknown-linux-gnu" ;;
                aarch64) echo "aarch64-unknown-linux-gnu" ;;
                *)       error "Unsupported architecture: $arch" ;;
            esac
            ;;
        Darwin*)
            case "$arch" in
                x86_64)  echo "x86_64-apple-darwin" ;;
                arm64)   echo "aarch64-apple-darwin" ;;
                *)       error "Unsupported architecture: $arch" ;;
            esac
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac
}

# Get latest release version
get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | \
        grep '"tag_name":' | \
        sed -E 's/.*"([^"]+)".*/\1/'
}

# Download and install
install() {
    local platform version download_url tmp_dir

    info "Detecting platform..."
    platform=$(detect_platform)
    info "Platform: $platform"

    info "Fetching latest version..."
    version=$(get_latest_version)
    if [ -z "$version" ]; then
        error "Failed to get latest version"
    fi
    info "Version: $version"

    download_url="https://github.com/${REPO}/releases/download/${version}/${BINARY_NAME}-${platform}.tar.gz"
    info "Downloading from: $download_url"

    tmp_dir=$(mktemp -d)
    trap "rm -rf $tmp_dir" EXIT

    curl -fsSL "$download_url" -o "$tmp_dir/lark-cli.tar.gz" || error "Download failed"

    info "Extracting..."
    tar -xzf "$tmp_dir/lark-cli.tar.gz" -C "$tmp_dir"

    info "Installing to $INSTALL_DIR..."
    mkdir -p "$INSTALL_DIR"
    mv "$tmp_dir/$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"

    success "Lark CLI $version installed successfully!"

    # Check if INSTALL_DIR is in PATH
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        warn "$INSTALL_DIR is not in your PATH"
        echo ""
        echo "Add the following to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        echo ""
    fi

    echo ""
    info "Run 'lark-cli --help' to get started"
}

# Run installer
install
