use crate::input::{
    keycode_to_letter, keycode_to_number, keycodes, letter_to_column, number_to_row,
};
use crate::{GridCell, GridCoord, GridLayout, KeyCode};
use pointless_x11::{Rectangle, ScreenInfo};

// Grid configuration constants with rationale
/// Default grid size of 4x4 provides good balance between precision and usability.
/// 4x4 = 16 cells can be navigated with A-D columns and 1-4 rows, fitting comfortably
/// within typical keyboard reach and visual processing limits.
const DEFAULT_GRID_SIZE: u32 = 4;

/// Maximum grid size of 6x6 prevents information overload while supporting up to A-F columns.
/// Larger grids become difficult to visually parse and require awkward key combinations.
const MAX_GRID_SIZE: u32 = 6;

/// Maximum subdivision levels of 3 provides pixel-level precision on most screens.
/// 3 levels of 4x4 subdivision = 64x64 effective resolution, which is sufficient
/// for precise cursor positioning without excessive nesting.
const MAX_SUBDIVISION_LEVELS: u32 = 3;

/// Maximum cells per grid prevents excessive memory usage and rendering overhead.
/// 100 cells accommodates even 10x10 grids while maintaining performance.
const MAX_CELLS_PER_GRID: usize = 100;

// Homogeneous grid implementation with two-key navigation and recursive subdivision
#[derive(Debug)]
pub struct HomogeneousGrid {
    cols: u32,
    rows: u32,
    selected_column: Option<u32>,
    current_region: Option<Rectangle>,
    subdivision_level: u32,
    max_subdivisions: u32,
    region_stack: Vec<Rectangle>,
}

impl Default for HomogeneousGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl HomogeneousGrid {
    pub fn new() -> Self {
        Self {
            cols: DEFAULT_GRID_SIZE,
            rows: DEFAULT_GRID_SIZE,
            selected_column: None,
            current_region: None,
            subdivision_level: 0,
            max_subdivisions: MAX_SUBDIVISION_LEVELS,
            region_stack: Vec::with_capacity(MAX_SUBDIVISION_LEVELS as usize),
        }
    }

    /// Validate internal state invariants for debugging
    #[cfg(debug_assertions)]
    fn validate_state_invariants(&self) {
        // Stack size should match subdivision level (except at root level)
        let expected_stack_size = if self.subdivision_level == 0 {
            0
        } else {
            self.subdivision_level
        };
        if self.region_stack.len() as u32 != expected_stack_size {
            eprintln!(
                "INVARIANT VIOLATION: subdivision_level={}, stack_len={}",
                self.subdivision_level,
                self.region_stack.len()
            );
        }

        // If at root level, current_region should be None
        if self.subdivision_level == 0 && self.current_region.is_some() {
            eprintln!("INVARIANT VIOLATION: At root level but current_region is Some");
        }
    }

    fn coord_to_label(&self, coord: GridCoord, buffer: &mut String) {
        // A-Z coordinate system: A1, A2, ..., A4, B1, B2, ..., D4
        buffer.clear();
        let col_char = (b'A' + (coord.x % 26) as u8) as char;
        let row_num = coord.y + 1;
        buffer.push(col_char);
        buffer.push_str(&row_num.to_string());
    }

    // All keycode mapping functions removed - now using shared input module

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

    fn process_key_with_screen(
        &mut self,
        key: KeyCode,
        _current_level: u32,
        screen: &ScreenInfo,
        density: u32,
    ) -> Option<GridCoord> {
        // First, try to interpret as a letter (column selection)
        if let Some(letter) = keycode_to_letter(key) {
            if let Some(column) = letter_to_column(letter, self.cols) {
                if self.selected_column.is_none() {
                    // First key: select column
                    self.selected_column = Some(column);
                    return None; // Don't move cursor yet, wait for row selection
                }
            }
        }

        // Next, try to interpret as a number (row selection)
        if let Some(number) = keycode_to_number(key) {
            if let Some(selected_col) = self.selected_column {
                if let Some(row) = number_to_row(number, self.rows) {
                    // Second key: select row, complete the coordinate
                    let coord = GridCoord {
                        x: selected_col,
                        y: row,
                    };

                    // If we can subdivide further, prepare for next level
                    if self.subdivision_level < self.max_subdivisions {
                        // Calculate the selected cell region for subdivision
                        let screen_region = self
                            .current_region
                            .unwrap_or_else(|| Rectangle::new(0, 0, screen.width, screen.height));

                        // Push current region onto stack for proper parent tracking
                        if self.region_stack.len() < MAX_SUBDIVISION_LEVELS as usize {
                            self.region_stack.push(screen_region);
                        } else {
                            eprintln!(
                                "ERROR: Region stack overflow at max subdivisions {}",
                                MAX_SUBDIVISION_LEVELS
                            );
                            return Some(coord); // Don't subdivide further
                        }

                        let grid_size = std::cmp::min(density, MAX_GRID_SIZE);
                        self.current_region = Some(self.calculate_subdivided_region(
                            &screen_region,
                            coord,
                            grid_size,
                            grid_size,
                        ));
                        self.subdivision_level += 1;
                    }

                    // Reset column selection for next input sequence
                    self.selected_column = None;

                    #[cfg(debug_assertions)]
                    self.validate_state_invariants();

                    return Some(coord);
                }
            }
        }

        // Handle escape key to go back up one subdivision level
        if key == keycodes::ESCAPE {
            if self.subdivision_level > 0 {
                self.subdivision_level -= 1;
                // Restore parent region from stack
                if self.subdivision_level == 0 {
                    self.current_region = None;
                    self.region_stack.clear(); // Clear stack when returning to root level
                } else {
                    // Pop the parent region from the stack
                    match self.region_stack.pop() {
                        Some(parent_region) => {
                            self.current_region = Some(parent_region);
                        }
                        None => {
                            // Critical invariant violation: subdivision_level > 0 but empty stack
                            eprintln!("ERROR: Region stack empty at subdivision level {}, resetting to root", 
                                     self.subdivision_level + 1);
                            self.subdivision_level = 0;
                            self.current_region = None;
                            self.region_stack.clear();
                        }
                    }
                }
            }
            self.selected_column = None;

            #[cfg(debug_assertions)]
            self.validate_state_invariants();

            return None;
        }

        None
    }
}

impl GridLayout for HomogeneousGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        let mut cells = Vec::new();
        let mut label_buffer = String::with_capacity(4); // Pre-allocate for performance

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

                // Reuse buffer for label generation
                self.coord_to_label(coord, &mut label_buffer);
                let label = label_buffer.clone();

                cells.push(GridCell::new(coord, rect, label));
            }
        }

        cells
    }

    fn process_key(
        &mut self,
        key: KeyCode,
        screen: &ScreenInfo,
        current_level: u32,
    ) -> Option<GridCoord> {
        self.process_key_with_screen(key, current_level, screen, DEFAULT_GRID_SIZE)
    }

    fn name(&self) -> &'static str {
        "Homogeneous Grid"
    }

    fn reset(&mut self) {
        self.selected_column = None;
        self.current_region = None;
        self.subdivision_level = 0;
        self.region_stack.clear();

        #[cfg(debug_assertions)]
        self.validate_state_invariants();
    }
}
