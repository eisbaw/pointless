use crate::config::GridType;
use crate::types::{GridCell, GridCoord, KeyCode, Rectangle, ScreenCoord};
use crate::x11_client::ScreenInfo;

pub trait GridLayout {
    /// Calculate grid cells for given screen dimensions
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell>;

    /// Process key input and return target cell coordinate
    fn process_key(&mut self, key: KeyCode, current_level: u32) -> Option<GridCoord>;

    /// Get human-readable name for this grid type
    fn name(&self) -> &'static str;

    /// Reset navigation state
    fn reset(&mut self);
}

pub struct GridManager {
    current_grid: Box<dyn GridLayout>,
    current_level: u32,
    current_cells: Vec<GridCell>,
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
        }
    }

    pub fn calculate_grid(&mut self, screen: &ScreenInfo, density: u32) -> &[GridCell] {
        self.current_cells = self.current_grid.calculate_grid(screen, density);
        &self.current_cells
    }

    pub fn process_key(&mut self, key: KeyCode) -> Option<ScreenCoord> {
        if let Some(coord) = self.current_grid.process_key(key, self.current_level) {
            // Find the corresponding screen coordinate
            for cell in &self.current_cells {
                if cell.coord == coord {
                    return Some(cell.center());
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

// Simple homogeneous grid implementation
pub struct HomogeneousGrid {
    cols: u32,
    rows: u32,
    selected_coord: Option<GridCoord>,
}

impl HomogeneousGrid {
    pub fn new() -> Self {
        Self {
            cols: 4, // Default 4x4 grid
            rows: 4,
            selected_coord: None,
        }
    }

    fn coord_to_label(&self, coord: GridCoord) -> String {
        // Simple labeling: A1, A2, A3, A4, B1, B2, etc.
        let col_char = (b'A' + (coord.x % 26) as u8) as char;
        let row_num = coord.y + 1;
        format!("{}{}", col_char, row_num)
    }

    fn key_to_coord(&self, key: KeyCode) -> Option<GridCoord> {
        // Map common keys to grid coordinates
        // For now, use a simple mapping for testing
        match key {
            // Top row: 1,2,3,4
            10 => Some(GridCoord { x: 0, y: 0 }), // Key '1'
            11 => Some(GridCoord { x: 1, y: 0 }), // Key '2'
            12 => Some(GridCoord { x: 2, y: 0 }), // Key '3'
            13 => Some(GridCoord { x: 3, y: 0 }), // Key '4'
            // Second row: Q,W,E,R
            24 => Some(GridCoord { x: 0, y: 1 }), // Key 'Q'
            25 => Some(GridCoord { x: 1, y: 1 }), // Key 'W'
            26 => Some(GridCoord { x: 2, y: 1 }), // Key 'E'
            27 => Some(GridCoord { x: 3, y: 1 }), // Key 'R'
            // Third row: A,S,D,F
            38 => Some(GridCoord { x: 0, y: 2 }), // Key 'A'
            39 => Some(GridCoord { x: 1, y: 2 }), // Key 'S'
            40 => Some(GridCoord { x: 2, y: 2 }), // Key 'D'
            41 => Some(GridCoord { x: 3, y: 2 }), // Key 'F'
            // Fourth row: Z,X,C,V
            52 => Some(GridCoord { x: 0, y: 3 }), // Key 'Z'
            53 => Some(GridCoord { x: 1, y: 3 }), // Key 'X'
            54 => Some(GridCoord { x: 2, y: 3 }), // Key 'C'
            55 => Some(GridCoord { x: 3, y: 3 }), // Key 'V'
            _ => None,
        }
    }
}

impl GridLayout for HomogeneousGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, _density: u32) -> Vec<GridCell> {
        let mut cells = Vec::new();

        let cell_width = screen.width / self.cols;
        let cell_height = screen.height / self.rows;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let coord = GridCoord { x: col, y: row };
                let rect = Rectangle::new(
                    (col * cell_width) as i32,
                    (row * cell_height) as i32,
                    cell_width,
                    cell_height,
                );
                let label = self.coord_to_label(coord);

                cells.push(GridCell::new(coord, rect, label));
            }
        }

        cells
    }

    fn process_key(&mut self, key: KeyCode, _current_level: u32) -> Option<GridCoord> {
        self.key_to_coord(key)
    }

    fn name(&self) -> &'static str {
        "Homogeneous Grid"
    }

    fn reset(&mut self) {
        self.selected_coord = None;
    }
}
