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

# Run integration tests with Xephyr
test-integration:
    ./scripts/xephyr_test.sh

# Run visual validation tests
test-visual:
    ./scripts/visual_test.sh

# Run all tests
test: test-unit

# Check code formatting
fmt:
    cargo fmt

# Check code with clippy
lint:
    cargo clippy

# Clean build artifacts
clean:
    cargo clean
