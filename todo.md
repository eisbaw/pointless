# TODO List for Pointless

AI.rule: Keep architecture_design.md updated as tasks are completed.
AI.rule: Keep readme.md updated as tasks are completed.


## Phase 1: Core Infrastructure
- DONE: Set up Cargo project with shell.nix environment
- DONE: Implement X11 connection and display management
- DONE: Create basic X11 cursor control using XWarpPointer
- DONE: Implement modifier key detection with XGrabKey (Note: Super key conflicts with WM, use Alt)
- DONE: Create basic overlay window with override_redirect
- DONE: Add command-line argument parsing

## Phase 2: Homogeneous Grid
- DONE: Implement uniform grid calculation for screen division
- DONE: Create A-Z coordinate naming system (basic implementation)
- DONE: Add two-key navigation (column + row selection)
- DONE: Implement recursive cell subdivision
- DONE: Add grid rendering with 1-pixel yellow borders
- DONE: Add minimal bitmap font labels

## Phase 3: QWERTY Grid
- DONE: Map QWERTY key positions to screen coordinates
- DONE: Implement single key to region mapping
- DONE: Add refinement subdivision within selected regions
- DONE: Create visual indicators for QWERTY layout projection

## Phase 4: Arrow Key Grid
- DONE: Implement 3x3 grid system with center default
- DONE: Add arrow key navigation between cells
- DONE: Create hierarchical subdivision with Enter key
- DONE: Add Escape key to ascend grid levels
- DONE: Handle recursive grid state management

## Phase 5: Vim Grid
- DONE: Implement h/j/k/l incremental movement
- DONE: Add number prefix support for movements
- DONE: Create w/b jump target patterns (forward/backward)
- DONE: Implement f{1-9} numbered jump system (basic implementation)
- DONE: Add H/M/L screen positioning (basic implementation)
- DONE: Implement 0/$ edge positioning and gg/G corners (basic implementation)

## Phase 6: Modularity & Polish
- DONE: Create pluggable GridLayout trait system
- DONE: Refactor all grids to use common interface
- DONE: Add grid type selection via command line
- DONE: Implement visual customization (color, opacity)
- DONE: Add modifier key configuration
- DONE: Performance optimization for sub-20ms latency
- DONE: Create comprehensive error handling

## Phase 7: Vision-Based Grids (Optional)
- DONE: Add OpenCV as optional Cargo feature
- DONE: Implement Hough line detection grid (stub implementation)
- DONE: Create morphological rectangle fitting (stub implementation)
- DONE: Add edge density adaptive grid (stub implementation)  
- DONE: Implement color clustering region detection (stub implementation)
- DONE: Create SIFT/ORB feature detection grid (stub implementation)
- TODO: Enhance stub implementations with full OpenCV integration

## Testing & Documentation
- DONE: Set up Xephyr testing environment
- DONE: Create visual reference PNG screenshots
- DONE: Implement automated screenshot comparison
- DONE: Write unit tests for all grid types
- Add integration tests for X11 compatibility
- Create man page documentation
- Write usage examples and tutorials

## Critical Fixes (MPED Review Issues)
- DONE: Fix silent X11 key grab failure - add proper error detection
- DONE: Add proper signal handling for cleanup (SIGINT/SIGTERM)
- DONE: Refactor grid state management to avoid inconsistencies
- DONE: Add proper error types (InvalidGridDensity, InvalidOpacity)
- DONE: Fix infinite loop without cleanup in main event loop
- DONE: Remove hardcoded magic numbers in grid size calculations
- DONE: Add proper logging instead of println! statements
- DONE: Fix X11 library linking issues in Nix environment using cargo config
- DONE: Fix critical clippy error in signal handling (never-loop pattern)
- DONE: Fix additional clippy warnings and add Default implementations

## Build System
- DONE: Create Justfile with all build commands
- DONE: Add separate builds for basic vs vision features
- Set up CI/CD pipeline
- Create installation scripts
- Package for major Linux distributions
