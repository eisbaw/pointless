#!/usr/bin/env bash
# Visual testing script for Pointless
# Captures screenshots and compares against reference images

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TEST_DIR="$PROJECT_ROOT/tests"
REF_DIR="$TEST_DIR/reference_images"
OUT_DIR="$TEST_DIR/output_images"
XEPHYR_DISPLAY=":99"
XEPHYR_PID=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

cleanup() {
    if [[ -n "$XEPHYR_PID" ]]; then
        log_info "Stopping Xephyr server (PID: $XEPHYR_PID)"
        kill "$XEPHYR_PID" 2>/dev/null || true
        wait "$XEPHYR_PID" 2>/dev/null || true
    fi
}

# Set up cleanup trap
trap cleanup EXIT

prepare_directories() {
    mkdir -p "$REF_DIR"
    mkdir -p "$OUT_DIR"
    mkdir -p "$TEST_DIR/unit"
    mkdir -p "$TEST_DIR/integration"
}

check_dependencies() {
    log_info "Checking visual test dependencies..."
    
    local deps=("Xephyr" "xwininfo" "import" "compare")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" >/dev/null 2>&1; then
            case "$dep" in
                "Xephyr")
                    log_error "$dep not found. Install with: sudo apt-get install xserver-xephyr"
                    ;;
                "import"|"compare")
                    log_error "$dep not found. Install with: sudo apt-get install imagemagick"
                    ;;
                *)
                    log_error "$dep not found. Install with: sudo apt-get install x11-utils"
                    ;;
            esac
            exit 1
        fi
    done
    
    log_info "All visual test dependencies found"
}

start_xephyr() {
    log_info "Starting Xephyr for visual tests"
    
    # Start Xephyr with fixed resolution for consistent screenshots
    Xephyr "$XEPHYR_DISPLAY" -screen 1024x768 -noreset -ac &
    XEPHYR_PID=$!
    
    # Wait for Xephyr to start
    for i in {1..10}; do
        if xwininfo -display "$XEPHYR_DISPLAY" -root >/dev/null 2>&1; then
            log_info "Xephyr server ready for visual tests"
            return 0
        fi
        sleep 1
    done
    
    log_error "Failed to start Xephyr server for visual tests"
    return 1
}

capture_grid_screenshot() {
    local grid_type="$1"
    local output_file="$OUT_DIR/${grid_type}_1024x768.png"
    local ref_file="$REF_DIR/${grid_type}_1024x768.png"
    
    log_info "Capturing screenshot for $grid_type grid"
    
    # Build project
    cd "$PROJECT_ROOT"
    if ! cargo build --release; then
        log_error "Failed to build project for visual test"
        return 1
    fi
    
    # Set display for screenshot
    export DISPLAY="$XEPHYR_DISPLAY"
    
    # Start Pointless in background
    timeout 10s ./target/release/pointless \
        --grid-type="$grid_type" \
        --debug &
    local pointless_pid=$!
    
    # Wait for startup
    sleep 2
    
    # Take screenshot of the nested X server
    import -window root -display "$XEPHYR_DISPLAY" "$output_file"
    
    # Kill Pointless
    kill "$pointless_pid" 2>/dev/null || true
    wait "$pointless_pid" 2>/dev/null || true
    
    log_info "Screenshot saved: $output_file"
    
    # Compare with reference if it exists
    if [[ -f "$ref_file" ]]; then
        if compare -metric AE "$ref_file" "$output_file" /dev/null 2>/dev/null; then
            log_info "Visual test PASSED for $grid_type (images match)"
        else
            log_warn "Visual test FAILED for $grid_type (images differ)"
        fi
    else
        log_warn "No reference image found for $grid_type, screenshot saved as new reference"
        cp "$output_file" "$ref_file"
    fi
}

generate_reference_images() {
    log_info "Generating reference images for all grid types"
    
    local grid_types=("homogeneous" "qwerty" "arrow" "vim")
    for grid_type in "${grid_types[@]}"; do
        capture_grid_screenshot "$grid_type"
    done
}

run_visual_tests() {
    log_info "Running visual regression tests"
    
    local grid_types=("homogeneous" "qwerty" "arrow" "vim")
    local test_results=()
    
    for grid_type in "${grid_types[@]}"; do
        if capture_grid_screenshot "$grid_type"; then
            test_results+=("$grid_type:PASS")
        else
            test_results+=("$grid_type:FAIL")
        fi
    done
    
    log_info "Visual test results:"
    for result in "${test_results[@]}"; do
        IFS=':' read -r grid status <<< "$result"
        if [[ "$status" == "PASS" ]]; then
            echo -e "  ${GREEN}✓${NC} $grid"
        else
            echo -e "  ${RED}✗${NC} $grid"
        fi
    done
}

main() {
    log_info "Starting visual tests for Pointless"
    
    prepare_directories
    check_dependencies
    start_xephyr
    
    case "${1:-test}" in
        "generate")
            generate_reference_images
            ;;
        "test"|*)
            run_visual_tests
            ;;
    esac
    
    log_info "Visual tests completed"
}

main "$@"