//! Pointless - Keyboard-driven X11 cursor positioning utility
//!
//! A UNIX-style command-line tool that makes pointing devices pointless by providing
//! keyboard-driven cursor positioning using grid overlay systems.
//!
//! ## Architecture Overview
//!
//! Pointless follows MPED (Mark Pedersen) architectural principles:
//! - **Fundamentals First**: Proper error handling, fail-fast design, structured logging
//! - **Composability**: Modular design with pluggable grid layouts and clean interfaces  
//! - **Data Design**: Minimal state, compute-on-demand, clear data structures
//! - **UNIX Principles**: Single responsibility, command-line interface, no config files
//!
//! ## Core Modules
//!
//! - **grids**: Pluggable grid layout implementations (homogeneous, qwerty, arrow, vim, vision)
//! - **x11_client**: Safe X11 wrapper for display connection and cursor control
//! - **input_handler**: Keyboard input capture and modifier key detection
//! - **renderer**: Grid overlay rendering with transparency
//! - **grid_manager**: Coordinates grid layout selection and state management
//! - **coordinate_resolver**: Efficient coordinate transformation utilities
//!
//! ## Features
//!
//! ### Basic Grids (always available)
//! - **Homogeneous**: Uniform rectangular grid with A-Z coordinates
//! - **QWERTY**: Physical keyboard layout projection
//! - **Arrow**: 3x3 directional navigation
//! - **Vim**: Vim-style movement commands
//!
//! ### Vision Grids (optional, requires `vision-grids` feature)
//! - **Hough**: Line detection intersection points
//! - **Rectangle**: UI element detection
//! - **Edge**: Content-aware positioning  
//! - **Color**: Color clustering regions
//! - **Feature**: SIFT/ORB feature detection
//!
//! ## Usage
//!
//! ```bash
//! pointless --grid-type homogeneous --modifier alt
//! pointless --grid-type vim --density 3 --debug
//! ```
//!
//! ## Performance
//!
//! - Sub-20ms input to visual update latency
//! - Sub-50ms modifier key to grid display latency  
//! - <5% CPU during active use, <1% when idle
//! - <10MB resident memory footprint

pub mod config;
pub mod constants;
pub mod coordinate_resolver;
pub mod error;
pub mod grid_manager;
pub mod grids;
pub mod input_handler;
pub mod renderer;
pub mod types;
pub mod x11_client;

// Re-export commonly used types
pub use config::{Config, GridType, ModifierKey};
pub use error::PointlessError;
pub use types::{GridCell, GridCoord, KeyCode, Rectangle, ScreenCoord};
pub use x11_client::{ScreenInfo, X11Client};
