# Pointless Package Building

This directory contains packaging configuration and build scripts for creating distribution packages of Pointless.

## Supported Package Formats

- **Debian/Ubuntu** (.deb packages)
- **Fedora/RHEL/openSUSE** (.rpm packages) 
- **Arch Linux** (.pkg.tar.* packages)
- **Flatpak** (universal Linux package)

## Quick Start

### Build for Current System
```bash
just package
# or
./packaging/build-packages.sh
```

### Build for All Distributions
```bash
just package-all
# or
./packaging/build-packages.sh all
```

### Build Specific Package Types
```bash
just package-deb      # Debian/Ubuntu
just package-rpm      # Fedora/RHEL/openSUSE
just package-arch     # Arch Linux
just package-flatpak  # Flatpak (universal)
```

## Prerequisites

### General
- Rust toolchain (rustc, cargo)
- X11 development libraries (libx11-dev, libxext-dev, libxi-dev)
- pkg-config

### Distribution-Specific

#### Debian/Ubuntu (.deb)
```bash
sudo apt-get install devscripts dh-cargo cargo rustc
```

#### Fedora/RHEL (.rpm) 
```bash
sudo dnf install rpm-build cargo rust
```

#### Arch Linux (.pkg.tar.*)
```bash
sudo pacman -S base-devel cargo rust
```

#### Flatpak
```bash
# Ubuntu/Debian
sudo apt install flatpak flatpak-builder

# Fedora
sudo dnf install flatpak flatpak-builder

# Arch
sudo pacman -S flatpak flatpak-builder
```

## Package Structure

### Basic Package (`pointless`)
- Main binary: `/usr/bin/pointless`
- Man page: `/usr/share/man/man1/pointless.1`
- Documentation: `/usr/share/doc/pointless/`
- Examples: `/usr/share/doc/pointless/examples/`

### Vision Package (`pointless-vision`)
- Additional OpenCV-based grid types
- Requires OpenCV libraries
- Extended examples and documentation

## Building Process

### 1. Source Preparation
The build process creates a clean source tarball from the git repository:
```bash
git archive --format=tar.gz --prefix=pointless-VERSION/ HEAD > pointless-VERSION.tar.gz
```

### 2. Package Building
Each package format follows its own build process:

- **Debian**: Uses `debuild` or `dpkg-buildpackage`
- **RPM**: Uses `rpmbuild` with spec file
- **Arch**: Uses `makepkg` with PKGBUILD
- **Flatpak**: Uses `flatpak-builder`

### 3. Output Location
All built packages are placed in `build/` directory:
```
build/
├── pointless_0.1.0-1_amd64.deb           # Debian package
├── pointless-0.1.0-1.x86_64.rpm          # RPM package  
├── pointless-0.1.0-1-x86_64.pkg.tar.xz   # Arch package
├── com.example.Pointless.flatpak          # Flatpak bundle
└── install-package.sh                     # Universal installer
```

## Manual Building

### Debian Package
```bash
cd packaging/
./build-packages.sh debian
```

### RPM Package
```bash
cd packaging/
./build-packages.sh rpm
```

### Arch Package
```bash
cd packaging/
./build-packages.sh arch
```

### Flatpak Package
```bash
cd packaging/flatpak/
./build-flatpak.sh
```

## Installation

### Using Universal Installer
```bash
cd build/
./install-package.sh  # Auto-detects system and installs appropriate package
```

### Manual Installation

#### Debian/Ubuntu
```bash
sudo dpkg -i pointless*.deb
sudo apt-get install -f  # Fix dependencies if needed
```

#### Fedora/RHEL
```bash
sudo rpm -ivh pointless*.rpm
```

#### Arch Linux
```bash
sudo pacman -U pointless*.pkg.tar.*
```

#### Flatpak
```bash
flatpak install com.example.Pointless.flatpak
```

## Package Validation

Each package build includes validation steps:
- Unit test execution
- Binary functionality verification
- Dependency checking
- Package integrity validation

## Customization

### Maintainer Information
Update maintainer details in:
- `debian/control`
- `rpm/pointless.spec`  
- `arch/PKGBUILD`

### Dependencies
Modify build and runtime dependencies in respective package files.

### Package Splitting
The packages are split into basic and vision variants:
- Basic package: Core functionality, minimal dependencies
- Vision package: Computer vision features, requires OpenCV

## Troubleshooting

### Common Issues

#### Missing Dependencies
Ensure all build dependencies are installed for your target distribution.

#### Build Failures
Check that:
- Rust toolchain is properly installed
- X11 development libraries are available
- pkg-config can find required libraries

#### Permission Errors
Make sure build scripts are executable:
```bash
chmod +x packaging/build-packages.sh
chmod +x packaging/flatpak/build-flatpak.sh
```

### Debug Mode
Enable verbose output for troubleshooting:
```bash
DH_VERBOSE=1 ./build-packages.sh debian  # Debian
rpmbuild --verbose ...                   # RPM
makepkg --log                           # Arch
```

## Contributing

When adding new packaging formats:
1. Create package configuration files
2. Add build logic to `build-packages.sh`
3. Update this documentation
4. Test on target distribution

## CI/CD Integration

The packaging system integrates with GitHub Actions:
- Automated package building on releases
- Multi-distribution testing
- Package artifact publishing

See `.github/workflows/release.yml` for CI/CD configuration.