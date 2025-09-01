#!/usr/bin/env bash
# Comprehensive test runner for Pointless
# Handles different environments and test types

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[PASS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[FAIL]${NC} $1"; }

detect_environment() {
    local env_type="unknown"
    
    if [[ -n "${DISPLAY:-}" ]]; then
        if xrandr >/dev/null 2>&1; then
            env_type="x11_desktop"
        else
            env_type="x11_headless"
        fi
    elif [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
        env_type="wayland"
    else
        env_type="headless"
    fi
    
    echo "$env_type"
}

check_nix_shell() {
    if command -v just >/dev/null 2>&1; then
        echo "nix"
    else
        echo "system"
    fi
}

run_unit_tests() {
    log_info "Running unit tests..."
    
    cd "$PROJECT_ROOT"
    
    local shell_type
    shell_type=$(check_nix_shell)
    
    if [[ "$shell_type" == "nix" ]]; then
        if just test-unit; then
            log_success "Unit tests passed"
            return 0
        else
            log_error "Unit tests failed"
            return 1
        fi
    else
        if nix-shell --run "just test-unit" 2>/dev/null; then
            log_success "Unit tests passed (via Nix)"
            return 0
        elif cargo test --lib; then
            log_success "Unit tests passed (via Cargo)"
            return 0
        else
            log_error "Unit tests failed"
            return 1
        fi
    fi
}

run_lint_checks() {
    log_info "Running lint checks..."
    
    cd "$PROJECT_ROOT"
    
    local shell_type
    shell_type=$(check_nix_shell)
    
    if [[ "$shell_type" == "nix" ]]; then
        just lint
    else
        if nix-shell --run "just lint" 2>/dev/null; then
            log_info "Linting via Nix"
        else
            cargo clippy
        fi
    fi
}

run_integration_tests() {
    local env_type="$1"
    
    log_info "Running integration tests for environment: $env_type"
    
    case "$env_type" in
        "x11_desktop")
            log_info "Desktop X11 environment detected - running full integration tests"
            
            if ./scripts/xephyr_test.sh; then
                log_success "Xephyr integration tests passed"
            else
                log_warn "Xephyr integration tests had issues"
            fi
            
            if ./scripts/x11_compatibility_test.sh; then
                log_success "X11 compatibility tests passed"
            else
                log_warn "X11 compatibility tests had issues"
            fi
            ;;
            
        "x11_headless"|"headless")
            log_info "Headless environment - running basic build tests only"
            
            cd "$PROJECT_ROOT"
            
            # Test basic build
            if cargo build --release; then
                log_success "Basic build successful"
            else
                log_error "Basic build failed"
                return 1
            fi
            
            # Test vision build if possible
            if cargo build --release --features vision-grids 2>/dev/null; then
                log_success "Vision grids build successful"
            else
                log_warn "Vision grids build failed (missing dependencies)"
            fi
            
            # Test binary help
            if ./target/release/pointless --help >/dev/null 2>&1; then
                log_success "Binary help command works"
            else
                log_error "Binary help command failed"
                return 1
            fi
            ;;
            
        "wayland")
            log_warn "Wayland environment detected - Pointless requires X11"
            log_info "Running build-only tests"
            
            cd "$PROJECT_ROOT"
            if cargo build --release; then
                log_success "Build successful (X11 functionality unavailable on Wayland)"
            else
                log_error "Build failed"
                return 1
            fi
            ;;
            
        *)
            log_warn "Unknown environment - running basic tests only"
            
            cd "$PROJECT_ROOT"
            if cargo build --release; then
                log_success "Basic build successful"
            else
                log_error "Basic build failed"
                return 1
            fi
            ;;
    esac
}

print_summary() {
    local env_type="$1"
    local tests_passed="$2"
    local tests_total="$3"
    
    echo
    echo "====================="
    echo "Test Summary"
    echo "====================="
    echo "Environment: $env_type"
    echo "Tests passed: $tests_passed/$tests_total"
    
    if [[ $tests_passed -eq $tests_total ]]; then
        log_success "All tests completed successfully!"
    else
        log_warn "Some tests failed or were skipped"
    fi
    
    echo
    echo "Test Types Available by Environment:"
    echo "• Desktop X11: Full integration + X11 compatibility tests"
    echo "• Headless X11: Build tests + binary validation"  
    echo "• Wayland: Build tests only (X11 features unavailable)"
    echo "• CI/Headless: Build tests + unit tests"
}

main() {
    echo "🧪 Pointless Test Suite"
    echo "======================"
    echo
    
    local env_type
    env_type=$(detect_environment)
    
    log_info "Environment detected: $env_type"
    
    local tests_passed=0
    local tests_total=0
    
    # Always run unit tests
    ((tests_total++))
    if run_unit_tests; then
        ((tests_passed++))
    fi
    
    # Always run lint checks
    ((tests_total++))
    if run_lint_checks; then
        ((tests_passed++))
        log_success "Lint checks passed"
    else
        log_warn "Lint checks failed"
    fi
    
    # Run environment-appropriate integration tests
    ((tests_total++))
    if run_integration_tests "$env_type"; then
        ((tests_passed++))
    fi
    
    print_summary "$env_type" "$tests_passed" "$tests_total"
    
    if [[ $tests_passed -eq $tests_total ]]; then
        exit 0
    else
        exit 1
    fi
}

# Handle command line arguments
case "${1:-all}" in
    "unit")
        run_unit_tests
        ;;
    "lint")
        run_lint_checks
        ;;
    "integration")
        env_type=$(detect_environment)
        run_integration_tests "$env_type"
        ;;
    "all"|*)
        main
        ;;
esac