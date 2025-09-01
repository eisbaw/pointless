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
- Add two-key navigation (column + row selection)
- Implement recursive cell subdivision
- DONE: Add grid rendering with 1-pixel yellow borders
- DONE: Add minimal bitmap font labels

## Phase 3: QWERTY Grid
- Map QWERTY key positions to screen coordinates
- Implement single key to region mapping
- Add refinement subdivision within selected regions
- Create visual indicators for QWERTY layout projection

## Phase 4: Arrow Key Grid
- Implement 3x3 grid system with center default
- Add arrow key navigation between cells
- Create hierarchical subdivision with Enter key
- Add Escape key to ascend grid levels
- Handle recursive grid state management

## Phase 5: Vim Grid
- Implement h/j/k/l incremental movement
- Add number prefix support for movements
- Create w/b jump target patterns (forward/backward)
- Implement f{1-9} numbered jump system
- Add H/M/L screen positioning (top/middle/bottom)
- Implement 0/$ edge positioning and gg/G corners

## Phase 6: Modularity & Polish
- DONE: Create pluggable GridLayout trait system
- DONE: Refactor all grids to use common interface
- DONE: Add grid type selection via command line
- DONE: Implement visual customization (color, opacity)
- DONE: Add modifier key configuration
- Performance optimization for sub-20ms latency
- DONE: Create comprehensive error handling

## Phase 7: Vision-Based Grids (Optional)
- Add OpenCV as optional Cargo feature
- Implement Hough line detection grid
- Create morphological rectangle fitting
- Add edge density adaptive grid
- Implement color clustering region detection
- Create SIFT/ORB feature detection grid

## Testing & Documentation
- Set up Xephyr testing environment
- Create visual reference PNG screenshots
- Implement automated screenshot comparison
- Write unit tests for all grid types
- Add integration tests for X11 compatibility
- Create man page documentation
- Write usage examples and tutorials

## Build System
- DONE: Create Justfile with all build commands
- DONE: Add separate builds for basic vs vision features
- Set up CI/CD pipeline
- Create installation scripts
- Package for major Linux distributions
