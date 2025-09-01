#!/bin/bash
# Installation script for pointless - keyboard-driven X11 cursor utility
# Supports multiple installation methods and platforms

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://github.com/user/pointless.git"  # Update with actual repo URL
INSTALL_DIR="$HOME/.local/bin"
INSTALL_TYPE="${1:-auto}"  # auto, nix, cargo, binary
ENABLE_VISION="${2:-false}"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

check_command() {
    if command -v "$1" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

detect_platform() {
    if [[ "$OSTYPE" != "linux-gnu"* ]]; then
        log_error "pointless currently only supports Linux with X11"
    fi

    if [[ -z "$DISPLAY" ]]; then
        log_warning "No X11 DISPLAY detected. Make sure you're running in an X11 session"
    fi

    log_info "Detected Linux with X11 support"
}

check_dependencies() {
    local missing_deps=()

    # Check X11 libraries
    if ! pkg-config --exists x11; then
        missing_deps+=("libx11-dev")
    fi
    if ! pkg-config --exists xext; then
        missing_deps+=("libxext-dev")
    fi
    if ! pkg-config --exists xi; then
        missing_deps+=("libxi-dev")
    fi

    # Check vision grid dependencies if enabled
    if [[ "$ENABLE_VISION" == "true" ]]; then
        if ! pkg-config --exists opencv4; then
            missing_deps+=("libopencv-dev")
        fi
        if ! check_command clang; then
            missing_deps+=("clang")
        fi
    fi

    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_warning "Missing dependencies: ${missing_deps[*]}"
        echo
        echo "Install them with:"
        if check_command apt; then
            echo "  sudo apt install ${missing_deps[*]}"
        elif check_command dnf; then
            echo "  sudo dnf install ${missing_deps[*]}"
        elif check_command pacman; then
            echo "  sudo pacman -S ${missing_deps[*]}"
        else
            echo "  Use your distribution's package manager to install: ${missing_deps[*]}"
        fi
        echo
        read -p "Continue installation anyway? [y/N] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_error "Installation cancelled"
        fi
    fi
}

install_with_nix() {
    if ! check_command nix-shell; then
        log_error "Nix not found. Install Nix or use a different installation method"
    fi

    log_info "Installing pointless using Nix environment..."
    
    # Clone repository if not already present
    if [[ ! -d "pointless" ]]; then
        log_info "Cloning repository..."
        git clone "$REPO_URL" pointless
    fi
    
    cd pointless
    
    # Build with Nix
    if [[ "$ENABLE_VISION" == "true" ]]; then
        log_info "Building with vision grids feature..."
        nix-shell --run 'just build-vision'
    else
        log_info "Building basic version..."
        nix-shell --run 'just build'
    fi
    
    # Install binary
    mkdir -p "$INSTALL_DIR"
    cp target/release/pointless "$INSTALL_DIR/"
    
    # Install man page
    mkdir -p "$HOME/.local/share/man/man1"
    cp man/man1/pointless.1 "$HOME/.local/share/man/man1/"
    
    log_success "pointless installed with Nix to $INSTALL_DIR/pointless"
}

install_with_cargo() {
    if ! check_command cargo; then
        log_error "Cargo not found. Install Rust or use Nix installation"
    fi

    log_info "Installing pointless using Cargo..."
    
    # Clone repository if not already present
    if [[ ! -d "pointless" ]]; then
        log_info "Cloning repository..."
        git clone "$REPO_URL" pointless
    fi
    
    cd pointless
    
    # Install dependencies
    check_dependencies
    
    # Build with Cargo
    if [[ "$ENABLE_VISION" == "true" ]]; then
        log_info "Building with vision grids feature..."
        cargo build --release --features vision-grids
    else
        log_info "Building basic version..."
        cargo build --release
    fi
    
    # Install binary
    mkdir -p "$INSTALL_DIR"
    cp target/release/pointless "$INSTALL_DIR/"
    
    # Install man page
    mkdir -p "$HOME/.local/share/man/man1"
    cp man/man1/pointless.1 "$HOME/.local/share/man/man1/"
    
    log_success "pointless installed with Cargo to $INSTALL_DIR/pointless"
}

install_binary() {
    log_info "Installing pre-built binary..."
    log_error "Binary installation not yet implemented. Use 'nix' or 'cargo' installation"
}

setup_path() {
    # Add install directory to PATH if not already present
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        log_info "Adding $INSTALL_DIR to PATH..."
        
        # Detect shell and add to appropriate config file
        if [[ "$SHELL" == *"bash"* ]]; then
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.bashrc"
            log_info "Added to ~/.bashrc"
        elif [[ "$SHELL" == *"zsh"* ]]; then
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.zshrc"
            log_info "Added to ~/.zshrc"
        else
            log_warning "Unknown shell. Add $INSTALL_DIR to your PATH manually"
        fi
        
        # Update current session
        export PATH="$PATH:$INSTALL_DIR"
    fi

    # Update man page database
    if check_command mandb && [[ -d "$HOME/.local/share/man" ]]; then
        mandb -q "$HOME/.local/share/man" 2>/dev/null || true
    fi
}

verify_installation() {
    if [[ ! -f "$INSTALL_DIR/pointless" ]]; then
        log_error "Installation failed: binary not found at $INSTALL_DIR/pointless"
    fi

    if ! "$INSTALL_DIR/pointless" --help >/dev/null 2>&1; then
        log_error "Installation failed: binary is not executable or missing dependencies"
    fi

    log_success "Installation verified successfully!"
    
    # Show version information (no --version available yet)
    echo "pointless 0.1.0 - keyboard-driven X11 cursor utility"
}

show_usage_info() {
    echo
    echo "🎯 pointless is now installed!"
    echo
    echo "Quick start:"
    echo "  1. Run: pointless"
    echo "  2. Hold Super key (Windows key) to show grid"
    echo "  3. Navigate using grid type (default: homogeneous A1-Z9)"
    echo "  4. Release Super key to position cursor"
    echo
    echo "Examples:"
    echo "  pointless                                    # Default homogeneous grid"
    echo "  pointless --grid-type qwerty                 # QWERTY layout"
    echo "  pointless --grid-type vim                    # Vim-style navigation"
    echo "  pointless --modifier-key alt --color FF0000  # Alt key, red grid"
    echo
    if [[ "$ENABLE_VISION" == "true" ]]; then
        echo "Vision grids (requires OpenCV):"
        echo "  pointless --grid-type hough-lines         # Line intersection detection"
        echo "  pointless --grid-type rectangle-fitting   # UI element detection"
        echo
    fi
    echo "Documentation:"
    echo "  man pointless                               # Full manual"
    echo "  pointless --help                            # Command-line options"
    echo "  $HOME/.local/share/doc/pointless/           # Additional docs (if installed)"
    echo
    echo "Need help? Check the documentation or report issues at:"
    echo "  $REPO_URL/issues"
}

main() {
    echo "🎯 pointless installer"
    echo "======================"
    echo

    # Parse command line arguments
    case "$INSTALL_TYPE" in
        "auto")
            if check_command nix-shell; then
                INSTALL_TYPE="nix"
                log_info "Auto-detected Nix environment"
            elif check_command cargo; then
                INSTALL_TYPE="cargo"
                log_info "Auto-detected Cargo/Rust environment"
            else
                log_error "No suitable installation method found. Install Nix or Rust"
            fi
            ;;
        "nix"|"cargo"|"binary")
            log_info "Using specified installation method: $INSTALL_TYPE"
            ;;
        *)
            log_error "Invalid installation type: $INSTALL_TYPE. Use: auto, nix, cargo, or binary"
            ;;
    esac

    # Perform installation steps
    detect_platform
    
    case "$INSTALL_TYPE" in
        "nix")
            install_with_nix
            ;;
        "cargo")
            install_with_cargo
            ;;
        "binary")
            install_binary
            ;;
    esac

    setup_path
    verify_installation
    show_usage_info
    
    log_success "Installation completed successfully! 🎉"
}

# Help function
show_help() {
    cat << EOF
pointless installer

Usage: $0 [INSTALL_TYPE] [ENABLE_VISION]

INSTALL_TYPE:
  auto     Auto-detect best installation method (default)
  nix      Install using Nix environment (recommended)
  cargo    Install using Cargo/Rust toolchain
  binary   Install pre-built binary (not yet implemented)

ENABLE_VISION:
  false    Install basic version without vision grids (default)
  true     Install with OpenCV vision grids support

Examples:
  $0                    # Auto-detect, basic version
  $0 nix                # Use Nix, basic version
  $0 cargo true         # Use Cargo, with vision grids
  $0 auto false         # Auto-detect, basic version (explicit)

For more information about pointless, visit: $REPO_URL
EOF
}

# Handle command line arguments
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    show_help
    exit 0
fi

# Run main installation
main