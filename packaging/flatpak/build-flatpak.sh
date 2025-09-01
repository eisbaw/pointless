#!/usr/bin/env bash
# Build Flatpak package for Pointless

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

check_dependencies() {
    local deps=("flatpak" "flatpak-builder")
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" >/dev/null 2>&1; then
            log_error "Missing dependency: $dep"
            echo "Install Flatpak with:"
            echo "  Ubuntu/Debian: sudo apt install flatpak flatpak-builder"
            echo "  Fedora: sudo dnf install flatpak flatpak-builder"
            echo "  Arch: sudo pacman -S flatpak flatpak-builder"
            return 1
        fi
    done
    
    # Check if Flathub repository is added
    if ! flatpak remotes | grep -q flathub; then
        log_warn "Flathub repository not found. Adding it..."
        flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
    fi
    
    log_success "All Flatpak dependencies satisfied"
}

install_runtime() {
    log_info "Installing Flatpak runtime and SDK..."
    
    local runtime="org.freedesktop.Platform//23.08"
    local sdk="org.freedesktop.Sdk//23.08"
    local rust_ext="org.freedesktop.Sdk.Extension.rust-stable//23.08"
    
    # Install runtime components
    flatpak install --user -y flathub "$runtime" "$sdk" "$rust_ext" || {
        log_warn "Runtime installation failed or already installed"
    }
    
    log_success "Runtime components ready"
}

generate_cargo_sources() {
    log_info "Generating Cargo sources for Flatpak..."
    
    cd "$PROJECT_ROOT"
    
    # Check if cargo-lock-to-flatpak is available
    if ! command -v flatpak-cargo-generator.py >/dev/null 2>&1; then
        log_info "Downloading flatpak-cargo-generator..."
        
        # Download the cargo source generator
        curl -o "$SCRIPT_DIR/flatpak-cargo-generator.py" \
            "https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py"
        chmod +x "$SCRIPT_DIR/flatpak-cargo-generator.py"
    fi
    
    # Generate sources
    python3 "$SCRIPT_DIR/flatpak-cargo-generator.py" Cargo.lock -o "$SCRIPT_DIR/generated-sources.json"
    
    log_success "Cargo sources generated"
}

build_flatpak() {
    log_info "Building Flatpak package..."
    
    cd "$SCRIPT_DIR"
    
    local build_dir="build"
    local repo_dir="repo"
    
    # Clean previous builds
    rm -rf "$build_dir" "$repo_dir"
    
    # Build the Flatpak
    flatpak-builder --user --install --force-clean \
        "$build_dir" com.example.Pointless.yml
    
    log_success "Flatpak package built and installed"
}

create_bundle() {
    log_info "Creating Flatpak bundle..."
    
    cd "$SCRIPT_DIR"
    
    # Create a bundle file for distribution
    flatpak build-bundle --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo \
        repo com.example.Pointless.flatpak com.example.Pointless
    
    log_success "Flatpak bundle created: com.example.Pointless.flatpak"
}

test_flatpak() {
    log_info "Testing Flatpak installation..."
    
    # Test if the app can be run
    if flatpak run com.example.Pointless --help >/dev/null 2>&1; then
        log_success "Flatpak package works correctly"
    else
        log_warn "Flatpak package may have issues"
    fi
}

main() {
    echo "📦 Pointless Flatpak Builder"
    echo "============================"
    echo
    
    check_dependencies
    install_runtime
    generate_cargo_sources
    build_flatpak
    create_bundle
    test_flatpak
    
    echo
    log_success "Flatpak build completed!"
    echo "To install the bundle on other systems:"
    echo "  flatpak install com.example.Pointless.flatpak"
    echo
    echo "To run:"
    echo "  flatpak run com.example.Pointless"
}

# Show help
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    cat << EOF
Usage: $0

This script builds a Flatpak package for Pointless.

Requirements:
- flatpak and flatpak-builder installed
- Internet connection for downloading dependencies

The script will:
1. Check and install required Flatpak runtimes
2. Generate Cargo dependency sources
3. Build the Flatpak package
4. Create a distributable bundle file
5. Test the installation

Output: com.example.Pointless.flatpak bundle file
EOF
    exit 0
fi

main "$@"