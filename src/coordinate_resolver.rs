//! Coordinate transformation utilities
//!
//! This module provides efficient coordinate transformation between grid coordinates
//! and screen coordinates without requiring full grid recalculation.

use crate::types::{GridCoord, ScreenCoord};
use crate::x11_client::ScreenInfo;

/// Resolves coordinates between grid space and screen space efficiently
pub struct CoordinateResolver;

impl CoordinateResolver {
    /// Convert grid coordinate to screen coordinate for homogeneous grids
    #[must_use]
    pub fn grid_to_screen_homogeneous(
        coord: GridCoord,
        screen: &ScreenInfo,
        density: u32,
    ) -> ScreenCoord {
        let cols = density;
        let rows = density;

        let cell_width = screen.width / cols;
        let cell_height = screen.height / rows;

        let x = (coord.x * cell_width + cell_width / 2) as i32;
        let y = (coord.y * cell_height + cell_height / 2) as i32;

        log::debug!("Coordinate conversion: grid({}, {}) -> screen({}, {}) [screen={}x{}, density={}, cell={}x{}]", 
                   coord.x, coord.y, x, y, screen.width, screen.height, density, cell_width, cell_height);

        ScreenCoord { x, y }
    }

    /// Convert grid coordinate to screen coordinate for QWERTY grids
    #[must_use]
    pub fn grid_to_screen_qwerty(
        coord: GridCoord,
        screen: &ScreenInfo,
        _density: u32,
    ) -> ScreenCoord {
        // QWERTY layout has variable columns per row:
        // Row 0: Q-P (10 keys)
        // Row 1: A-L (9 keys)
        // Row 2: Z-M (7 keys)
        let row_key_counts = [10, 9, 7];
        let rows = 3;

        let row_height = screen.height / rows;

        // Get the number of keys in this row
        let row_keys = if coord.y < row_key_counts.len() as u32 {
            row_key_counts[coord.y as usize]
        } else {
            10 // fallback
        };

        let col_width = screen.width / row_keys;

        let x = (coord.x * col_width + col_width / 2) as i32;
        let y = (coord.y * row_height + row_height / 2) as i32;

        log::debug!("QWERTY coordinate conversion: grid({}, {}) -> screen({}, {}) [screen={}x{}, row_keys={}, cell={}x{}]", 
                   coord.x, coord.y, x, y, screen.width, screen.height, row_keys, col_width, row_height);

        ScreenCoord { x, y }
    }

    /// Convert grid coordinate to screen coordinate for arrow grids
    #[must_use]
    pub const fn grid_to_screen_arrow(
        coord: GridCoord,
        screen: &ScreenInfo,
        _density: u32,
    ) -> ScreenCoord {
        // Arrow grid is always 3x3
        let cols = 3;
        let rows = 3;

        let cell_width = screen.width / cols;
        let cell_height = screen.height / rows;

        let x = (coord.x * cell_width + cell_width / 2) as i32;
        let y = (coord.y * cell_height + cell_height / 2) as i32;

        ScreenCoord { x, y }
    }

    /// Convert grid coordinate directly to screen coordinate (for Vim grids that work in screen space)
    #[must_use]
    pub const fn grid_to_screen_direct(
        coord: GridCoord,
        _screen: &ScreenInfo,
        _density: u32,
    ) -> ScreenCoord {
        // Vim grids already work in screen coordinate space
        ScreenCoord {
            x: coord.x as i32,
            y: coord.y as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homogeneous_coordinate_resolution() {
        let screen = ScreenInfo {
            width: 1920,
            height: 1080,
            root_window: 0,
        };

        let coord = GridCoord { x: 0, y: 0 };
        let screen_coord = CoordinateResolver::grid_to_screen_homogeneous(coord, &screen, 4);

        // Should be center of top-left cell
        assert_eq!(screen_coord.x, 240); // (0 * 480 + 240)
        assert_eq!(screen_coord.y, 135); // (0 * 270 + 135)
    }

    #[test]
    fn test_qwerty_coordinate_resolution() {
        let screen = ScreenInfo {
            width: 1920,
            height: 1080,
            root_window: 0,
        };

        let coord = GridCoord { x: 1, y: 1 };
        let screen_coord = CoordinateResolver::grid_to_screen_qwerty(coord, &screen, 4);

        // Should be center of specified QWERTY cell
        // Row 1 has 9 keys (A-L), so col_width = 1920/9 = 213 (integer division)
        // Center of column 1 = 213 * 1 + 213/2 = 213 + 106 = 319
        assert_eq!(screen_coord.x, 319); // (1 * 213 + 106)
        assert_eq!(screen_coord.y, 540); // (1 * 360 + 180) for 3 rows
    }
}
