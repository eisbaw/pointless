//! Grid layout implementations for Pointless X11 cursor utility
//!
//! This module provides pluggable grid layout systems for keyboard-driven cursor positioning.
//! Following MPED principles, each grid type is implemented as a composable unit that can be
//! independently tested and reasoned about.
//!
//! ## Grid Types
//!
//! ### Basic Grids (always available)
//! - **Homogeneous**: Uniform rectangular grid with A-Z coordinate naming and recursive subdivision
//! - **QWERTY**: Physical keyboard layout projected onto screen space for intuitive navigation
//! - **Arrow**: 3x3 directional grid with hierarchical navigation using arrow keys
//! - **Vim**: Vim-like movement commands (h/j/k/l) with jump targets and screen positioning
//!
//! ### Vision-Based Grids (optional, requires OpenCV)
//! - **Hough**: Line detection grid using Hough transform to find intersection points
//! - **Rectangle**: Morphological rectangle fitting for UI element detection
//! - **Edge**: Edge density adaptive grid for content-aware positioning  
//! - **Color**: Color clustering region detection
//! - **Feature**: SIFT/ORB feature detection grid
//!
//! ## Architecture
//!
//! All grids implement the `GridLayout` trait which provides:
//! - Grid cell calculation with screen-aware positioning
//! - Key processing with unified interface (screen info always available)
//! - Coordinate transformation between grid space and screen space
//! - State management with reset capabilities
//!
//! ## Design Principles
//!
//! - **Stateless Calculation**: Grid cells are computed on-demand, not cached
//! - **Fail-Fast**: Input validation happens immediately
//! - **Composable**: Each grid can be used independently or combined
//! - **Minimal State**: Only essential navigation state is maintained
//! - **Performance**: Sub-20ms latency with efficient coordinate transformations

pub mod arrow;
pub mod homogeneous;
pub mod qwerty;
pub mod vim;

pub use arrow::ArrowGrid;
pub use homogeneous::HomogeneousGrid;
pub use qwerty::QwertyGrid;
pub use vim::VimGrid;

use crate::types::{GridCell, GridCoord, KeyCode, ScreenCoord};
use crate::x11_client::ScreenInfo;
use std::any::Any;

pub trait GridLayout {
    /// Calculate grid cells for given screen dimensions
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell>;

    /// Process key input and return target cell coordinate
    /// All grids receive screen info and density for consistent interface
    fn process_key(
        &mut self,
        key: KeyCode,
        current_level: u32,
        screen: &ScreenInfo,
        density: u32,
    ) -> Option<GridCoord>;

    /// Get human-readable name for this grid type
    #[allow(dead_code)]
    fn name(&self) -> &'static str;

    /// Reset navigation state
    fn reset(&mut self);

    /// Convert grid coordinate to screen coordinate efficiently (without full grid calculation)
    fn grid_to_screen_coord(
        &self,
        coord: GridCoord,
        screen: &ScreenInfo,
        density: u32,
    ) -> ScreenCoord;

    /// Allow downcasting to specific grid types
    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    fn mock_screen() -> ScreenInfo {
        ScreenInfo {
            width: 1024,
            height: 768,
            root_window: 0,
        }
    }

    mod homogeneous_grid_tests {
        use super::*;

        #[test]
        fn test_homogeneous_grid_creation() {
            let grid = HomogeneousGrid::new();
            assert_eq!(grid.name(), "Homogeneous Grid");
        }

        #[test]
        fn test_homogeneous_grid_calculate_basic() {
            let grid = HomogeneousGrid::new();
            let screen = mock_screen();
            let cells = grid.calculate_grid(&screen, 4);

            // Should create a 4x4 grid = 16 cells
            assert_eq!(cells.len(), 16);

            // Check first cell
            let first_cell = &cells[0];
            assert_eq!(first_cell.coord.x, 0);
            assert_eq!(first_cell.coord.y, 0);
            assert_eq!(first_cell.label, "A1");
        }

        #[test]
        fn test_homogeneous_grid_cell_sizes() {
            let grid = HomogeneousGrid::new();
            let screen = mock_screen();
            let cells = grid.calculate_grid(&screen, 4);

            let expected_width = screen.width / 4;
            let expected_height = screen.height / 4;

            for cell in cells {
                assert_eq!(cell.screen_rect.width, expected_width);
                assert_eq!(cell.screen_rect.height, expected_height);
            }
        }

        #[test]
        fn test_homogeneous_grid_two_key_navigation() {
            let mut grid = HomogeneousGrid::new();
            let screen = mock_screen();

            // First key: select column A (should not return coordinate yet)
            let result1 = grid.process_key(KEYCODE_A, 0, &screen, 4);
            assert!(result1.is_none());

            // Second key: select row 1 (should return coordinate)
            let result2 = grid.process_key(KEYCODE_1, 0, &screen, 4);
            assert!(
                result2.is_some(),
                "Expected coordinate after valid navigation"
            );
            let coord = result2.expect("Expected coordinate after valid navigation");
            assert_eq!(coord.x, 0); // Column A = 0
            assert_eq!(coord.y, 0); // Row 1 = 0
        }

        #[test]
        fn test_homogeneous_grid_reset() {
            let mut grid = HomogeneousGrid::new();
            let screen = mock_screen();

            // Select a column
            grid.process_key(KEYCODE_A, 0, &screen, 4);

            // Reset
            grid.reset();

            // Should be able to select the same column again
            let result = grid.process_key(KEYCODE_A, 0, &screen, 4);
            assert!(result.is_none()); // Still waiting for row selection
        }
    }

    mod qwerty_grid_tests {
        use super::*;

        #[test]
        fn test_qwerty_grid_creation() {
            let grid = QwertyGrid::new();
            assert_eq!(grid.name(), "QWERTY Grid");
        }

        #[test]
        fn test_qwerty_grid_calculate() {
            let grid = QwertyGrid::new();
            let screen = mock_screen();
            let cells = grid.calculate_grid(&screen, 4);

            // Should create cells for QWERTY layout
            // Top row (10) + Home row (9) + Bottom row (7) = 26 cells
            assert_eq!(cells.len(), 26);
        }

        #[test]
        fn test_qwerty_grid_key_mapping() {
            let mut grid = QwertyGrid::new();
            let screen = mock_screen();

            // Press Q (should map to top-left region)
            let result = grid.process_key(KEYCODE_Q, 0, &screen, 4);
            assert!(
                result.is_some(),
                "Expected coordinate after QWERTY key press"
            );
            let coord = result.expect("Expected coordinate after QWERTY key press");
            assert_eq!(coord.x, 0); // First column
            assert_eq!(coord.y, 0); // First row
        }

        #[test]
        fn test_qwerty_grid_escape_reset() {
            let mut grid = QwertyGrid::new();
            let screen = mock_screen();

            // Press a key to start refinement
            grid.process_key(KEYCODE_Q, 0, &screen, 4);

            // Press escape to reset
            let result = grid.process_key(KEYCODE_ESCAPE, 0, &screen, 4);
            assert!(result.is_none());
        }
    }

    mod arrow_grid_tests {
        use super::*;

        #[test]
        fn test_arrow_grid_creation() {
            let grid = ArrowGrid::new();
            assert_eq!(grid.name(), "Arrow Key Grid");
        }

        #[test]
        fn test_arrow_grid_calculate() {
            let grid = ArrowGrid::new();
            let screen = mock_screen();
            let cells = grid.calculate_grid(&screen, 4);

            // Should create a 3x3 grid = 9 cells
            assert_eq!(cells.len(), 9);
        }

        #[test]
        fn test_arrow_grid_navigation() {
            let mut grid = ArrowGrid::new();
            let screen = mock_screen();

            // Initial position should be center (1,1)
            let cells = grid.calculate_grid(&screen, 4);
            assert_eq!(cells.len(), 9);

            // Move up
            let result = grid.process_key(KEYCODE_UP, 0, &screen, 4);
            assert!(
                result.is_some(),
                "Expected coordinate after valid navigation"
            );
            let coord = result.expect("Expected coordinate after valid navigation");
            assert_eq!(coord.y, 0); // Moved from 1 to 0

            // Move left
            let result = grid.process_key(KEYCODE_LEFT, 0, &screen, 4);
            assert!(
                result.is_some(),
                "Expected coordinate after valid navigation"
            );
            let coord = result.expect("Expected coordinate after valid navigation");
            assert_eq!(coord.x, 0); // Moved from 1 to 0
        }

        #[test]
        fn test_arrow_grid_bounds() {
            let mut grid = ArrowGrid::new();
            let screen = mock_screen();

            // Move to top-left corner
            grid.process_key(KEYCODE_UP, 0, &screen, 4);
            grid.process_key(KEYCODE_LEFT, 0, &screen, 4);

            // Try to move further up (should stay at boundary)
            let result = grid.process_key(KEYCODE_UP, 0, &screen, 4);
            assert!(
                result.is_some(),
                "Expected coordinate after valid navigation"
            );
            let coord = result.expect("Expected coordinate after valid navigation");
            assert_eq!(coord.y, 0); // Should stay at 0
        }
    }

    mod vim_grid_tests {
        use super::*;

        #[test]
        fn test_vim_grid_creation() {
            let grid = VimGrid::new();
            assert_eq!(grid.name(), "Vim Grid");
        }

        #[test]
        fn test_vim_grid_basic_movement() {
            let mut grid = VimGrid::new();
            let screen = mock_screen();

            // Test h/j/k/l movement
            let result_h = grid.process_key(KEYCODE_H, 0, &screen, 4);
            assert!(result_h.is_some());

            let result_j = grid.process_key(KEYCODE_J, 0, &screen, 4);
            assert!(result_j.is_some());

            let result_k = grid.process_key(KEYCODE_K, 0, &screen, 4);
            assert!(result_k.is_some());

            let result_l = grid.process_key(KEYCODE_L, 0, &screen, 4);
            assert!(result_l.is_some());
        }

        #[test]
        fn test_vim_grid_jump_modes() {
            let mut grid = VimGrid::new();
            let screen = mock_screen();

            // Enter forward jump mode with 'w'
            let result = grid.process_key(KEYCODE_W, 0, &screen, 4);
            assert!(result.is_none()); // Should enter mode but not move cursor yet

            // Calculate grid should now show jump targets
            let cells = grid.calculate_grid(&screen, 4);
            assert!(!cells.is_empty()); // Should have jump targets
        }

        #[test]
        fn test_vim_grid_screen_positioning() {
            let mut grid = VimGrid::new();
            let screen = mock_screen();

            // Test jump to left edge with '0'
            let result = grid.process_key(KEYCODE_0, 0, &screen, 4);
            assert!(
                result.is_some(),
                "Expected coordinate after valid navigation"
            );
            let coord = result.expect("Expected coordinate after valid navigation");
            assert_eq!(coord.x, 0); // Should be at left edge
        }

        #[test]
        fn test_vim_grid_reset() {
            let mut grid = VimGrid::new();
            let screen = mock_screen();

            // Enter jump mode
            grid.process_key(KEYCODE_W, 0, &screen, 4);

            // Reset
            grid.reset();

            // Should be back in normal mode
            let cells = grid.calculate_grid(&screen, 4);
            assert_eq!(cells.len(), 1); // Should show only cursor position
        }
    }
}
