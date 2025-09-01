// Library interface for Pointless
// This allows external crates and tests to use the modules

pub mod config;
pub mod constants;
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
