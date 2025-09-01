#!/bin/bash
# Uninstall script for pointless

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Standard installation locations
INSTALL_LOCATIONS=(
    "$HOME/.local/bin/pointless"
    "/usr/local/bin/pointless"
    "/usr/bin/pointless"
)

MAN_LOCATIONS=(
    "$HOME/.local/share/man/man1/pointless.1"
    "/usr/local/share/man/man1/pointless.1"
    "/usr/share/man/man1/pointless.1"
)

DOC_LOCATIONS=(
    "$HOME/.local/share/doc/pointless"
    "/usr/local/share/doc/pointless"
    "/usr/share/doc/pointless"
)

remove_files() {
    local files=("$@")
    local removed=false
    
    for file in "${files[@]}"; do
        if [[ -f "$file" || -d "$file" ]]; then
            log_info "Removing $file"
            if [[ "$file" =~ ^/usr ]]; then
                # System location, need sudo
                if command -v sudo >/dev/null 2>&1; then
                    sudo rm -rf "$file"
                else
                    log_warning "Cannot remove $file (no sudo available)"
                    continue
                fi
            else
                rm -rf "$file"
            fi
            removed=true
        fi
    done
    
    if [[ "$removed" == "false" ]]; then
        log_info "No files found to remove in this category"
    fi
}

check_running() {
    if pgrep -f "pointless" >/dev/null 2>&1; then
        log_warning "pointless is currently running"
        echo "Kill running instances? [y/N]"
        read -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            pkill -f "pointless" || true
            sleep 1
            log_info "Stopped running pointless instances"
        fi
    fi
}

main() {
    echo "🗑️  pointless uninstaller"
    echo "========================"
    echo
    
    # Check for running instances
    check_running
    
    # Remove binaries
    echo "Removing binaries..."
    remove_files "${INSTALL_LOCATIONS[@]}"
    
    # Remove man pages
    echo
    echo "Removing man pages..."
    remove_files "${MAN_LOCATIONS[@]}"
    
    # Remove documentation
    echo
    echo "Removing documentation..."
    remove_files "${DOC_LOCATIONS[@]}"
    
    # Update man database
    if command -v mandb >/dev/null 2>&1; then
        log_info "Updating man page database..."
        mandb -q 2>/dev/null || true
    fi
    
    echo
    log_info "pointless uninstalled successfully"
    echo
    echo "Note: This script doesn't remove:"
    echo "  - Configuration in shell rc files (PATH modifications)"
    echo "  - Source code if you cloned the repository"
    echo "  - Build dependencies (Rust, Nix, OpenCV, etc.)"
    echo
    echo "To completely remove, manually check:"
    echo "  - ~/.bashrc or ~/.zshrc for PATH modifications"
    echo "  - Any pointless directories you may have created"
}

# Help
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    cat << EOF
pointless uninstaller

Removes pointless binaries, man pages, and documentation from standard locations.

Usage: $0

This script will look for pointless installations in:
  Binaries: ~/.local/bin, /usr/local/bin, /usr/bin
  Man pages: ~/.local/share/man, /usr/local/share/man, /usr/share/man
  Docs: ~/.local/share/doc, /usr/local/share/doc, /usr/share/doc

System locations (/usr/*) require sudo access.
EOF
    exit 0
fi

main