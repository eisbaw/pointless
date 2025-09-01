use crate::{GridLayout, GridCell, GridCoord, KeyCode};
use pointless_x11::{Rectangle, ScreenCoord, ScreenInfo};

// X11 Keycode mappings
const KEYCODE_UP: u32 = 111;
const KEYCODE_DOWN: u32 = 116;
const KEYCODE_LEFT: u32 = 113;
const KEYCODE_RIGHT: u32 = 114;
const KEYCODE_ENTER: u32 = 36;
const KEYCODE_ESCAPE: u32 = 9;
const KEYCODE_SPACE: u32 = 65;

#[derive(Debug, Clone)]
struct GridLevel {
    center: ScreenCoord,
    size: u32,
}

/// Arrow Key Grid provides 3x3 navigation using arrow keys
#[derive(Debug)]
pub struct ArrowGrid {
    grid_stack: Vec<GridLevel>,
    current_pos: GridCoord, // Position within current 3x3 grid (0,0 to 2,2)
    max_levels: u32,
}

impl Default for ArrowGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrowGrid {
    pub fn new() -> Self {
        Self {
            grid_stack: Vec::new(),
            current_pos: GridCoord { x: 1, y: 1 }, // Start at center
            max_levels: 5,
        }
    }

    fn calculate_3x3_cells(&self, center: ScreenCoord, size: u32) -> Vec<GridCell> {
        let mut cells = Vec::new();
        let cell_size = size / 3;

        // Create 3x3 grid centered around the given point
        let start_x = center.x - (size as i32 / 2);
        let start_y = center.y - (size as i32 / 2);

        for row in 0..3 {
            for col in 0..3 {
                let coord = GridCoord { x: col, y: row };
                let rect = Rectangle::new(
                    start_x + (col * cell_size) as i32,
                    start_y + (row * cell_size) as i32,
                    cell_size,
                    cell_size,
                );

                // Label cells with arrow directions or center
                let label = match (col, row) {
                    (1, 0) => "↑".to_string(), // Up
                    (1, 2) => "↓".to_string(), // Down
                    (0, 1) => "←".to_string(), // Left
                    (2, 1) => "→".to_string(), // Right
                    (0, 0) => "↖".to_string(), // Up-left
                    (2, 0) => "↗".to_string(), // Up-right
                    (0, 2) => "↙".to_string(), // Down-left
                    (2, 2) => "↘".to_string(), // Down-right
                    (1, 1) => "●".to_string(), // Center
                    _ => "?".to_string(),
                };

                cells.push(GridCell::new(coord, rect, label));
            }
        }

        cells
    }

    fn screen_to_grid_size(screen: &ScreenInfo) -> u32 {
        // Start with a reasonable fraction of screen size
        std::cmp::min(screen.width, screen.height) / 2
    }

    fn get_current_center(&self, screen: &ScreenInfo) -> ScreenCoord {
        if let Some(level) = self.grid_stack.last() {
            level.center
        } else {
            // Default to screen center
            ScreenCoord {
                x: screen.width as i32 / 2,
                y: screen.height as i32 / 2,
            }
        }
    }

    fn get_current_size(&self, screen: &ScreenInfo) -> u32 {
        if let Some(level) = self.grid_stack.last() {
            level.size
        } else {
            Self::screen_to_grid_size(screen)
        }
    }

    fn process_key_with_screen(
        &mut self,
        key: KeyCode,
        _current_level: u32,
        screen: &ScreenInfo,
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
                if self.current_pos.y < 2 {
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
                if self.current_pos.x < 2 {
                    self.current_pos.x += 1;
                }
                Some(self.current_pos)
            }
            // Enter key to descend one level
            KEYCODE_ENTER => {
                if self.grid_stack.len() < self.max_levels as usize {
                    let current_center = self.get_current_center(screen);
                    let current_size = self.get_current_size(screen);
                    let cell_size = current_size / 3;

                    // Calculate new center based on current position
                    let start_x = current_center.x - (current_size as i32 / 2);
                    let start_y = current_center.y - (current_size as i32 / 2);

                    let new_center = ScreenCoord {
                        x: start_x
                            + (self.current_pos.x * cell_size) as i32
                            + (cell_size as i32 / 2),
                        y: start_y
                            + (self.current_pos.y * cell_size) as i32
                            + (cell_size as i32 / 2),
                    };

                    // Push new level onto stack
                    self.grid_stack.push(GridLevel {
                        center: new_center,
                        size: cell_size,
                    });

                    // Reset position to center for new level
                    self.current_pos = GridCoord { x: 1, y: 1 };
                }
                Some(self.current_pos)
            }
            // Escape key to ascend one level
            KEYCODE_ESCAPE => {
                if !self.grid_stack.is_empty() {
                    self.grid_stack.pop();
                    // Reset to center position when going back
                    self.current_pos = GridCoord { x: 1, y: 1 };
                }
                Some(self.current_pos)
            }
            // Space key to select current position and move cursor
            KEYCODE_SPACE => Some(self.current_pos),
            _ => None,
        }
    }
}

impl GridLayout for ArrowGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, _density: u32) -> Vec<GridCell> {
        let center = self.get_current_center(screen);
        let size = self.get_current_size(screen);
        self.calculate_3x3_cells(center, size)
    }

    fn process_key(&mut self, key: KeyCode, screen: &ScreenInfo, current_level: u32) -> Option<GridCoord> {
        self.process_key_with_screen(key, current_level, screen, 4)
    }

    fn name(&self) -> &'static str {
        "Arrow Key Grid"
    }

    fn reset(&mut self) {
        self.grid_stack.clear();
        self.current_pos = GridCoord { x: 1, y: 1 };
    }
}