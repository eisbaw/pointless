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

        #[test]
        fn test_arrow_grid_recursive_navigation() {
            let mut grid = ArrowGrid::new();
            let screen = mock_screen();

            // Initial grid should have 9 cells (3x3)
            let cells = grid.calculate_grid(&screen, 4);
            assert_eq!(cells.len(), 9);

            // Move to a specific position
            grid.process_key(KEYCODE_RIGHT, 0, &screen, 4); // Move right from center
            grid.process_key(KEYCODE_UP, 0, &screen, 4); // Move up

            // Press PageUp to recurse into current cell
            let result = grid.process_key(KEYCODE_PAGE_UP, 0, &screen, 4);
            assert!(result.is_some(), "PageUp should return a coordinate");

            // Now we should have cells from both levels (parent + child)
            let cells = grid.calculate_grid(&screen, 4);
            assert_eq!(cells.len(), 18); // 9 parent cells + 9 child cells

            // Check labels to ensure we have both levels
            let labels: Vec<&str> = cells.iter().map(|cell| cell.label.as_str()).collect();
            assert!(
                labels.iter().any(|&label| label.starts_with("L1:")),
                "Should have level 1 labels"
            );
            assert!(
                labels.iter().any(|&label| !label.starts_with("L")),
                "Should have root level labels"
            );

            // Navigate in the sub-grid
            grid.process_key(KEYCODE_LEFT, 0, &screen, 4);
            grid.process_key(KEYCODE_DOWN, 0, &screen, 4);

            // PageDown to go back to parent level
            let result = grid.process_key(KEYCODE_PAGE_DOWN, 0, &screen, 4);
            assert!(result.is_some(), "PageDown should return a coordinate");

            // Should be back to just the root level
            let cells = grid.calculate_grid(&screen, 4);
            assert_eq!(cells.len(), 9); // Just root level cells

            // Try PageDown again - should stay at root level
            let result = grid.process_key(KEYCODE_PAGE_DOWN, 0, &screen, 4);
            assert!(
                result.is_some(),
                "PageDown at root should still return coordinate"
            );

            let cells = grid.calculate_grid(&screen, 4);
            assert_eq!(cells.len(), 9); // Still just root level cells
        }

        #[test]
        fn test_arrow_grid_multi_level_recursion() {
            let mut grid = ArrowGrid::new();
            let screen = mock_screen();

            // Go to level 1
            grid.process_key(KEYCODE_PAGE_UP, 0, &screen, 4);
            let cells_level1 = grid.calculate_grid(&screen, 4);
            assert_eq!(cells_level1.len(), 18); // Root + Level 1

            // Go to level 2
            grid.process_key(KEYCODE_PAGE_UP, 0, &screen, 4);
            let cells_level2 = grid.calculate_grid(&screen, 4);
            assert_eq!(cells_level2.len(), 27); // Root + Level 1 + Level 2

            // Go back to level 1
            grid.process_key(KEYCODE_PAGE_DOWN, 0, &screen, 4);
            let cells_back_to_level1 = grid.calculate_grid(&screen, 4);
            assert_eq!(cells_back_to_level1.len(), 18); // Root + Level 1

            // Go back to root
            grid.process_key(KEYCODE_PAGE_DOWN, 0, &screen, 4);
            let cells_root = grid.calculate_grid(&screen, 4);
            assert_eq!(cells_root.len(), 9); // Just root
        }

        #[test]
        fn test_arrow_grid_cursor_positioning_in_subgrids() {
            use crate::grid_manager::GridManager;
            use crate::config::{Config, GridType, ModifierKey, MoveStyle};
            
            let config = Config {
                grid_type: GridType::Arrow,
                modifier_key: ModifierKey::Alt,
                grid_density: 3,
                color: "FFFF00".to_string(),
                opacity: 0.8,
                debug: false,
                png_output: None,
                png_resolution: "1920x1080".to_string(),
                arrow: "3x3".to_string(),
                always_show: false,
                movestyle: MoveStyle::None,
            };

            let mut grid_manager = GridManager::new(&config).unwrap();
            let screen = mock_screen();

            // Get initial grid (root level)
            let root_cells = grid_manager.calculate_grid(&screen, 3);
            assert_eq!(root_cells.len(), 9);

            // Navigate within root grid to position (2,1) - right edge, middle row
            grid_manager.process_key_with_cells(KEYCODE_RIGHT, &root_cells, &screen);
            
            // Create sub-grid with PageUp at current position
            grid_manager.process_key_with_cells(KEYCODE_PAGE_UP, &root_cells, &screen);
            
            // Get new cells with hierarchy
            let sub_cells = grid_manager.calculate_grid(&screen, 3);
            assert_eq!(sub_cells.len(), 18); // Root + Sub-grid

            // Find root cells vs sub-grid cells by looking at their screen positions
            let mut root_level_cells = Vec::new();
            let mut sub_level_cells = Vec::new();
            
            for cell in &sub_cells {
                if cell.label.starts_with('L') {
                    sub_level_cells.push(cell);
                } else {
                    root_level_cells.push(cell);
                }
            }
            
            assert_eq!(root_level_cells.len(), 9, "Should have 9 root level cells");
            assert_eq!(sub_level_cells.len(), 9, "Should have 9 sub-level cells");

            // Sub-grid cells should be smaller and positioned within one of the root cells
            let root_cell_area = root_level_cells[0].screen_rect.width * root_level_cells[0].screen_rect.height;
            let sub_cell_area = sub_level_cells[0].screen_rect.width * sub_level_cells[0].screen_rect.height;
            
            assert!(sub_cell_area < root_cell_area, "Sub-grid cells should be smaller than root cells");

            // Navigate within sub-grid - cursor should move to sub-grid cell positions
            let pos_in_sub = grid_manager.process_key_with_cells(KEYCODE_LEFT, &sub_cells, &screen);
            assert!(pos_in_sub.is_some(), "Should be able to navigate within sub-grid");

            if let Some(sub_pos) = pos_in_sub {
                // Verify the cursor is positioned within the sub-grid area
                let sub_grid_bounds = sub_level_cells.iter().fold(
                    (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
                    |(min_x, min_y, max_x, max_y), cell| {
                        (
                            min_x.min(cell.screen_rect.x),
                            min_y.min(cell.screen_rect.y),
                            max_x.max(cell.screen_rect.x + cell.screen_rect.width as i32),
                            max_y.max(cell.screen_rect.y + cell.screen_rect.height as i32),
                        )
                    }
                );
                
                assert!(
                    sub_pos.x >= sub_grid_bounds.0 && sub_pos.x <= sub_grid_bounds.2,
                    "Cursor X should be within sub-grid bounds"
                );
                assert!(
                    sub_pos.y >= sub_grid_bounds.1 && sub_pos.y <= sub_grid_bounds.3,
                    "Cursor Y should be within sub-grid bounds"
                );
            }
        }
    }
}
