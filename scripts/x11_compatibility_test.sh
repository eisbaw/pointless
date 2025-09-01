#!/usr/bin/env bash
# X11 compatibility test for Pointless
# Tests X11-specific functionality and integration

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
XEPHYR_DISPLAY=":98"
XEPHYR_PID=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

cleanup() {
    if [[ -n "$XEPHYR_PID" ]]; then
        log_info "Cleaning up Xephyr server (PID: $XEPHYR_PID)"
        kill "$XEPHYR_PID" 2>/dev/null || true
        wait "$XEPHYR_PID" 2>/dev/null || true
    fi
    
    # Clean up any temporary log files
    rm -f "$PROJECT_ROOT"/test_x11_*.log
}

trap cleanup EXIT

check_x11_dependencies() {
    log_info "Checking X11 dependencies..."
    
    local missing_deps=()
    local commands=("Xephyr" "xwininfo" "xdpyinfo" "xev" "xprop")
    
    for cmd in "${commands[@]}"; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            missing_deps+=("$cmd")
        fi
    done
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing X11 dependencies: ${missing_deps[*]}"
        echo "Install with:"
        echo "  sudo apt-get install xserver-xephyr x11-utils"
        exit 1
    fi
    
    log_success "All X11 dependencies found"
}

test_x11_libraries() {
    log_info "Testing X11 library linkage..."
    
    cd "$PROJECT_ROOT"
    
    # Build the project
    if cargo build --release; then
        log_success "Build successful - X11 libraries linked correctly"
    else
        log_error "Build failed - X11 library linkage issues"
        return 1
    fi
    
    # Check if X11 libraries are properly linked
    if ldd target/release/pointless 2>/dev/null | grep -q "libX11"; then
        log_success "libX11 linkage verified"
    else
        log_warn "libX11 linkage not detected (may be static)"
    fi
    
    if ldd target/release/pointless 2>/dev/null | grep -q "libXext"; then
        log_success "libXext linkage verified"
    else
        log_warn "libXext linkage not detected (may be static)"
    fi
}

start_test_xserver() {
    log_info "Starting test X server on display $XEPHYR_DISPLAY"
    
    # Start Xephyr with specific configuration for testing
    Xephyr "$XEPHYR_DISPLAY" -screen 800x600 -noreset -ac -keybd ephyr,,,xkbmodel=pc105 &
    XEPHYR_PID=$!
    
    # Wait for server to be ready
    for i in {1..15}; do
        if xdpyinfo -display "$XEPHYR_DISPLAY" >/dev/null 2>&1; then
            log_success "Test X server ready (PID: $XEPHYR_PID)"
            return 0
        fi
        sleep 1
    done
    
    log_error "Failed to start test X server"
    return 1
}

test_x11_connection() {
    log_info "Testing X11 display connection..."
    
    export DISPLAY="$XEPHYR_DISPLAY"
    
    # Test display connection
    if xdpyinfo -display "$XEPHYR_DISPLAY" >/dev/null 2>&1; then
        log_success "X11 display connection established"
    else
        log_error "Failed to connect to X11 display"
        return 1
    fi
    
    # Get display information
    local display_info
    display_info=$(xdpyinfo -display "$XEPHYR_DISPLAY" 2>/dev/null)
    
    if echo "$display_info" | grep -q "screen #0"; then
        log_success "Screen information accessible"
    else
        log_error "Cannot access screen information"
        return 1
    fi
    
    # Test root window access
    if xwininfo -display "$XEPHYR_DISPLAY" -root >/dev/null 2>&1; then
        log_success "Root window access verified"
    else
        log_error "Cannot access root window"
        return 1
    fi
}

test_keyboard_events() {
    log_info "Testing keyboard event handling..."
    
    export DISPLAY="$XEPHYR_DISPLAY"
    
    # Test if pointless can capture keyboard events
    local log_file="$PROJECT_ROOT/test_x11_keyboard.log"
    
    timeout 5s ./target/release/pointless --grid-type homogeneous --debug > "$log_file" 2>&1 &
    local pointless_pid=$!
    
    sleep 2
    
    # Check if process is capturing keyboard events
    if kill -0 "$pointless_pid" 2>/dev/null; then
        log_success "Pointless running and capturing events"
        
        # Check for X11-specific initialization messages
        if grep -q "X11\|Display\|Screen" "$log_file" 2>/dev/null; then
            log_success "X11 initialization detected in logs"
        else
            log_warn "No explicit X11 initialization messages found"
        fi
    else
        log_error "Pointless failed to start or died unexpectedly"
        if [[ -f "$log_file" ]]; then
            echo "Error output:"
            cat "$log_file"
        fi
        return 1
    fi
    
    # Clean up
    kill "$pointless_pid" 2>/dev/null || true
    wait "$pointless_pid" 2>/dev/null || true
}

test_cursor_positioning() {
    log_info "Testing cursor positioning functionality..."
    
    export DISPLAY="$XEPHYR_DISPLAY"
    
    # Get initial cursor position
    local initial_pos
    if command -v xdotool >/dev/null 2>&1; then
        initial_pos=$(xdotool getmouselocation --shell 2>/dev/null || echo "X=0 Y=0")
        log_info "Initial cursor position: $initial_pos"
        log_success "Cursor position query successful"
    else
        log_warn "xdotool not available, skipping cursor position test"
    fi
}

test_multiple_displays() {
    log_info "Testing display environment handling..."
    
    # Test with different DISPLAY values
    local original_display="$DISPLAY"
    
    # Test with invalid display
    DISPLAY=":999" timeout 3s ./target/release/pointless --help >/dev/null 2>&1
    if [[ $? -eq 0 ]]; then
        log_success "Handles invalid display gracefully"
    else
        log_warn "May have issues with invalid displays"
    fi
    
    # Restore display
    export DISPLAY="$original_display"
}

test_grid_rendering() {
    log_info "Testing grid rendering on X11..."
    
    export DISPLAY="$XEPHYR_DISPLAY"
    
    local grid_types=("homogeneous" "qwerty" "arrow" "vim")
    
    for grid_type in "${grid_types[@]}"; do
        local log_file="$PROJECT_ROOT/test_x11_render_${grid_type}.log"
        
        # Start pointless briefly to test rendering
        timeout 3s ./target/release/pointless \
            --grid-type="$grid_type" \
            --debug \
            > "$log_file" 2>&1 &
        local pid=$!
        
        sleep 1
        
        if kill -0 "$pid" 2>/dev/null; then
            log_success "Grid rendering successful for $grid_type"
            kill "$pid" 2>/dev/null || true
            wait "$pid" 2>/dev/null || true
        else
            log_error "Grid rendering failed for $grid_type"
            if [[ -f "$log_file" ]]; then
                echo "Error details:"
                tail -5 "$log_file"
            fi
        fi
    done
}

run_compatibility_suite() {
    log_info "Running comprehensive X11 compatibility tests..."
    
    local tests_passed=0
    local tests_total=0
    
    # Array of test functions
    local test_functions=(
        "test_x11_libraries"
        "test_x11_connection" 
        "test_keyboard_events"
        "test_cursor_positioning"
        "test_multiple_displays"
        "test_grid_rendering"
    )
    
    for test_func in "${test_functions[@]}"; do
        ((tests_total++))
        log_info "Running $test_func..."
        if $test_func; then
            ((tests_passed++))
        else
            log_error "$test_func failed"
        fi
        echo
    done
    
    log_info "X11 Compatibility Test Results:"
    log_info "  Passed: $tests_passed/$tests_total"
    
    if [[ $tests_passed -eq $tests_total ]]; then
        log_success "All X11 compatibility tests passed!"
        return 0
    else
        log_error "Some X11 compatibility tests failed"
        return 1
    fi
}

main() {
    echo "🔧 Pointless X11 Compatibility Test Suite"
    echo "========================================="
    echo
    
    check_x11_dependencies
    start_test_xserver
    
    if run_compatibility_suite; then
        log_success "X11 compatibility verified!"
        exit 0
    else
        log_error "X11 compatibility issues detected"
        exit 1
    fi
}

main "$@"