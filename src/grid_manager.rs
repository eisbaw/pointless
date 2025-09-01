use crate::config::GridType;
use crate::constants::*;
use crate::types::{GridCell, GridCoord, KeyCode, Rectangle, ScreenCoord};
use crate::x11_client::ScreenInfo;

pub trait GridLayout {
    /// Calculate grid cells for given screen dimensions
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell>;

    /// Process key input and return target cell coordinate
    fn process_key(&mut self, key: KeyCode, current_level: u32) -> Option<GridCoord>;

    /// Process key input with screen info available, return target cell coordinate
    fn process_key_with_screen(
        &mut self,
        key: KeyCode,
        current_level: u32,
        _screen: &ScreenInfo,
        _density: u32,
    ) -> Option<GridCoord> {
        // Default implementation delegates to process_key
        self.process_key(key, current_level)
    }

    /// Get human-readable name for this grid type
    fn name(&self) -> &'static str;

    /// Reset navigation state
    fn reset(&mut self);
}

pub struct GridManager {
    current_grid: Box<dyn GridLayout>,
    current_level: u32,
    current_cells: Vec<GridCell>,
    current_screen: Option<ScreenInfo>,
    current_density: u32,
    cached_cells: Vec<GridCell>, // Pre-allocated cache for performance
}

impl GridManager {
    pub fn new(grid_type: GridType) -> Self {
        let current_grid: Box<dyn GridLayout> = match grid_type {
            GridType::Homogeneous => Box::new(HomogeneousGrid::new()),
            _ => {
                // For now, default to homogeneous for other types
                Box::new(HomogeneousGrid::new())
            }
        };

        Self {
            current_grid,
            current_level: 0,
            current_cells: Vec::new(),
            current_screen: None,
            current_density: DEFAULT_GRID_SIZE,
            cached_cells: Vec::with_capacity(MAX_CELLS_PER_GRID),
        }
    }

    pub fn calculate_grid(&mut self, screen: &ScreenInfo, density: u32) -> &[GridCell] {
        // Fast path: reuse cached grid if screen and density haven't changed
        let needs_recalculation = match self.current_screen {
            Some(cached_screen) => {
                cached_screen.width != screen.width
                    || cached_screen.height != screen.height
                    || self.current_density != density
            }
            None => true,
        };

        if needs_recalculation {
            self.current_screen = Some(*screen);
            self.current_density = density;
            self.current_cells = self.current_grid.calculate_grid(screen, density);
        }

        &self.current_cells
    }

    pub fn process_key(&mut self, key: KeyCode) -> Option<ScreenCoord> {
        if let Some(screen) = &self.current_screen {
            if let Some(coord) = self.current_grid.process_key_with_screen(
                key,
                self.current_level,
                screen,
                self.current_density,
            ) {
                // Find the corresponding screen coordinate
                for cell in &self.current_cells {
                    if cell.coord == coord {
                        return Some(cell.center());
                    }
                }
            }
        }
        None
    }

    pub fn reset(&mut self) {
        self.current_grid.reset();
        self.current_level = 0;
        self.current_cells.clear();
    }

    pub fn current_cells(&self) -> &[GridCell] {
        &self.current_cells
    }
}

// Homogeneous grid implementation with two-key navigation and recursive subdivision
pub struct HomogeneousGrid {
    cols: u32,
    rows: u32,
    selected_column: Option<u32>,
    current_region: Option<Rectangle>,
    subdivision_level: u32,
    max_subdivisions: u32,
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
        }
    }

    fn coord_to_label(&self, coord: GridCoord, buffer: &mut String) {
        // A-Z coordinate system: A1, A2, ..., A4, B1, B2, ..., D4
        // Reuse buffer to avoid allocations
        buffer.clear();
        let col_char = (b'A' + (coord.x % 26) as u8) as char;
        let row_num = coord.y + 1;
        buffer.push(col_char);
        buffer.push_str(&row_num.to_string());
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

    fn process_key(&mut self, _key: KeyCode, _current_level: u32) -> Option<GridCoord> {
        // Fallback implementation - shouldn't be called directly
        None
    }

    fn process_key_with_screen(
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
                        // Calculate the selected cell region for subdivision
                        let screen_region = self
                            .current_region
                            .unwrap_or_else(|| Rectangle::new(0, 0, screen.width, screen.height));
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
                    return Some(coord);
                }
            }
        }

        // Handle escape key to go back up one subdivision level
        if key == KEYCODE_ESCAPE {
            // Escape key
            if self.subdivision_level > 0 {
                self.subdivision_level -= 1;
                // Reset current region to parent region
                // For now, just reset to full screen if going back to level 0
                if self.subdivision_level == 0 {
                    self.current_region = None;
                } else {
                    // TODO: Properly track parent regions in a stack
                    // For now, just reset to full screen
                    self.current_region = None;
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
    }
}
