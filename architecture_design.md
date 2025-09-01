# Architecture Design: Pointless

*Making pointing devices pointless through keyboard-driven cursor control*

## 1. Overview

This document details the modular architecture for Pointless, a UNIX-style X11 cursor positioning utility. The design follows layered composition principles with pluggable components.

## 2. System Architecture

### 2.1 High-Level Layers

```
┌─────────────────────────────────────────┐
│               main.rs                   │
│           (CLI & Orchestration)         │
├─────────────────────────────────────────┤
│            input_handler.rs             │
│        (Key Capture & Processing)       │
├─────────────────────────────────────────┤
│             grid_manager.rs             │
│         (Grid Logic Coordinator)        │
├─────────────────────────────────────────┤
│              grids/*.rs                 │
│         (Pluggable Grid Layouts)        │
├─────────────────────────────────────────┤
│             renderer.rs                 │
│          (X11 Overlay Drawing)          │
├─────────────────────────────────────────┤
│             x11_client.rs               │
│        (X11 Connection & Control)       │
└─────────────────────────────────────────┘
```

### 2.2 Module Responsibilities

#### **main.rs**
- Command-line argument parsing
- Module initialization and orchestration
- Event loop coordination
- Clean shutdown handling

#### **input_handler.rs**
- X11 key event capture (XGrabKey)
- Modifier key state tracking
- Key sequence processing
- Event dispatching to grid manager

#### **grid_manager.rs**
- Grid type selection and instantiation
- Coordinate translation (screen ↔ grid)
- Navigation state management
- Grid subdivision logic

#### **grids/mod.rs + grids/*.rs**
- Pluggable grid layout implementations
- Each grid type in separate module:
  - `grids/homogeneous.rs`
  - `grids/qwerty.rs`
  - `grids/arrow.rs`
  - `grids/vim.rs`
  - `grids/vision/` (optional feature)
    - `grids/vision/hough.rs`
    - `grids/vision/rectangles.rs`
    - `grids/vision/edges.rs`
    - `grids/vision/colors.rs`
    - `grids/vision/features.rs`

#### **renderer.rs**
- X11 overlay window creation
- Grid line and label drawing
- Visual update management
- Window positioning and transparency

#### **x11_client.rs**
- X11 connection management
- Cursor positioning (XWarpPointer)
- Display information queries
- Error handling and recovery

## 3. Data Structures

### 3.1 Core Types

```rust
// Grid coordinate system
#[derive(Debug, Clone, Copy)]
pub struct GridCoord {
    pub x: u32,
    pub y: u32,
}

// Screen pixel coordinates
#[derive(Debug, Clone, Copy)]
pub struct ScreenCoord {
    pub x: i32,
    pub y: i32,
}

// Grid cell definition
#[derive(Debug, Clone)]
pub struct GridCell {
    pub coord: GridCoord,
    pub screen_rect: Rectangle,
    pub label: String,
}

// Screen dimensions and properties
#[derive(Debug, Clone)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    pub root_window: u64,
}
```

### 3.2 Configuration

```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_type: GridType,
    pub modifier_key: ModifierKey,
    pub grid_density: u32,
    pub color: u32,  // RGB hex value
    pub opacity: f32,
}

#[derive(Debug, Clone)]
pub enum GridType {
    Homogeneous,
    Qwerty,
    Arrow,
    Vim,
    #[cfg(feature = "vision-grids")]
    HoughLines,
    #[cfg(feature = "vision-grids")]
    RectangleFitting,
    #[cfg(feature = "vision-grids")]
    EdgeDensity,
    #[cfg(feature = "vision-grids")]
    ColorClustering,
    #[cfg(feature = "vision-grids")]
    FeatureDetection,
}

#[derive(Debug, Clone)]
pub enum ModifierKey {
    Super,
    Alt,
    Ctrl,
}
```

## 4. Module Interfaces

### 4.1 GridLayout Trait (Pluggable Interface)

```rust
pub trait GridLayout {
    /// Calculate grid cells for given screen dimensions
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell>;
    
    /// Process key input and return target cell
    fn process_key(&mut self, key: KeyCode, current_level: u32) -> Option<GridCoord>;
    
    /// Get human-readable name for this grid type
    fn name(&self) -> &'static str;
    
    /// Reset navigation state
    fn reset(&mut self);
}
```

### 4.2 InputHandler Interface

```rust
pub trait InputHandler {
    /// Start capturing input events
    fn start_capture(&mut self) -> Result<(), X11Error>;
    
    /// Process next input event (blocking)
    fn next_event(&mut self) -> Result<InputEvent, X11Error>;
    
    /// Stop capturing and release resources
    fn stop_capture(&mut self);
}

#[derive(Debug)]
pub enum InputEvent {
    ModifierPressed,
    ModifierReleased,
    NavigationKey(KeyCode),
    Unknown,
}
```

### 4.3 Renderer Interface

```rust
pub trait Renderer {
    /// Create overlay window
    fn create_overlay(&mut self, screen: &ScreenInfo) -> Result<(), X11Error>;
    
    /// Draw grid cells
    fn draw_grid(&mut self, cells: &[GridCell], config: &Config) -> Result<(), X11Error>;
    
    /// Hide overlay
    fn hide_overlay(&mut self) -> Result<(), X11Error>;
    
    /// Destroy overlay and cleanup
    fn destroy_overlay(&mut self);
}
```

## 5. Control Flow

### 5.1 Initialization Sequence

1. **main.rs**: Parse command-line arguments → Config
2. **x11_client.rs**: Connect to X11 display, get screen info
3. **input_handler.rs**: Setup key grabbing for modifier key
4. **grid_manager.rs**: Instantiate selected grid layout
5. **renderer.rs**: Prepare overlay window (hidden initially)

### 5.2 Runtime Event Loop

```
┌─ Input Event ─────────────────────┐
│                                   │
▼                                   │
┌─────────────────┐                │
│  ModifierPressed │                │
└─────────┬───────┘                │
          │                        │
          ▼                        │
┌─────────────────┐                │
│ Show Grid       │                │
│ Start Navigation│                │
└─────────┬───────┘                │
          │                        │
          ▼                        │
┌─────────────────┐                │
│ NavigationKey   │                │
└─────────┬───────┘                │
          │                        │
          ▼                        │
┌─────────────────┐                │
│ Update Cursor   │                │
│ Redraw Grid     │                │
└─────────┬───────┘                │
          │                        │
          ▼                        │
┌─────────────────┐                │
│ ModifierReleased│ ───────────────┘
└─────────┬───────┘
          │
          ▼
┌─────────────────┐
│ Hide Grid       │
│ Reset State     │
└─────────────────┘
```

## 6. Grid Layout Implementations

### 6.1 Homogeneous Grid (grids/homogeneous.rs)

```rust
pub struct HomogeneousGrid {
    cols: u32,
    rows: u32,
    current_level: u32,
    selected_cell: Option<GridCoord>,
}

impl GridLayout for HomogeneousGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        // Divide screen into uniform rectangular cells
        // Generate A-Z labels for each cell
    }
    
    fn process_key(&mut self, key: KeyCode, _level: u32) -> Option<GridCoord> {
        // Two-key selection: first key = column, second key = row
        // Handle recursive subdivision
    }
}
```

### 6.2 QWERTY Grid (grids/qwerty.rs)

```rust
pub struct QwertyGrid {
    key_map: HashMap<KeyCode, ScreenCoord>,
    refinement_level: u32,
    selected_region: Option<Rectangle>,
}

impl GridLayout for QwertyGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        // Map QWERTY key positions to screen regions
        // Generate cells based on physical keyboard layout
    }
    
    fn process_key(&mut self, key: KeyCode, _level: u32) -> Option<GridCoord> {
        // Single key maps to screen region
        // Second key refines within that region
    }
}
```

### 6.3 Arrow Grid (grids/arrow.rs)

```rust
pub struct ArrowGrid {
    grid_stack: Vec<GridLevel>,  // For hierarchical navigation
    current_pos: GridCoord,      // Current position in 3x3 grid
}

struct GridLevel {
    center: ScreenCoord,
    size: u32,
}

impl GridLayout for ArrowGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        // Create 3x3 grid around current position
        // Handle recursive subdivision levels
    }
    
    fn process_key(&mut self, key: KeyCode, _level: u32) -> Option<GridCoord> {
        // Arrow keys move within 3x3 grid
        // Enter key descends a level
        // Escape key ascends a level
    }
}
```

### 6.4 Vim Grid (grids/vim.rs)

```rust
pub struct VimGrid {
    current_cursor: ScreenCoord,
    jump_targets: Vec<VimJumpTarget>,
    movement_mode: VimMovementMode,
    pending_multiplier: Option<u32>,
}

#[derive(Debug, Clone)]
struct VimJumpTarget {
    position: ScreenCoord,
    label: char,  // 1-9 for jump targets
}

#[derive(Debug, Clone)]
enum VimMovementMode {
    Normal,
    FindForward,   // After 'w' pressed
    FindBackward,  // After 'b' pressed
    WaitingNumber, // After 'f' pressed
}

impl GridLayout for VimGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        match self.movement_mode {
            VimMovementMode::FindForward => {
                // Generate 9 numbered targets in forward/right pattern from cursor
            }
            VimMovementMode::FindBackward => {
                // Generate 9 numbered targets in backward/left pattern from cursor
            }
            _ => vec![], // No grid overlay for basic h/j/k/l movement
        }
    }
    
    fn process_key(&mut self, key: KeyCode, _level: u32) -> Option<GridCoord> {
        match key {
            // Basic movement: h/j/k/l with optional number prefix
            KeyCode::H => self.move_relative(-10, 0),
            KeyCode::J => self.move_relative(0, 10),
            KeyCode::K => self.move_relative(0, -10), 
            KeyCode::L => self.move_relative(10, 0),
            
            // Jump commands
            KeyCode::W => self.enter_find_forward_mode(),
            KeyCode::B => self.enter_find_backward_mode(),
            KeyCode::F => self.enter_find_number_mode(),
            
            // Screen positioning
            KeyCode::H_UPPER => self.jump_to_top(),
            KeyCode::M_UPPER => self.jump_to_middle(),
            KeyCode::L_UPPER => self.jump_to_bottom(),
            KeyCode::Zero => self.jump_to_left_edge(),
            KeyCode::Dollar => self.jump_to_right_edge(),
            
            // Numbers for jump targets
            KeyCode::Num1..=KeyCode::Num9 => self.jump_to_numbered_target(key),
            
            _ => None,
        }
    }
}
```

## 7. Error Handling Strategy

### 7.1 Error Types

```rust
#[derive(Debug)]
pub enum AppError {
    X11Error(String),
    InvalidConfig(String),
    InputError(String),
    RenderError(String),
}

impl From<x11::xlib::XError> for AppError {
    fn from(err: x11::xlib::XError) -> Self {
        AppError::X11Error(format!("X11 error: {:?}", err))
    }
}
```

### 7.2 Recovery Strategies

- **X11 Connection Loss**: Attempt reconnection, fall back to graceful exit
- **Key Grab Failure**: Log warning, continue without exclusive grab
- **Render Failure**: Hide overlay, continue with cursor-only mode
- **Invalid Input**: Ignore unknown keys, maintain current state

## 8. Testing Architecture

### 8.1 Unit Testing Strategy

- **Grid Layouts**: Test coordinate calculations with mock screen dimensions
- **Input Processing**: Test key sequence handling with synthetic events
- **Coordinate Mapping**: Verify screen ↔ grid transformations

### 8.2 Integration Testing with Xephyr

- **Visual Validation**: Screenshot comparison against reference PNGs
- **End-to-End Testing**: Full interaction sequences in nested X server
- **Performance Testing**: Latency measurements for input → visual update

### 8.3 Test Structure

```
tests/
├── unit/
│   ├── grid_homogeneous_test.rs
│   ├── grid_qwerty_test.rs
│   ├── grid_arrow_test.rs
│   ├── grid_vim_test.rs
│   └── coordinate_mapping_test.rs
├── integration/
│   ├── xephyr_visual_test.rs
│   └── end_to_end_test.rs
└── reference_images/
    ├── homogeneous_1920x1080.png
    ├── qwerty_1920x1080.png
    ├── arrow_1920x1080.png
    └── vim_1920x1080.png
```

## 9. Build and Deployment

### 9.1 Justfile Commands

```make
# Build the application
build:
    cargo build --release

# Run unit tests
test-unit:
    cargo test --lib

# Run integration tests with Xephyr
test-integration:
    ./scripts/xephyr_test.sh

# Run visual validation tests
test-visual:
    ./scripts/visual_test.sh

# Build with vision grids
build-vision:
    cargo build --release --features vision-grids

# Run unit tests with vision grids
test-unit-vision:
    cargo test --lib --features vision-grids

# Install to system
install: build
    sudo cp target/release/pointless /usr/local/bin/

# Install vision-enabled version
install-vision: build-vision
    sudo cp target/release/pointless /usr/local/bin/pointless-vision
```

### 9.2 Deployment Structure

```
/usr/local/bin/pointless         # Main executable
/usr/local/bin/pointless-vision  # Vision-enabled version (optional)
/usr/share/man/man1/pointless.1  # Manual page
```

## 10. Vision-Based Grids Architecture

### 10.1 Optional Feature Implementation

```rust
// Cargo.toml
[dependencies]
x11 = "2.21"
opencv = { version = "0.92", optional = true }

[features]
default = ["basic-grids"]
basic-grids = []
vision-grids = ["opencv"]
```

### 10.2 Vision Grid Implementations

```rust
#[cfg(feature = "vision-grids")]
pub struct HoughGrid {
    screenshot: opencv::core::Mat,
    detected_lines: Vec<opencv::core::Vec4i>,
    intersection_points: Vec<ScreenCoord>,
}

#[cfg(feature = "vision-grids")]
impl GridLayout for HoughGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        // Take screenshot using X11 XGetImage
        // Apply Hough line transform
        // Find line intersections as jump targets
        // Generate numbered cells at intersections
    }
    
    fn process_key(&mut self, key: KeyCode, _level: u32) -> Option<GridCoord> {
        // Jump to numbered intersection points
        // Refresh screenshot periodically for dynamic content
    }
}

#[cfg(feature = "vision-grids")]
pub struct RectangleGrid {
    screenshot: opencv::core::Mat,
    detected_rects: Vec<opencv::core::Rect>,
    morph_kernel: opencv::core::Mat,
}

#[cfg(feature = "vision-grids")]
impl GridLayout for RectangleGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        // Take screenshot
        // Apply morphological operations (opening/closing)
        // Detect rectangular contours using findContours
        // Create jump targets for each rectangle center
        // Filter by minimum size and aspect ratio
    }
}
```

### 10.3 Dependencies

**System Requirements for Vision Grids:**
- libopencv-dev (OpenCV 4.x recommended)
- libopencv-contrib-dev (for additional algorithms)
- pkg-config (for OpenCV detection)

**Performance Considerations:**
- Screenshot capture adds ~5-10ms latency
- Computer vision processing adds ~50-200ms depending on algorithm
- Caching and incremental updates minimize impact
- Vision grids refresh only on explicit user request or timer

This architecture provides clean separation of concerns, pluggable grid systems, and follows UNIX principles while maintaining testability and maintainability. Vision-based grids are completely optional and don't affect core functionality.
