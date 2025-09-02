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
pub struct ColorGrid {
    color_regions: Vec<ScreenCoord>,
    jump_target_map: HashMap<KeyCode, usize>,
}

#[cfg(feature = "vision-grids")]
impl Default for ColorGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "vision-grids")]
impl ColorGrid {
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

        // Some default color region centers
        let color_regions = vec![
            ScreenCoord { x: 220, y: 160 },
            ScreenCoord { x: 420, y: 320 },
            ScreenCoord { x: 620, y: 220 },
            ScreenCoord { x: 320, y: 520 },
        ];

        Self {
            color_regions,
            jump_target_map,
        }
    }
}

#[cfg(feature = "vision-grids")]
impl GridLayout for ColorGrid {
    fn calculate_grid(&self, _screen: &ScreenInfo, _density: u32) -> Vec<GridCell> {
        self.color_regions
            .iter()
            .enumerate()
            .map(|(i, &pos)| {
                GridCell::new(
                    GridCoord { x: i as u32, y: 0 },
                    Rectangle::new(pos.x - 30, pos.y - 30, 60, 60),
                    format!("{}", i + 1),
                )
            })
            .collect()
    }

    fn process_key(
        &mut self,
        key: KeyCode,
        _screen: &ScreenInfo,
        _current_level: u32,
    ) -> Option<GridCoord> {
        self.jump_target_map.get(&key).map(|&index| GridCoord {
            x: index as u32,
            y: 0,
        })
    }

    fn name(&self) -> &'static str {
        "Color Clustering Grid"
    }

    fn reset(&mut self) {}

    fn grid_to_screen_coord(
        &self,
        coord: GridCoord,
        _screen: &ScreenInfo,
        _density: u32,
    ) -> ScreenCoord {
        // Return the pre-computed color region coordinates
        if (coord.x as usize) < self.color_regions.len() {
            self.color_regions[coord.x as usize]
        } else {
            // Fallback to first region if index is out of bounds
            self.color_regions.get(0).copied().unwrap_or(ScreenCoord { x: 0, y: 0 })
        }
    }
}
