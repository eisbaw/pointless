use super::GridLayout;
use crate::constants::*;
use crate::types::{GridCell, GridCoord, KeyCode, Rectangle, ScreenCoord};
use crate::x11_client::ScreenInfo;

#[derive(Debug, Clone)]
struct VimJumpTarget {
    position: ScreenCoord,
    label: char, // 1-9 for jump targets
}

#[derive(Debug, Clone)]
enum VimMovementMode {
    Normal,
    FindForward,  // After 'w' pressed
    FindBackward, // After 'b' pressed
}

/// Vim Grid provides Vim-like navigation commands
pub struct VimGrid {
    current_cursor: ScreenCoord,
    jump_targets: Vec<VimJumpTarget>,
    movement_mode: VimMovementMode,
    pending_multiplier: Option<u32>,
    movement_step: i32, // Base movement step in pixels
}

impl Default for VimGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl VimGrid {
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_cursor: ScreenCoord { x: 0, y: 0 },
            jump_targets: Vec::new(),
            movement_mode: VimMovementMode::Normal,
            pending_multiplier: None,
            movement_step: 10, // Default 10 pixels per movement
        }
    }

    fn move_relative(&mut self, dx: i32, dy: i32, screen: &ScreenInfo) -> ScreenCoord {
        let multiplier = self.pending_multiplier.unwrap_or(1) as i32;
        let new_x = (self.current_cursor.x + dx * multiplier)
            .max(0)
            .min(screen.width as i32 - 1);
        let new_y = (self.current_cursor.y + dy * multiplier)
            .max(0)
            .min(screen.height as i32 - 1);

        self.current_cursor = ScreenCoord { x: new_x, y: new_y };
        self.pending_multiplier = None;
        self.current_cursor
    }

    fn jump_to_position(&mut self, x: i32, y: i32, screen: &ScreenInfo) -> ScreenCoord {
        let new_x = x.max(0).min(screen.width as i32 - 1);
        let new_y = y.max(0).min(screen.height as i32 - 1);
        self.current_cursor = ScreenCoord { x: new_x, y: new_y };
        self.current_cursor
    }

    fn generate_forward_targets(&self, screen: &ScreenInfo) -> Vec<VimJumpTarget> {
        let mut targets = Vec::new();
        let step_size = screen.width as i32 / 10; // Divide screen into 10 sections

        // Generate 9 targets in forward/right direction from current cursor
        for i in 1..=9 {
            let target_x = self.current_cursor.x + i * step_size;
            let target_y = self.current_cursor.y;

            if target_x < screen.width as i32 {
                targets.push(VimJumpTarget {
                    position: ScreenCoord {
                        x: target_x,
                        y: target_y,
                    },
                    label: char::from_digit(i as u32, 10).unwrap_or('?'),
                });
            }
        }

        targets
    }

    fn generate_backward_targets(&self, screen: &ScreenInfo) -> Vec<VimJumpTarget> {
        let mut targets = Vec::new();
        let step_size = screen.width as i32 / 10;

        // Generate 9 targets in backward/left direction from current cursor
        for i in 1..=9 {
            let target_x = self.current_cursor.x - i * step_size;
            let target_y = self.current_cursor.y;

            if target_x >= 0 {
                targets.push(VimJumpTarget {
                    position: ScreenCoord {
                        x: target_x,
                        y: target_y,
                    },
                    label: char::from_digit(i as u32, 10).unwrap_or('?'),
                });
            }
        }

        targets
    }

    fn enter_find_forward_mode(&mut self, screen: &ScreenInfo) {
        self.movement_mode = VimMovementMode::FindForward;
        self.jump_targets = self.generate_forward_targets(screen);
    }

    fn enter_find_backward_mode(&mut self, screen: &ScreenInfo) {
        self.movement_mode = VimMovementMode::FindBackward;
        self.jump_targets = self.generate_backward_targets(screen);
    }

    fn jump_to_numbered_target(&mut self, number: u32) -> Option<ScreenCoord> {
        let target_char = char::from_digit(number, 10)?;

        for target in &self.jump_targets {
            if target.label == target_char {
                self.current_cursor = target.position;
                self.movement_mode = VimMovementMode::Normal;
                self.jump_targets.clear();
                return Some(self.current_cursor);
            }
        }
        None
    }
}

impl GridLayout for VimGrid {
    fn calculate_grid(&self, _screen: &ScreenInfo, _density: u32) -> Vec<GridCell> {
        let mut cells = Vec::new();

        match self.movement_mode {
            VimMovementMode::FindForward | VimMovementMode::FindBackward => {
                // Show jump targets as grid cells
                for target in &self.jump_targets {
                    let coord = GridCoord {
                        x: target.position.x as u32,
                        y: target.position.y as u32,
                    };

                    // Create small rectangles around each jump target
                    let rect = Rectangle::new(
                        target.position.x - crate::constants::VIM_TARGET_OFFSET as i32,
                        target.position.y - crate::constants::VIM_TARGET_OFFSET as i32,
                        crate::constants::VIM_TARGET_SIZE,
                        crate::constants::VIM_TARGET_SIZE,
                    );

                    cells.push(GridCell::new(coord, rect, target.label.to_string()));
                }
            }
            _ => {
                // No grid overlay for basic h/j/k/l movement
                // Just show current cursor position
                let coord = GridCoord {
                    x: self.current_cursor.x as u32,
                    y: self.current_cursor.y as u32,
                };

                let rect = Rectangle::new(
                    self.current_cursor.x - 10,
                    self.current_cursor.y - 10,
                    crate::constants::VIM_TARGET_OFFSET,
                    crate::constants::VIM_TARGET_OFFSET,
                );

                cells.push(GridCell::new(coord, rect, "●".to_string()));
            }
        }

        cells
    }

    fn process_key(
        &mut self,
        key: KeyCode,
        _current_level: u32,
        screen: &ScreenInfo,
        _density: u32,
    ) -> Option<GridCoord> {
        // Handle number multipliers first, but exclude 0 which is used for positioning
        if key != KEYCODE_0 {
            if let Some(number) = self.keycode_to_number(key) {
                if matches!(
                    self.movement_mode,
                    VimMovementMode::FindForward | VimMovementMode::FindBackward
                ) {
                    // In jump mode, numbers select targets
                    if let Some(pos) = self.jump_to_numbered_target(number) {
                        return Some(GridCoord {
                            x: pos.x as u32,
                            y: pos.y as u32,
                        });
                    }
                } else {
                    // In normal mode, numbers are multipliers
                    self.pending_multiplier =
                        Some(self.pending_multiplier.unwrap_or(0) * 10 + number);
                    return None;
                }
            }
        }

        // Handle movement commands
        let result = match key {
            // Basic movement: h/j/k/l
            KEYCODE_H => {
                let pos = self.move_relative(-self.movement_step, 0, screen);
                Some(GridCoord {
                    x: pos.x as u32,
                    y: pos.y as u32,
                })
            }
            KEYCODE_J => {
                let pos = self.move_relative(0, self.movement_step, screen);
                Some(GridCoord {
                    x: pos.x as u32,
                    y: pos.y as u32,
                })
            }
            KEYCODE_K => {
                let pos = self.move_relative(0, -self.movement_step, screen);
                Some(GridCoord {
                    x: pos.x as u32,
                    y: pos.y as u32,
                })
            }
            KEYCODE_L => {
                let pos = self.move_relative(self.movement_step, 0, screen);
                Some(GridCoord {
                    x: pos.x as u32,
                    y: pos.y as u32,
                })
            }

            // Jump commands
            KEYCODE_W => {
                self.enter_find_forward_mode(screen);
                None
            }
            KEYCODE_B => {
                self.enter_find_backward_mode(screen);
                None
            }

            // Screen positioning commands (uppercase variants)
            // Note: These would need uppercase keycode constants, for now we use combinations
            KEYCODE_0 => {
                // 0 - jump to left edge
                let pos = self.jump_to_position(0, self.current_cursor.y, screen);
                Some(GridCoord {
                    x: pos.x as u32,
                    y: pos.y as u32,
                })
            }
            KEYCODE_DOLLAR => {
                // $ - jump to right edge
                let pos =
                    self.jump_to_position(screen.width as i32 - 1, self.current_cursor.y, screen);
                Some(GridCoord {
                    x: pos.x as u32,
                    y: pos.y as u32,
                })
            }

            // G commands (need to handle gg)
            KEYCODE_G => {
                // For now, just jump to bottom-right corner
                let pos = self.jump_to_position(
                    screen.width as i32 - 1,
                    screen.height as i32 - 1,
                    screen,
                );
                Some(GridCoord {
                    x: pos.x as u32,
                    y: pos.y as u32,
                })
            }

            // Escape - exit jump mode
            KEYCODE_ESCAPE => {
                self.movement_mode = VimMovementMode::Normal;
                self.jump_targets.clear();
                self.pending_multiplier = None;
                None
            }

            _ => None,
        };

        // Clear multiplier after successful movement
        if result.is_some()
            && !matches!(
                self.movement_mode,
                VimMovementMode::FindForward | VimMovementMode::FindBackward
            )
        {
            self.pending_multiplier = None;
        }

        result
    }

    fn name(&self) -> &'static str {
        "Vim Grid"
    }

    fn reset(&mut self) {
        self.movement_mode = VimMovementMode::Normal;
        self.jump_targets.clear();
        self.pending_multiplier = None;
        self.current_cursor = ScreenCoord { x: 0, y: 0 };
    }

    fn grid_to_screen_coord(
        &self,
        coord: GridCoord,
        screen: &ScreenInfo,
        density: u32,
    ) -> ScreenCoord {
        use crate::coordinate_resolver::CoordinateResolver;

        // Vim grids already work in screen coordinate space
        CoordinateResolver::grid_to_screen_direct(coord, screen, density)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl VimGrid {
    fn keycode_to_number(&self, keycode: KeyCode) -> Option<u32> {
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
            KEYCODE_0 => Some(0),
            _ => None,
        }
    }
}
