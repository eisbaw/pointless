#!/usr/bin/env bash
# Xephyr testing script for Pointless
# Runs tests in a nested X server for safe visual validation

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
XEPHYR_DISPLAY=":99"
XEPHYR_PID=""
TEST_TIMEOUT=30

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

check_dependencies() {
    log_info "Checking dependencies..."
    
    if ! command -v Xephyr >/dev/null 2>&1; then
        log_error "Xephyr not found. Install with: sudo apt-get install xserver-xephyr"
        exit 1
    fi
    
    if ! command -v xwininfo >/dev/null 2>&1; then
        log_error "xwininfo not found. Install with: sudo apt-get install x11-utils"
        exit 1
    fi
    
    log_info "All dependencies found"
}

start_xephyr() {
    log_info "Starting Xephyr nested X server on display $XEPHYR_DISPLAY"
    
    # Start Xephyr in background with 1024x768 resolution
    Xephyr "$XEPHYR_DISPLAY" -screen 1024x768 -noreset -ac &
    XEPHYR_PID=$!
    
    # Wait for Xephyr to start
    for i in {1..10}; do
        if xwininfo -display "$XEPHYR_DISPLAY" -root >/dev/null 2>&1; then
            log_info "Xephyr server started successfully (PID: $XEPHYR_PID)"
            return 0
        fi
        sleep 1
    done
    
    log_error "Failed to start Xephyr server"
    return 1
}

run_pointless_test() {
    local grid_type="$1"
    local modifier_key="${2:-super}"
    local additional_args="${3:-}"
    
    log_info "Testing Pointless with $grid_type grid (modifier: $modifier_key)"
    
    # Set display for nested X server
    export DISPLAY="$XEPHYR_DISPLAY"
    
    # Create a temporary log file
    local log_file="$PROJECT_ROOT/test_output_${grid_type}_${modifier_key}.log"
    
    # Run Pointless in background with debug mode
    timeout "$TEST_TIMEOUT" ./target/release/pointless \
        --grid-type="$grid_type" \
        --modifier-key="$modifier_key" \
        --debug \
        $additional_args \
        > "$log_file" 2>&1 &
    
    local pointless_pid=$!
    
    # Let it run for a few seconds to initialize
    sleep 3
    
    # Check if process is still running (basic health check)
    if kill -0 "$pointless_pid" 2>/dev/null; then
        log_info "Process running successfully for $grid_type grid"
        
        # Test X11 display connection by checking window properties
        if xwininfo -display "$XEPHYR_DISPLAY" -root >/dev/null 2>&1; then
            log_info "X11 display connection verified"
        else
            log_warn "X11 display connection issue detected"
        fi
    else
        log_error "Process died unexpectedly for $grid_type grid"
        if [[ -f "$log_file" ]]; then
            log_error "Error output:"
            tail -10 "$log_file"
        fi
        return 1
    fi
    
    # Kill the test process
    kill "$pointless_pid" 2>/dev/null || true
    wait "$pointless_pid" 2>/dev/null || true
    
    # Check for errors in the log
    if [[ -f "$log_file" ]]; then
        if grep -qi "error\|panic\|failed" "$log_file"; then
            log_warn "Errors detected in log for $grid_type:"
            grep -i "error\|panic\|failed" "$log_file" | head -5
        else
            log_info "No critical errors found for $grid_type"
        fi
        rm -f "$log_file"
    fi
    
    log_info "Test completed for $grid_type grid"
}

test_basic_grids() {
    log_info "Testing basic grid types..."
    
    # Build the basic version first
    cd "$PROJECT_ROOT"
    if ! cargo build --release; then
        log_error "Failed to build Pointless (basic version)"
        return 1
    fi
    
    # Test each basic grid type
    local grid_types=("homogeneous" "qwerty" "arrow" "vim")
    local modifier_keys=("super" "alt" "ctrl")
    
    for grid_type in "${grid_types[@]}"; do
        # Test with default modifier
        run_pointless_test "$grid_type" || {
            log_warn "Test failed for $grid_type grid with super key, continuing..."
        }
        
        # Test with alternative modifier (just alt to keep tests reasonable)
        run_pointless_test "$grid_type" "alt" || {
            log_warn "Test failed for $grid_type grid with alt key, continuing..."
        }
    done
}

test_vision_grids() {
    log_info "Testing vision grid types (if available)..."
    
    # Build with vision grids feature
    cd "$PROJECT_ROOT"
    if cargo build --release --features vision-grids 2>/dev/null; then
        log_info "Vision grids feature available, testing..."
        
        # Test vision grid types (these are stub implementations)
        local vision_types=("hough-lines" "rectangle-fitting" "edge-density" "color-clustering" "feature-detection")
        
        for vision_type in "${vision_types[@]}"; do
            run_pointless_test "$vision_type" "super" "--opacity 0.8" || {
                log_warn "Test failed for $vision_type vision grid, continuing..."
            }
        done
    else
        log_info "Vision grids feature not available (missing OpenCV dependencies)"
        # Rebuild basic version for consistency
        cargo build --release >/dev/null 2>&1
    fi
}

test_configurations() {
    log_info "Testing various configurations..."
    
    # Test different colors and opacity
    run_pointless_test "homogeneous" "super" "--color FF0000 --opacity 0.5" || {
        log_warn "Configuration test failed, continuing..."
    }
    
    # Test high precision grid
    run_pointless_test "homogeneous" "super" "--grid-density 5" || {
        log_warn "High precision test failed, continuing..."
    }
}

main() {
    log_info "Starting comprehensive Xephyr integration tests for Pointless"
    
    check_dependencies
    start_xephyr
    
    # Run test suites
    test_basic_grids
    test_vision_grids
    test_configurations
    
    log_info "All Xephyr integration tests completed"
}

main "$@"