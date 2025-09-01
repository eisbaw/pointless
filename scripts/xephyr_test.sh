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
    log_info "Testing Pointless with $grid_type grid"
    
    # Build the project first
    cd "$PROJECT_ROOT"
    if ! cargo build --release; then
        log_error "Failed to build Pointless"
        return 1
    fi
    
    # Set display for nested X server
    export DISPLAY="$XEPHYR_DISPLAY"
    
    # Run Pointless in background with debug mode
    timeout "$TEST_TIMEOUT" ./target/release/pointless \
        --grid-type="$grid_type" \
        --debug \
        2>&1 | grep -E "(ERROR|WARN|Grid|Cursor)" | head -20 &
    
    local pointless_pid=$!
    
    # Let it run for a few seconds
    sleep 5
    
    # Kill the test process
    kill "$pointless_pid" 2>/dev/null || true
    wait "$pointless_pid" 2>/dev/null || true
    
    log_info "Test completed for $grid_type grid"
}

main() {
    log_info "Starting Xephyr integration tests for Pointless"
    
    check_dependencies
    start_xephyr
    
    # Test each grid type
    local grid_types=("homogeneous" "qwerty" "arrow" "vim")
    for grid_type in "${grid_types[@]}"; do
        run_pointless_test "$grid_type" || {
            log_warn "Test failed for $grid_type grid, continuing with next..."
        }
    done
    
    log_info "All Xephyr tests completed"
}

main "$@"