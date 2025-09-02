#[cfg(feature = "vision-grids")]
use crate::{GridCell, GridCoord, GridLayout, KeyCode};
#[cfg(feature = "vision-grids")]
use pointless_x11::{Rectangle, ScreenCoord, ScreenInfo};
#[cfg(feature = "vision-grids")]
use std::collections::HashMap;

// X11 Keycode mappings for number keys
#[cfg(feature = "vision-grids")]
const KEYCODE_1: u32 = 10;
#[cfg(feature = "vision-grids")]
const KEYCODE_2: u32 = 11;
#[cfg(feature = "vision-grids")]
const KEYCODE_3: u32 = 12;
#[cfg(feature = "vision-grids")]
const KEYCODE_4: u32 = 13;
#[cfg(feature = "vision-grids")]
const KEYCODE_5: u32 = 14;
#[cfg(feature = "vision-grids")]
const KEYCODE_6: u32 = 15;
#[cfg(feature = "vision-grids")]
const KEYCODE_7: u32 = 16;
#[cfg(feature = "vision-grids")]
const KEYCODE_8: u32 = 17;
#[cfg(feature = "vision-grids")]
const KEYCODE_9: u32 = 18;

#[cfg(feature = "vision-grids")]
#[derive(Debug)]
pub struct RectangleGrid {
    rectangles: Vec<ScreenCoord>,
    jump_target_map: HashMap<KeyCode, usize>,
}

#[cfg(feature = "vision-grids")]
impl Default for RectangleGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "vision-grids")]
impl RectangleGrid {
    pub fn new() -> Self {
        let mut jump_target_map = HashMap::new();
        for i in 1..=9 {
            let keycode = match i {
                1 => KEYCODE_1,
                2 => KEYCODE_2,
                3 => KEYCODE_3,
                4 => KEYCODE_4,
                5 => KEYCODE_5,
                6 => KEYCODE_6,
                7 => KEYCODE_7,
                8 => KEYCODE_8,
                9 => KEYCODE_9,
                _ => continue,
            };
            jump_target_map.insert(keycode, i - 1);
        }

        // Some default rectangle centers
        let rectangles = vec![
            ScreenCoord { x: 150, y: 100 },
            ScreenCoord { x: 350, y: 250 },
            ScreenCoord { x: 550, y: 400 },
            ScreenCoord { x: 750, y: 150 },
        ];

        Self {
            rectangles,
            jump_target_map,
        }
    }
}

#[cfg(feature = "vision-grids")]
impl GridLayout for RectangleGrid {
    fn calculate_grid(&self, _screen: &ScreenInfo, _density: u32) -> Vec<GridCell> {
        self.rectangles
            .iter()
            .enumerate()
            .map(|(i, &pos)| {
                GridCell::new(
                    GridCoord { x: i as u32, y: 0 },
                    Rectangle::new(pos.x - 25, pos.y - 25, 50, 50),
                    format!("{}", i + 1),
                )
            })
            .collect()
    }

    fn process_key(
        &mut self,
        key: KeyCode,
        _current_level: u32,
        _screen: &ScreenInfo,
        _density: u32,
    ) -> Option<GridCoord> {
        self.jump_target_map.get(&key).map(|&index| GridCoord {
            x: index as u32,
            y: 0,
        })
    }

    fn name(&self) -> &'static str {
        "Rectangle Detection Grid"
    }

    fn reset(&mut self) {}

    fn grid_to_screen_coord(
        &self,
        coord: GridCoord,
        _screen: &ScreenInfo,
        _density: u32,
    ) -> ScreenCoord {
        // Return the pre-computed rectangle center coordinates
        // Clamp coordinate to valid bounds instead of silent fallback
        let index = if self.rectangles.is_empty() {
            // If no rectangles detected, return screen center
            return ScreenCoord {
                x: _screen.width as i32 / 2,
                y: _screen.height as i32 / 2,
            };
        } else {
            // Clamp to valid range
            (coord.x as usize).min(self.rectangles.len() - 1)
        };

        self.rectangles[index]
    }
}
