#!/usr/bin/env bash
# Build script for creating distribution packages
# Supports Debian, RPM, and Arch Linux packages

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/build"
PACKAGE_TYPE="${1:-all}"

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

detect_system() {
    if [[ -f /etc/debian_version ]]; then
        echo "debian"
    elif [[ -f /etc/redhat-release ]] || [[ -f /etc/fedora-release ]]; then
        echo "rpm"
    elif [[ -f /etc/arch-release ]]; then
        echo "arch"
    else
        echo "unknown"
    fi
}

check_dependencies() {
    local system_type="$1"
    
    case "$system_type" in
        "debian")
            local deps=("debuild" "dh-cargo" "cargo" "rustc")
            for dep in "${deps[@]}"; do
                if ! command -v "$dep" >/dev/null 2>&1 && ! dpkg -l | grep -q "$dep"; then
                    log_error "Missing dependency: $dep"
                    echo "Install with: sudo apt-get install devscripts dh-cargo cargo rustc"
                    return 1
                fi
            done
            ;;
        "rpm")
            local deps=("rpmbuild" "cargo" "rustc")
            for dep in "${deps[@]}"; do
                if ! command -v "$dep" >/dev/null 2>&1; then
                    log_error "Missing dependency: $dep"
                    echo "Install with: sudo dnf install rpm-build cargo rust"
                    return 1
                fi
            done
            ;;
        "arch")
            local deps=("makepkg" "cargo" "rustc")
            for dep in "${deps[@]}"; do
                if ! command -v "$dep" >/dev/null 2>&1; then
                    log_error "Missing dependency: $dep"
                    echo "Install with: sudo pacman -S base-devel cargo rust"
                    return 1
                fi
            done
            ;;
    esac
    
    log_success "All dependencies satisfied for $system_type"
}

prepare_source() {
    log_info "Preparing source package..."
    
    cd "$PROJECT_ROOT"
    
    # Clean previous builds
    rm -rf "$BUILD_DIR"
    mkdir -p "$BUILD_DIR"
    
    # Create source tarball
    local version
    version=$(grep '^version = ' Cargo.toml | sed 's/.*= "\(.*\)"/\1/')
    
    local archive_name="pointless-${version}"
    local archive_path="$BUILD_DIR/${archive_name}.tar.gz"
    
    # Create clean source archive
    git archive --format=tar.gz --prefix="${archive_name}/" HEAD > "$archive_path"
    
    log_success "Source archive created: $archive_path"
    echo "$archive_path"
}

build_debian_package() {
    log_info "Building Debian package..."
    
    local archive_path="$1"
    local work_dir="$BUILD_DIR/debian-build"
    
    mkdir -p "$work_dir"
    cd "$work_dir"
    
    # Extract source
    tar -xzf "$archive_path"
    local source_dir
    source_dir=$(tar -tzf "$archive_path" | head -1 | cut -f1 -d/)
    cd "$source_dir"
    
    # Copy debian packaging files
    cp -r "$SCRIPT_DIR/debian" .
    
    # Make rules executable
    chmod +x debian/rules
    
    # Build package
    if command -v debuild >/dev/null 2>&1; then
        debuild -us -uc -b
    else
        dpkg-buildpackage -us -uc -b
    fi
    
    # Copy packages to build directory
    cp ../*.deb "$BUILD_DIR/"
    
    log_success "Debian packages built successfully"
}

build_rpm_package() {
    log_info "Building RPM package..."
    
    local archive_path="$1"
    local rpm_build_dir="$BUILD_DIR/rpm-build"
    
    # Set up RPM build environment
    mkdir -p "$rpm_build_dir"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
    
    # Copy source and spec file
    cp "$archive_path" "$rpm_build_dir/SOURCES/"
    cp "$SCRIPT_DIR/rpm/pointless.spec" "$rpm_build_dir/SPECS/"
    
    # Build RPM
    cd "$rpm_build_dir"
    rpmbuild --define "_topdir $(pwd)" -bb SPECS/pointless.spec
    
    # Copy packages to build directory
    cp RPMS/*/*.rpm "$BUILD_DIR/"
    
    log_success "RPM packages built successfully"
}

build_arch_package() {
    log_info "Building Arch Linux package..."
    
    local archive_path="$1"
    local work_dir="$BUILD_DIR/arch-build"
    
    mkdir -p "$work_dir"
    cd "$work_dir"
    
    # Copy PKGBUILD
    cp "$SCRIPT_DIR/arch/PKGBUILD" .
    
    # Copy source archive
    cp "$archive_path" .
    
    # Update checksum in PKGBUILD
    local checksum
    checksum=$(sha256sum "$(basename "$archive_path")" | cut -d' ' -f1)
    sed -i "s/sha256sums=('SKIP')/sha256sums=('$checksum')/" PKGBUILD
    
    # Build package
    makepkg -s --noconfirm
    
    # Copy packages to build directory
    cp *.pkg.tar.* "$BUILD_DIR/"
    
    log_success "Arch Linux packages built successfully"
}

create_install_scripts() {
    log_info "Creating installation scripts for packages..."
    
    cd "$BUILD_DIR"
    
    # Create universal installer that detects system and installs appropriate package
    cat > install-package.sh << 'EOF'
#!/bin/bash
# Universal package installer for pointless

set -e

detect_system() {
    if command -v apt-get >/dev/null 2>&1; then
        echo "debian"
    elif command -v dnf >/dev/null 2>&1; then
        echo "fedora"
    elif command -v yum >/dev/null 2>&1; then
        echo "rhel"
    elif command -v pacman >/dev/null 2>&1; then
        echo "arch"
    elif command -v zypper >/dev/null 2>&1; then
        echo "opensuse"
    else
        echo "unknown"
    fi
}

install_package() {
    local system_type="$1"
    
    case "$system_type" in
        "debian")
            if ls *.deb >/dev/null 2>&1; then
                echo "Installing Debian package..."
                sudo dpkg -i pointless*.deb
                sudo apt-get install -f  # Fix dependencies if needed
            else
                echo "No Debian packages found"
                exit 1
            fi
            ;;
        "fedora"|"rhel")
            if ls *.rpm >/dev/null 2>&1; then
                echo "Installing RPM package..."
                sudo rpm -ivh pointless*.rpm
            else
                echo "No RPM packages found"
                exit 1
            fi
            ;;
        "arch")
            if ls *.pkg.tar.* >/dev/null 2>&1; then
                echo "Installing Arch package..."
                sudo pacman -U pointless*.pkg.tar.*
            else
                echo "No Arch packages found"
                exit 1
            fi
            ;;
        *)
            echo "Unsupported system type: $system_type"
            echo "Please install manually or use the generic install script"
            exit 1
            ;;
    esac
}

system_type=$(detect_system)
echo "Detected system: $system_type"
install_package "$system_type"

echo "pointless installed successfully!"
echo "Run 'pointless --help' to get started"
EOF
    
    chmod +x install-package.sh
    
    log_success "Installation scripts created"
}

main() {
    echo "🏗️  Pointless Package Builder"
    echo "============================"
    echo
    
    local system_type
    system_type=$(detect_system)
    
    log_info "Detected system: $system_type"
    
    # Prepare source archive
    local archive_path
    archive_path=$(prepare_source)
    
    case "$PACKAGE_TYPE" in
        "all")
            log_info "Building packages for all supported distributions..."
            
            # Try building for current system first
            case "$system_type" in
                "debian")
                    if check_dependencies "debian"; then
                        build_debian_package "$archive_path"
                    fi
                    ;;
                "rpm")
                    if check_dependencies "rpm"; then
                        build_rpm_package "$archive_path"
                    fi
                    ;;
                "arch")
                    if check_dependencies "arch"; then
                        build_arch_package "$archive_path"
                    fi
                    ;;
            esac
            ;;
        "debian")
            check_dependencies "debian"
            build_debian_package "$archive_path"
            ;;
        "rpm")
            check_dependencies "rpm"
            build_rpm_package "$archive_path"
            ;;
        "arch")
            check_dependencies "arch"
            build_arch_package "$archive_path"
            ;;
        *)
            log_error "Unknown package type: $PACKAGE_TYPE"
            echo "Supported types: all, debian, rpm, arch"
            exit 1
            ;;
    esac
    
    create_install_scripts
    
    echo
    log_success "Package building completed!"
    echo "Packages created in: $BUILD_DIR"
    ls -la "$BUILD_DIR"
}

# Show help
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    cat << EOF
Usage: $0 [PACKAGE_TYPE]

PACKAGE_TYPE:
  all      Build packages for all supported distributions (default)
  debian   Build Debian/Ubuntu .deb package
  rpm      Build Fedora/RHEL/openSUSE .rpm package  
  arch     Build Arch Linux package

Examples:
  $0           # Build for current system
  $0 all       # Build for all distributions
  $0 debian    # Build only Debian package
  $0 rpm       # Build only RPM package
  $0 arch      # Build only Arch package

The script will detect your system and build the appropriate package type.
All packages will be created in the build/ directory.
EOF
    exit 0
fi

main "$@"