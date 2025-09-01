# Product Requirements Document: Pointless

*A keyboard-driven X11 cursor utility that makes pointing devices pointless*

IMPORTANT: Obey "AI.rule" comments in existing files.

## 1. Overview

### 1.1 Product Vision
A minimal Linux X11 utility for keyboard-driven cursor positioning using grid overlays. Built following UNIX philosophy: do one thing well, compose with other tools, simple command-line interface.

### 1.2 Core Objectives
- Replace mouse movement with keyboard-driven cursor positioning
- Modular grid layout system for extensibility
- Sub-20ms update latency for responsive interaction
- Simple command-line tool following KISS and POLA principles

## 2. Functional Requirements

### 2.1 Core Functionality
- **Cursor Positioning**: Direct X11 cursor manipulation via XWarpPointer
- **Grid Overlay**: Real-time overlay rendering at root window level
- **Modifier Activation**: Grid display triggered by configurable modifier key (default: Super/Windows key)
- **Multi-Mode Navigation**: Support for four distinct grid types

### 2.2 Grid Types

#### 2.2.1 Homogeneous Recursive Grid
- **Layout**: Screen divided into uniform rectangular cells
- **Naming**: 2D coordinates using A-Z base naming scheme
- **Navigation**: Two-key selection (e.g., "C4" for column C, row 4)
- **Recursion**: Selected cell subdivides for finer positioning
- **Use Case**: Precise positioning with consistent spatial mapping

#### 2.2.2 QWERTY Projection Grid
- **Layout**: Physical QWERTY keyboard layout projected onto screen space
- **Navigation**: Single key press maps to screen region corresponding to key position
- **Refinement**: Second key press refines within selected region
- **Use Case**: Intuitive navigation leveraging muscle memory

#### 2.2.3 Arrow Key Grid
- **Layout**: 3x3 grid with center as default position
- **Navigation**: Arrow keys select adjacent cells
- **Hierarchy**: Recursive subdivision with Escape key to ascend levels
- **Use Case**: Simple directional navigation without memorizing labels

#### 2.2.4 Vim-like Grid
- **Layout**: Numbered jump targets in geometric patterns from current cursor position
- **Basic Movement**: h/j/k/l for incremental pixel movement with optional numbers
- **Jump Commands**: w (forward pattern), b (backward pattern), f{1-9} (numbered jumps)
- **Screen Positioning**: H/M/L (top/middle/bottom), 0/$ (left/right edge), gg/G (corners)
- **Use Case**: Familiar Vim navigation without OCR, pure coordinate-based movement

### 2.3 Visual Design
- **Grid Lines**: 1-pixel yellow borders (#FFFF00)
- **Labels**: Minimal bitmap font, center-aligned within cells
- **Transparency**: Grid overlay maintains desktop visibility
- **Update Latency**: <20ms from input to visual update (event-driven, not continuous)

## 3. Technical Architecture

### 3.1 Development Environment
- **Language**: Rust with native cargo dependency management
- **Build System**: Justfile for build, test, and run commands
- **Environment**: shell.nix for Rust toolchain provisioning
- **X11 Bindings**: `x11` crate for libX11/libXext access
- **Architecture**: Layered UNIX-style modules, composable design

### 3.2 X11 Integration
- **Display Management**: XOpenDisplay for X server connection via Rust bindings
- **Cursor Control**: XWarpPointer for position updates
- **Event Handling**: XGrabKey for modifier detection
- **Overlay Rendering**: X11 window with override_redirect for grid display

### 3.3 Input System
- **Key Capture**: Low-level keyboard hook for grid navigation
- **Modifier Detection**: Real-time Super key state monitoring
- **Event Processing**: Asynchronous input handling to prevent blocking

### 3.4 Modular Architecture
- **Grid Layouts**: Pluggable grid calculation modules
- **Rendering**: Separate overlay drawing layer
- **Input Handling**: Isolated key capture and processing
- **Coordinate Mapping**: Abstracted screen-to-grid translation

### 3.5 Performance Requirements
- **Activation Latency**: <50ms from modifier press to grid display
- **Update Latency**: <20ms from key press to visual/cursor update
- **CPU Usage**: <5% during active use, <1% when idle
- **Memory Footprint**: <10MB resident memory

## 4. User Experience

### 4.1 Interaction Flow
1. User holds Super key → Grid overlay appears
2. User presses navigation key(s) → Cursor moves to selected cell
3. User releases Super key → Grid disappears, cursor remains positioned
4. Optional: User continues holding Super for refinement navigation

### 4.2 Command-Line Interface
- **Grid Mode**: `--grid-type={homogeneous|qwerty|arrow|vim}`
- **Modifier Key**: `--modifier={super|alt|ctrl}`
- **Grid Density**: `--density=N` (subdivision levels)
- **Visual**: `--color=#RRGGBB --opacity=N`
- **No Config Files**: All configuration via command-line arguments

### 4.3 Accessibility
- **High Contrast**: Support for visibility-impaired users
- **Key Alternatives**: Alternative activation methods for users unable to use Super key
- **Documentation**: Clear key mapping references

## 5. Implementation Phases

### Phase 1: Core Infrastructure
- X11 display connection and cursor control
- Basic overlay window creation
- Modifier key detection

### Phase 2: Homogeneous Grid
- Uniform grid calculation and rendering
- A-Z coordinate system implementation
- Basic two-key navigation

### Phase 3: QWERTY Grid
- QWERTY layout mapping to screen coordinates
- Single-key region selection
- Refinement subdivision

### Phase 4: Arrow Key Grid
- 3x3 grid implementation
- Directional navigation logic
- Hierarchical level management

### Phase 5: Vim Grid
- Vim-like movement commands (h/j/k/l)
- Jump target patterns (w/b/f{1-9})
- Screen positioning commands (H/M/L, 0/$, gg/G)
- Number prefix support for movements

### Phase 6: Modularity & Polish
- Pluggable grid layout system
- Command-line argument processing
- Performance optimization
- User documentation

### Phase 7: Vision-Based Grids (Optional)
- OpenCV integration as optional feature (`--features vision-grids`)
- Hough transform line detection grid
- Morphological rectangle fitting grid
- Edge density adaptive grid
- Color clustering region grid
- Feature detection grid (SIFT/ORB)
- Requires OpenCV system libraries

## 6. Success Metrics

### 6.1 Performance Benchmarks
- Grid activation time: <50ms
- Cursor positioning accuracy: 100% to target pixel
- System resource usage within specified limits

### 6.2 Usability Metrics
- Learning curve: Proficiency within 10 minutes of use
- Speed comparison: Competitive with mouse for common tasks
- User satisfaction: Positive feedback on intuitive operation

## 7. Technical Constraints

### 7.1 Dependencies
- **System Libraries**: libX11-dev, libXext-dev, libXi-dev (for input handling)
- **Rust Toolchain**: Managed via shell.nix environment
- **Rust Crates**: x11 crate for X11 bindings, additional crates as needed
- **Linux Kernel**: 3.10+ for input event handling

### 7.2 UNIX Design Principles
- **Single Purpose**: Only handles cursor positioning via grids
- **Composability**: Works with other UNIX tools via command-line
- **Simplicity**: No configuration files, minimal dependencies
- **Modularity**: Pluggable grid layouts, layered architecture

### 7.3 Compatibility
- Support for major Linux distributions
- Integration with common desktop environments (GNOME, KDE, XFCE)
- Minimal interference with existing applications

## 8. Security Considerations

- Input capture requires appropriate permissions
- No persistent storage of sensitive key combinations
- Graceful handling of X11 connection failures
- Protection against input injection attacks

## 9. Testing Strategy

### 9.1 Automated Visual Testing
- **Test Environment**: Xephyr nested X server for isolated testing
- **Screenshot Validation**: PNG reference images for grid rendering verification
- **Test Automation**: Justfile integration for automated screenshot comparison
- **Coverage**: All grid types, overlay rendering, cursor positioning accuracy

### 9.2 Integration Testing
- **X11 Compatibility**: Testing across different window managers and desktop environments
- **Performance Testing**: Latency and resource usage validation
- **Input Testing**: Modifier key detection and navigation accuracy

## 10. Future Enhancements

- Wayland protocol support
- Machine learning for adaptive grid sizing
- Integration with accessibility tools
- Cross-platform compatibility (BSD, macOS X11)
- Advanced vision algorithms (optical flow, watershed segmentation)
- Real-time adaptive grids based on screen content changes