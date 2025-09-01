#!/bin/bash
# Quick installation script for pointless
# One-liner install for common scenarios

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Check if we're in the pointless directory
if [[ ! -f "Cargo.toml" ]] || ! grep -q "pointless" Cargo.toml; then
    log_error "Run this script from the pointless project directory"
fi

# Quick dependency check
if ! command -v nix-shell >/dev/null 2>&1; then
    if ! command -v cargo >/dev/null 2>&1; then
        log_error "Need either Nix or Rust/Cargo installed"
    fi
fi

# Choose installation method
if command -v nix-shell >/dev/null 2>&1; then
    log_info "Using Nix for installation..."
    nix-shell --run 'just build'
    BINARY_PATH="target/release/pointless"
else
    log_info "Using Cargo for installation..."
    cargo build --release
    BINARY_PATH="target/release/pointless"
fi

# Install to user directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
cp "$BINARY_PATH" "$INSTALL_DIR/"

# Install man page
mkdir -p "$HOME/.local/share/man/man1"
cp man/man1/pointless.1 "$HOME/.local/share/man/man1/"

# Verify installation
if ! "$INSTALL_DIR/pointless" --help >/dev/null 2>&1; then
    log_error "Installation failed - binary not working"
fi

echo
echo "🎯 pointless installed successfully!"
echo
echo "Quick start:"
echo "  1. Add $INSTALL_DIR to your PATH if needed"
echo "  2. Run: pointless"
echo "  3. Hold Super key, navigate with A1-Z9 grid"
echo "  4. Release Super key to position cursor"
echo
echo "For detailed usage: man pointless"

# Check if in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo
    echo "⚠️  Add to your shell config:"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
fi