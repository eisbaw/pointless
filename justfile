# AI.rule: Keep this file in sync with the readme.md file
# AI.rule: Keep the number of recipes low in this file.

# Build the application
build:
    cargo build --release

# Build with vision grids feature
build-vision:
    cargo build --release --features vision-grids

# Run the application
run *args:
    cargo run -- {{args}}

# Run unit tests
test-unit:
    cargo test --lib

# Run unit tests with vision grids
test-unit-vision:
    cargo test --lib --features vision-grids

# Run integration tests with Xephyr (basic grids only)
test-integration:
    ./scripts/xephyr_test.sh

# Run integration tests with vision grids
test-integration-vision:
    ENABLE_VISION=true ./scripts/xephyr_test.sh

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
