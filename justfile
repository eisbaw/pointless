# AI.rule: Keep this file in sync with the readme.md file
# AI.rule: Keep the number of recipes low in this file.

# Build the application
build:
    cargo build --release

# Build with vision grids feature (DEPRECATED - vision grids removed)
build-vision:
    @echo "Vision grids feature has been removed from this project"
    @echo "Use 'just build' for the standard build"

# Run the application
run *args:
    cargo run -- {{args}}

# Run unit tests
test-unit:
    cargo test --lib

# Run unit tests with vision grids (DEPRECATED - vision grids removed)
test-unit-vision:
    @echo "Vision grids feature has been removed from this project"
    @echo "Use 'just test-unit' for the standard unit tests"

# Run integration tests with Xephyr (basic grids only)
test-integration:
    ./scripts/xephyr_test.sh

# Run integration tests with vision grids (DEPRECATED - vision grids removed)
test-integration-vision:
    @echo "Vision grids feature has been removed from this project"
    @echo "Use 'just test-integration' for the standard integration tests"

# Run visual validation tests
test-visual:
    ./scripts/visual_test.sh

# Run comprehensive integration tests (all grid types, configurations)
test-integration-full:
    chmod +x ./scripts/xephyr_test.sh && ./scripts/xephyr_test.sh

# Run X11 compatibility tests
test-x11:
    ./scripts/x11_compatibility_test.sh

# Run all tests (unit + basic integration)
test: test-unit test-integration

# Run comprehensive test suite (environment-aware)
test-comprehensive:
    ./scripts/run_tests.sh

# Run all tests including vision grids and X11 compatibility
test-all: test-unit test-integration-full test-x11 test-visual

# Check code formatting
fmt:
    cargo fmt

# Check code with clippy
lint:
    cargo clippy

# Clean build artifacts
clean:
    cargo clean

# Package management
# =================

# Build packages for current distribution
package:
    ./packaging/build-packages.sh

# Build packages for all distributions
package-all:
    ./packaging/build-packages.sh all

# Build Debian package
package-deb:
    ./packaging/build-packages.sh debian

# Build RPM package  
package-rpm:
    ./packaging/build-packages.sh rpm

# Build Arch package
package-arch:
    ./packaging/build-packages.sh arch

# Build Flatpak package
package-flatpak:
    ./packaging/flatpak/build-flatpak.sh
