use super::GridLayout;
use crate::constants::*;
use crate::types::{GridCell, GridCoord, KeyCode, Rectangle, ScreenCoord};
use crate::x11_client::ScreenInfo;

/// Arrow Key Grid provides configurable dimensions navigation using arrow keys
pub struct ArrowGrid {
    current_pos: GridCoord, // Position within current grid
    cols: u32,              // Number of columns
    rows: u32,              // Number of rows
}

impl Default for ArrowGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrowGrid {
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_dimensions(3, 3) // Default 3x3 grid
    }

    #[must_use]
    pub fn new_with_dimensions(cols: u32, rows: u32) -> Self {
        Self {
            current_pos: GridCoord {
                x: cols / 2, // Start at center column
                y: rows / 2, // Start at center row
            },
            cols,
            rows,
        }
    }

    fn calculate_fullscreen_cells(&self, screen: &ScreenInfo) -> Vec<GridCell> {
        let mut cells = Vec::new();
        let cell_width = screen.width / self.cols;
        let cell_height = screen.height / self.rows;

        // Create grid spanning the entire screen
        for row in 0..self.rows {
            for col in 0..self.cols {
                let coord = GridCoord { x: col, y: row };
                let rect = Rectangle::new(
                    (col * cell_width) as i32,
                    (row * cell_height) as i32,
                    cell_width,
                    cell_height,
                );

                // Label cells with coordinates
                let label = format!("{},{}", col, row);

                cells.push(GridCell::new(coord, rect, label));
            }
        }

        cells
    }
}

impl GridLayout for ArrowGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, _density: u32) -> Vec<GridCell> {
        self.calculate_fullscreen_cells(screen)
    }

    fn process_key(
        &mut self,
        key: KeyCode,
        _current_level: u32,
        _screen: &ScreenInfo,
        _density: u32,
    ) -> Option<GridCoord> {
        match key {
            // Arrow keys for navigation
            KEYCODE_UP => {
                if self.current_pos.y > 0 {
                    self.current_pos.y -= 1;
                }
                Some(self.current_pos)
            }
            KEYCODE_DOWN => {
                if self.current_pos.y < (self.rows - 1) {
                    self.current_pos.y += 1;
                }
                Some(self.current_pos)
            }
            KEYCODE_LEFT => {
                if self.current_pos.x > 0 {
                    self.current_pos.x -= 1;
                }
                Some(self.current_pos)
            }
            KEYCODE_RIGHT => {
                if self.current_pos.x < (self.cols - 1) {
                    self.current_pos.x += 1;
                }
                Some(self.current_pos)
            }
            // Space or Enter key to select current position and move cursor
            KEYCODE_SPACE | KEYCODE_ENTER => Some(self.current_pos),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        "Arrow Key Grid"
    }

    fn reset(&mut self) {
        // Reset to center position
        self.current_pos = GridCoord {
            x: self.cols / 2,
            y: self.rows / 2,
        };
    }

    fn grid_to_screen_coord(
        &self,
        coord: GridCoord,
        screen: &ScreenInfo,
        _density: u32,
    ) -> ScreenCoord {
        // Calculate position in full-screen grid
        let cell_width = screen.width / self.cols;
        let cell_height = screen.height / self.rows;

        let x = (coord.x * cell_width + cell_width / 2) as i32;
        let y = (coord.y * cell_height + cell_height / 2) as i32;

        ScreenCoord { x, y }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
