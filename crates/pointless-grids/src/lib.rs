pub mod constants;
pub mod input;
pub mod layout;
pub mod types;

// Basic grid implementations
pub mod arrow;
pub mod homogeneous;
pub mod qwerty;
pub mod vim;

// Vision-based grids (optional)
#[cfg(feature = "vision-grids")]
pub mod vision;

// Re-exports
pub use layout::GridLayout;
pub use types::{GridCell, GridCoord, KeyCode};

// Grid implementations
pub use arrow::ArrowGrid;
pub use homogeneous::HomogeneousGrid;
pub use qwerty::QwertyGrid;
pub use vim::VimGrid;

#[cfg(feature = "vision-grids")]
pub use vision::{ColorGrid, EdgeGrid, FeatureGrid, HoughGrid, RectangleGrid};
