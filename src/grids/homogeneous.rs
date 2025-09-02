use super::GridLayout;
use crate::constants::*;
use crate::types::{GridCell, GridCoord, KeyCode, Rectangle, ScreenCoord};
use crate::x11_client::ScreenInfo;

// Homogeneous grid implementation with two-key navigation and recursive subdivision
pub struct HomogeneousGrid {
    cols: u32,
    rows: u32,
    selected_column: Option<u32>,
    current_region: Option<Rectangle>,
    subdivision_level: u32,
    max_subdivisions: u32,
    // Stack-based region tracking for proper hierarchical navigation
    region_stack: Vec<Rectangle>,
}

impl Default for HomogeneousGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl HomogeneousGrid {
    #[must_use]
    pub fn new() -> Self {
        Self {
            cols: DEFAULT_GRID_SIZE,
            rows: DEFAULT_GRID_SIZE,
            selected_column: None,
            current_region: None,
            subdivision_level: 0,
            max_subdivisions: MAX_SUBDIVISION_LEVELS,
            region_stack: Vec::new(),
        }
    }

    /// Create label string directly with pre-allocated capacity to avoid buffer operations
    fn create_label(&self, coord: GridCoord) -> String {
        let col_char = (b'A' + (coord.x % 26) as u8) as char;
        let row_num = coord.y + 1;
        // Pre-allocate with expected capacity (1 char + up to 2 digits)
        let mut label = String::with_capacity(3);
        label.push(col_char);
        use std::fmt::Write;
        write!(label, "{}", row_num).expect("Writing to String cannot fail");
        label
    }

    fn keycode_to_letter(&self, keycode: KeyCode) -> Option<char> {
        // Map X11 keycodes to letters A-Z using constants
        match keycode {
            KEYCODE_A => Some('A'),
            KEYCODE_B => Some('B'),
            KEYCODE_C => Some('C'),
            KEYCODE_D => Some('D'),
            KEYCODE_E => Some('E'),
            KEYCODE_F => Some('F'),
            KEYCODE_G => Some('G'),
            KEYCODE_H => Some('H'),
            KEYCODE_I => Some('I'),
            KEYCODE_J => Some('J'),
            KEYCODE_K => Some('K'),
            KEYCODE_L => Some('L'),
            KEYCODE_M => Some('M'),
            KEYCODE_N => Some('N'),
            KEYCODE_O => Some('O'),
            KEYCODE_P => Some('P'),
            KEYCODE_Q => Some('Q'),
            KEYCODE_R => Some('R'),
            KEYCODE_S => Some('S'),
            KEYCODE_T => Some('T'),
            KEYCODE_U => Some('U'),
            KEYCODE_V => Some('V'),
            KEYCODE_W => Some('W'),
            KEYCODE_X => Some('X'),
            KEYCODE_Y => Some('Y'),
            KEYCODE_Z => Some('Z'),
            _ => None,
        }
    }

    fn keycode_to_number(&self, keycode: KeyCode) -> Option<u32> {
        // Map X11 keycodes to numbers 1-9,0 using constants
        match keycode {
            KEYCODE_1 => Some(1),
            KEYCODE_2 => Some(2),
            KEYCODE_3 => Some(3),
            KEYCODE_4 => Some(4),
            KEYCODE_5 => Some(5),
            KEYCODE_6 => Some(6),
            KEYCODE_7 => Some(7),
            KEYCODE_8 => Some(8),
            KEYCODE_9 => Some(9),
            KEYCODE_0 => Some(0), // 0 (treated as 10)
            _ => None,
        }
    }

    fn letter_to_column(&self, letter: char) -> Option<u32> {
        let col_index = (letter as u32).saturating_sub('A' as u32);
        if col_index < self.cols {
            Some(col_index)
        } else {
            None
        }
    }

    fn number_to_row(&self, number: u32) -> Option<u32> {
        let row_index = if number == 0 { 9 } else { number - 1 }; // 0 maps to row 10, 1-9 map to rows 1-9
        if row_index < self.rows {
            Some(row_index)
        } else {
            None
        }
    }

    fn calculate_subdivided_region(
        &self,
        parent_region: &Rectangle,
        coord: GridCoord,
        cols: u32,
        rows: u32,
    ) -> Rectangle {
        let cell_width = parent_region.width / cols;
        let cell_height = parent_region.height / rows;

        Rectangle::new(
            parent_region.x + (coord.x * cell_width) as i32,
            parent_region.y + (coord.y * cell_height) as i32,
            cell_width,
            cell_height,
        )
    }
}

impl GridLayout for HomogeneousGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        let mut cells = Vec::new();

        // Determine the region to subdivide - either full screen or current subdivision
        let region = self
            .current_region
            .unwrap_or_else(|| Rectangle::new(0, 0, screen.width, screen.height));

        // Adjust grid size based on density if at top level
        let (cols, rows) = if self.subdivision_level == 0 {
            let grid_size = std::cmp::min(density, MAX_GRID_SIZE);
            (grid_size, grid_size)
        } else {
            (self.cols, self.rows)
        };

        let cell_width = region.width / cols;
        let cell_height = region.height / rows;

        // Pre-allocate cells vector for performance
        cells.reserve(((cols * rows) as usize).min(MAX_CELLS_PER_GRID));

        for row in 0..rows {
            for col in 0..cols {
                let coord = GridCoord { x: col, y: row };
                let rect = Rectangle::new(
                    region.x + (col * cell_width) as i32,
                    region.y + (row * cell_height) as i32,
                    cell_width,
                    cell_height,
                );

                // Generate label efficiently with pre-allocated capacity
                let label = self.create_label(coord);

                cells.push(GridCell::new(coord, rect, label));
            }
        }

        cells
    }

    fn process_key(
        &mut self,
        key: KeyCode,
        _current_level: u32,
        screen: &ScreenInfo,
        density: u32,
    ) -> Option<GridCoord> {
        // First, try to interpret as a letter (column selection)
        if let Some(letter) = self.keycode_to_letter(key) {
            if let Some(column) = self.letter_to_column(letter) {
                if self.selected_column.is_none() {
                    // First key: select column
                    self.selected_column = Some(column);
                    return None; // Don't move cursor yet, wait for row selection
                }
            }
        }

        // Next, try to interpret as a number (row selection)
        if let Some(number) = self.keycode_to_number(key) {
            if let Some(selected_col) = self.selected_column {
                if let Some(row) = self.number_to_row(number) {
                    // Second key: select row, complete the coordinate
                    let coord = GridCoord {
                        x: selected_col,
                        y: row,
                    };

                    // If we can subdivide further, prepare for next level
                    if self.subdivision_level < self.max_subdivisions {
                        // Push current region onto stack before subdividing
                        let current_region = self
                            .current_region
                            .unwrap_or_else(|| Rectangle::new(0, 0, screen.width, screen.height));
                        self.region_stack.push(current_region);

                        // Calculate the selected cell region for subdivision
                        let grid_size = std::cmp::min(density, MAX_GRID_SIZE);
                        self.current_region = Some(self.calculate_subdivided_region(
                            &current_region,
                            coord,
                            grid_size,
                            grid_size,
                        ));
                        self.subdivision_level += 1;
                    }

                    // Reset column selection for next input sequence
                    self.selected_column = None;
                    return Some(coord);
                }
            }
        }

        // Handle escape key to go back up one subdivision level
        if key == KEYCODE_ESCAPE {
            // Escape key
            if self.subdivision_level > 0 {
                self.subdivision_level -= 1;

                // Pop parent region from stack to properly restore hierarchical navigation
                if self.subdivision_level == 0 {
                    // Back to full screen level
                    self.current_region = None;
                    self.region_stack.clear(); // Clear any remaining stack entries
                } else {
                    // Restore parent region from stack
                    self.current_region = self.region_stack.pop();
                }
            }
            self.selected_column = None;
            return None;
        }

        None
    }

    fn name(&self) -> &'static str {
        "Homogeneous Grid"
    }

    fn reset(&mut self) {
        self.selected_column = None;
        self.current_region = None;
        self.subdivision_level = 0;
        self.region_stack.clear();
    }

    fn grid_to_screen_coord(
        &self,
        coord: GridCoord,
        screen: &ScreenInfo,
        density: u32,
    ) -> ScreenCoord {
        use crate::coordinate_resolver::CoordinateResolver;

        // Always use absolute screen coordinates for cursor positioning
        // Subdivision regions are only for display purposes, not cursor movement
        log::debug!("Using absolute screen coordinate calculation");
        CoordinateResolver::grid_to_screen_homogeneous(coord, screen, density)
    }
}
