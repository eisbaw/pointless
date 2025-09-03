use super::GridLayout;
use crate::constants::*;
use crate::types::{GridCell, GridCoord, KeyCode, Rectangle, ScreenCoord};
use crate::x11_client::ScreenInfo;

/// Represents a single level in the grid hierarchy
#[derive(Debug, Clone)]
struct GridLevel {
    current_pos: GridCoord, // Position within this grid level
    region: Rectangle,      // Screen region this level covers
    cols: u32,              // Number of columns at this level
    rows: u32,              // Number of rows at this level
}

/// Arrow Key Grid provides configurable dimensions navigation using arrow keys
pub struct ArrowGrid {
    grid_stack: Vec<GridLevel>, // Stack of grid levels for recursion
    cols: u32,                  // Base number of columns
    rows: u32,                  // Base number of rows
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
            grid_stack: Vec::new(), // Start with empty stack - will be initialized on first use
            cols,
            rows,
        }
    }

    fn initialize_root_level(&mut self, screen: &ScreenInfo) {
        if self.grid_stack.is_empty() {
            let root_level = GridLevel {
                current_pos: GridCoord {
                    x: self.cols / 2, // Start at center column
                    y: self.rows / 2, // Start at center row
                },
                region: Rectangle::new(0, 0, screen.width, screen.height),
                cols: self.cols,
                rows: self.rows,
            };
            self.grid_stack.push(root_level);
        }
    }

    fn current_level(&self) -> Option<&GridLevel> {
        self.grid_stack.last()
    }

    fn current_level_mut(&mut self) -> Option<&mut GridLevel> {
        self.grid_stack.last_mut()
    }

    /// Migrate sub-grid to adjacent parent cell when hitting boundary
    fn migrate_subgrid_on_boundary(
        &mut self,
        direction: KeyCode,
        _screen: &ScreenInfo,
    ) -> Option<GridCoord> {
        // Only migrate if we have at least 2 levels (parent + sub-grid)
        if self.grid_stack.len() < 2 {
            log::debug!("Cannot migrate: not in sub-grid, staying at boundary");
            // Return current position when at root level boundary
            return self.current_level().map(|level| level.current_pos);
        }

        // Get parent level (second to last)
        let parent_idx = self.grid_stack.len() - 2;
        let mut parent_level = self.grid_stack[parent_idx].clone();

        // Try to move parent position in the given direction
        let mut can_migrate = false;
        match direction {
            KEYCODE_UP => {
                if parent_level.current_pos.y > 0 {
                    parent_level.current_pos.y -= 1;
                    can_migrate = true;
                }
            }
            KEYCODE_DOWN => {
                if parent_level.current_pos.y < (parent_level.rows - 1) {
                    parent_level.current_pos.y += 1;
                    can_migrate = true;
                }
            }
            KEYCODE_LEFT => {
                if parent_level.current_pos.x > 0 {
                    parent_level.current_pos.x -= 1;
                    can_migrate = true;
                }
            }
            KEYCODE_RIGHT => {
                if parent_level.current_pos.x < (parent_level.cols - 1) {
                    parent_level.current_pos.x += 1;
                    can_migrate = true;
                }
            }
            _ => {}
        }

        if !can_migrate {
            log::debug!("Cannot migrate: parent at boundary too, staying at current position");
            // Return current position when parent is at boundary too
            return self.current_level().map(|level| level.current_pos);
        }

        // Calculate new sub-grid region based on new parent position
        let parent_cell_width = parent_level.region.width / parent_level.cols;
        let parent_cell_height = parent_level.region.height / parent_level.rows;

        let new_sub_region = Rectangle::new(
            parent_level.region.x + (parent_level.current_pos.x * parent_cell_width) as i32,
            parent_level.region.y + (parent_level.current_pos.y * parent_cell_height) as i32,
            parent_cell_width,
            parent_cell_height,
        );

        // Update parent level in stack
        self.grid_stack[parent_idx] = parent_level;

        // Store parent position for logging
        let parent_pos = self.grid_stack[parent_idx].current_pos;

        // Update sub-grid region and position cursor at appropriate edge
        if let Some(current_level) = self.current_level_mut() {
            current_level.region = new_sub_region;

            // Position cursor at opposite edge from where we came
            match direction {
                KEYCODE_UP => {
                    // Came from bottom, go to bottom edge of new sub-grid
                    current_level.current_pos.y = current_level.rows - 1;
                }
                KEYCODE_DOWN => {
                    // Came from top, go to top edge of new sub-grid
                    current_level.current_pos.y = 0;
                }
                KEYCODE_LEFT => {
                    // Came from right, go to right edge of new sub-grid
                    current_level.current_pos.x = current_level.cols - 1;
                }
                KEYCODE_RIGHT => {
                    // Came from left, go to left edge of new sub-grid
                    current_level.current_pos.x = 0;
                }
                _ => {}
            }

            let cursor_pos = current_level.current_pos;

            log::debug!(
                "Sub-grid migrated {} to parent cell ({}, {}), cursor at ({}, {})",
                match direction {
                    KEYCODE_UP => "UP",
                    KEYCODE_DOWN => "DOWN", 
                    KEYCODE_LEFT => "LEFT",
                    KEYCODE_RIGHT => "RIGHT",
                    _ => "UNKNOWN",
                },
                parent_pos.x,
                parent_pos.y,
                cursor_pos.x,
                cursor_pos.y
            );

            Some(cursor_pos)
        } else {
            None
        }
    }

    fn calculate_hierarchical_cells(&self, _screen: &ScreenInfo) -> Vec<GridCell> {
        let mut cells = Vec::new();

        // Draw all levels in the hierarchy
        for (level_idx, level) in self.grid_stack.iter().enumerate() {
            let level_cells = self.calculate_cells_for_level(level, level_idx);
            cells.extend(level_cells);
        }

        cells
    }

    fn calculate_cells_for_level(&self, level: &GridLevel, level_idx: usize) -> Vec<GridCell> {
        let mut cells = Vec::new();
        let cell_width = level.region.width / level.cols;
        let cell_height = level.region.height / level.rows;

        for row in 0..level.rows {
            for col in 0..level.cols {
                let coord = GridCoord { x: col, y: row };
                let rect = Rectangle::new(
                    level.region.x + (col * cell_width) as i32,
                    level.region.y + (row * cell_height) as i32,
                    cell_width,
                    cell_height,
                );

                // Label cells with level and coordinates
                let label = if level_idx == 0 {
                    format!("{},{}", col, row)
                } else {
                    format!("L{}:{},{}", level_idx, col, row)
                };

                cells.push(GridCell::new(coord, rect, label));
            }
        }

        cells
    }
}

impl GridLayout for ArrowGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, _density: u32) -> Vec<GridCell> {
        // Initialize root level if empty (for initial rendering and testing)
        if self.grid_stack.is_empty() {
            // Create a temporary ArrowGrid with initialized root level
            let temp_grid = ArrowGrid {
                grid_stack: vec![GridLevel {
                    current_pos: GridCoord {
                        x: self.cols / 2,
                        y: self.rows / 2,
                    },
                    region: Rectangle::new(0, 0, screen.width, screen.height),
                    cols: self.cols,
                    rows: self.rows,
                }],
                cols: self.cols,
                rows: self.rows,
            };
            return temp_grid.calculate_hierarchical_cells(screen);
        }
        self.calculate_hierarchical_cells(screen)
    }

    fn process_key(
        &mut self,
        key: KeyCode,
        _current_level: u32,
        screen: &ScreenInfo,
        _density: u32,
    ) -> Option<GridCoord> {
        // Initialize root level if this is the first key press
        self.initialize_root_level(screen);

        match key {
            // Arrow keys for navigation within current level
            KEYCODE_UP => {
                if let Some(current) = self.current_level_mut() {
                    if current.current_pos.y > 0 {
                        current.current_pos.y -= 1;
                        Some(current.current_pos)
                    } else {
                        // Hit top boundary - migrate sub-grid up in parent level if possible
                        self.migrate_subgrid_on_boundary(KEYCODE_UP, screen)
                    }
                } else {
                    None
                }
            }
            KEYCODE_DOWN => {
                if let Some(current) = self.current_level_mut() {
                    if current.current_pos.y < (current.rows - 1) {
                        current.current_pos.y += 1;
                        Some(current.current_pos)
                    } else {
                        // Hit bottom boundary - migrate sub-grid down in parent level if possible
                        self.migrate_subgrid_on_boundary(KEYCODE_DOWN, screen)
                    }
                } else {
                    None
                }
            }
            KEYCODE_LEFT => {
                if let Some(current) = self.current_level_mut() {
                    if current.current_pos.x > 0 {
                        current.current_pos.x -= 1;
                        Some(current.current_pos)
                    } else {
                        // Hit left boundary - migrate sub-grid left in parent level if possible
                        self.migrate_subgrid_on_boundary(KEYCODE_LEFT, screen)
                    }
                } else {
                    None
                }
            }
            KEYCODE_RIGHT => {
                if let Some(current) = self.current_level_mut() {
                    if current.current_pos.x < (current.cols - 1) {
                        current.current_pos.x += 1;
                        Some(current.current_pos)
                    } else {
                        // Hit right boundary - migrate sub-grid right in parent level if possible
                        self.migrate_subgrid_on_boundary(KEYCODE_RIGHT, screen)
                    }
                } else {
                    None
                }
            }
            // PageUp: Recurse into the current cell (create sub-grid)
            KEYCODE_PAGE_UP => {
                if let Some(current) = self.current_level() {
                    let cell_width = current.region.width / current.cols;
                    let cell_height = current.region.height / current.rows;

                    // Calculate the region of the selected cell
                    let sub_region = Rectangle::new(
                        current.region.x + (current.current_pos.x * cell_width) as i32,
                        current.region.y + (current.current_pos.y * cell_height) as i32,
                        cell_width,
                        cell_height,
                    );

                    // Create new level with the same dimensions as base grid
                    let new_level = GridLevel {
                        current_pos: GridCoord {
                            x: self.cols / 2, // Start at center
                            y: self.rows / 2,
                        },
                        region: sub_region,
                        cols: self.cols,
                        rows: self.rows,
                    };

                    self.grid_stack.push(new_level);
                    log::debug!(
                        "PageUp: Created sub-grid at level {}",
                        self.grid_stack.len() - 1
                    );
                    Some(self.current_level().unwrap().current_pos)
                } else {
                    None
                }
            }
            // PageDown: Pop back to parent grid level
            KEYCODE_PAGE_DOWN => {
                if self.grid_stack.len() > 1 {
                    self.grid_stack.pop();
                    log::debug!("PageDown: Returned to level {}", self.grid_stack.len() - 1);
                    Some(self.current_level().unwrap().current_pos)
                } else {
                    log::debug!("PageDown: Already at root level");
                    // Stay at root level
                    Some(self.current_level().unwrap().current_pos)
                }
            }
            // Space or Enter key to select current position and move cursor
            KEYCODE_SPACE | KEYCODE_ENTER => self.current_level().map(|level| level.current_pos),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        "Arrow Key Grid"
    }

    fn reset(&mut self) {
        // Clear all levels except root, or clear entirely for fresh start
        self.grid_stack.clear();
    }

    fn grid_to_screen_coord(
        &self,
        coord: GridCoord,
        screen: &ScreenInfo,
        _density: u32,
    ) -> ScreenCoord {
        // Use the current (deepest) level if available, otherwise fallback to full screen
        if let Some(current_level) = self.current_level() {
            let cell_width = current_level.region.width / current_level.cols;
            let cell_height = current_level.region.height / current_level.rows;

            let x = current_level.region.x + (coord.x * cell_width + cell_width / 2) as i32;
            let y = current_level.region.y + (coord.y * cell_height + cell_height / 2) as i32;

            ScreenCoord { x, y }
        } else {
            // Fallback to full screen
            let cell_width = screen.width / self.cols;
            let cell_height = screen.height / self.rows;

            let x = (coord.x * cell_width + cell_width / 2) as i32;
            let y = (coord.y * cell_height + cell_height / 2) as i32;

            ScreenCoord { x, y }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
