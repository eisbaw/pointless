use super::GridLayout;
use crate::constants::*;
use crate::types::{GridCell, GridCoord, KeyCode, Rectangle, ScreenCoord};
use crate::x11_client::ScreenInfo;
use std::collections::HashMap;

/// QWERTY Grid maps keyboard layout to screen regions
pub struct QwertyGrid {
    key_map: HashMap<KeyCode, ScreenCoord>,
    selected_region: Option<Rectangle>,
    refinement_level: u32,
    max_refinements: u32,
}

impl QwertyGrid {
    pub fn new() -> Self {
        Self {
            key_map: HashMap::new(),
            selected_region: None,
            refinement_level: 0,
            max_refinements: 3,
        }
    }

    fn keycode_to_label(&self, keycode: KeyCode) -> String {
        match keycode {
            KEYCODE_Q => "Q".to_string(),
            KEYCODE_W => "W".to_string(),
            KEYCODE_E => "E".to_string(),
            KEYCODE_R => "R".to_string(),
            KEYCODE_T => "T".to_string(),
            KEYCODE_Y => "Y".to_string(),
            KEYCODE_U => "U".to_string(),
            KEYCODE_I => "I".to_string(),
            KEYCODE_O => "O".to_string(),
            KEYCODE_P => "P".to_string(),
            KEYCODE_A => "A".to_string(),
            KEYCODE_S => "S".to_string(),
            KEYCODE_D => "D".to_string(),
            KEYCODE_F => "F".to_string(),
            KEYCODE_G => "G".to_string(),
            KEYCODE_H => "H".to_string(),
            KEYCODE_J => "J".to_string(),
            KEYCODE_K => "K".to_string(),
            KEYCODE_L => "L".to_string(),
            KEYCODE_Z => "Z".to_string(),
            KEYCODE_X => "X".to_string(),
            KEYCODE_C => "C".to_string(),
            KEYCODE_V => "V".to_string(),
            KEYCODE_B => "B".to_string(),
            KEYCODE_N => "N".to_string(),
            KEYCODE_M => "M".to_string(),
            _ => "?".to_string(),
        }
    }

    fn get_region_for_key(&self, keycode: KeyCode, screen: &ScreenInfo) -> Option<Rectangle> {
        // Get the region that this key represents
        let qwerty_rows = vec![
            vec![
                KEYCODE_Q, KEYCODE_W, KEYCODE_E, KEYCODE_R, KEYCODE_T, KEYCODE_Y, KEYCODE_U,
                KEYCODE_I, KEYCODE_O, KEYCODE_P,
            ],
            vec![
                KEYCODE_A, KEYCODE_S, KEYCODE_D, KEYCODE_F, KEYCODE_G, KEYCODE_H, KEYCODE_J,
                KEYCODE_K, KEYCODE_L,
            ],
            vec![
                KEYCODE_Z, KEYCODE_X, KEYCODE_C, KEYCODE_V, KEYCODE_B, KEYCODE_N, KEYCODE_M,
            ],
        ];

        let region = self
            .selected_region
            .unwrap_or_else(|| Rectangle::new(0, 0, screen.width, screen.height));

        for (row_idx, row) in qwerty_rows.iter().enumerate() {
            for (col_idx, &key) in row.iter().enumerate() {
                if key == keycode {
                    let rows = qwerty_rows.len() as u32;
                    let cols = row.len() as u32;
                    let row_height = region.height / rows;
                    let col_width = region.width / cols;

                    let x = region.x + (col_idx as u32 * col_width) as i32;
                    let y = region.y + (row_idx as u32 * row_height) as i32;

                    return Some(Rectangle::new(x, y, col_width, row_height));
                }
            }
        }
        None
    }
}

impl GridLayout for QwertyGrid {
    fn calculate_grid(&self, screen: &ScreenInfo, _density: u32) -> Vec<GridCell> {
        let mut cells = Vec::new();

        // Define QWERTY layout
        let qwerty_rows = vec![
            vec![
                KEYCODE_Q, KEYCODE_W, KEYCODE_E, KEYCODE_R, KEYCODE_T, KEYCODE_Y, KEYCODE_U,
                KEYCODE_I, KEYCODE_O, KEYCODE_P,
            ],
            vec![
                KEYCODE_A, KEYCODE_S, KEYCODE_D, KEYCODE_F, KEYCODE_G, KEYCODE_H, KEYCODE_J,
                KEYCODE_K, KEYCODE_L,
            ],
            vec![
                KEYCODE_Z, KEYCODE_X, KEYCODE_C, KEYCODE_V, KEYCODE_B, KEYCODE_N, KEYCODE_M,
            ],
        ];

        let region = self
            .selected_region
            .unwrap_or_else(|| Rectangle::new(0, 0, screen.width, screen.height));

        let rows = qwerty_rows.len() as u32;
        let row_height = region.height / rows;

        for (row_idx, row) in qwerty_rows.iter().enumerate() {
            let cols = row.len() as u32;
            let col_width = region.width / cols;

            for (col_idx, &keycode) in row.iter().enumerate() {
                let coord = GridCoord {
                    x: col_idx as u32,
                    y: row_idx as u32,
                };

                let rect = Rectangle::new(
                    region.x + (col_idx as u32 * col_width) as i32,
                    region.y + (row_idx as u32 * row_height) as i32,
                    col_width,
                    row_height,
                );

                let label = self.keycode_to_label(keycode);
                cells.push(GridCell::new(coord, rect, label));
            }
        }

        cells
    }

    fn process_key(&mut self, _key: KeyCode, _current_level: u32) -> Option<GridCoord> {
        None // Use process_key_with_screen instead
    }

    fn process_key_with_screen(
        &mut self,
        key: KeyCode,
        _current_level: u32,
        screen: &ScreenInfo,
        _density: u32,
    ) -> Option<GridCoord> {
        // Handle escape key to reset refinement
        if key == KEYCODE_ESCAPE {
            self.selected_region = None;
            self.refinement_level = 0;
            return None;
        }

        // Check if this is a valid QWERTY key
        let valid_keys = vec![
            KEYCODE_Q, KEYCODE_W, KEYCODE_E, KEYCODE_R, KEYCODE_T, KEYCODE_Y, KEYCODE_U, KEYCODE_I,
            KEYCODE_O, KEYCODE_P, KEYCODE_A, KEYCODE_S, KEYCODE_D, KEYCODE_F, KEYCODE_G, KEYCODE_H,
            KEYCODE_J, KEYCODE_K, KEYCODE_L, KEYCODE_Z, KEYCODE_X, KEYCODE_C, KEYCODE_V, KEYCODE_B,
            KEYCODE_N, KEYCODE_M,
        ];

        if !valid_keys.contains(&key) {
            return None;
        }

        // If we can refine further, set up the selected region for next level
        if self.refinement_level < self.max_refinements {
            if let Some(region) = self.get_region_for_key(key, screen) {
                self.selected_region = Some(region);
                self.refinement_level += 1;
            }
        }

        // Always return the coordinate for cursor movement
        // Find the coordinate for this key
        let qwerty_rows = vec![
            vec![
                KEYCODE_Q, KEYCODE_W, KEYCODE_E, KEYCODE_R, KEYCODE_T, KEYCODE_Y, KEYCODE_U,
                KEYCODE_I, KEYCODE_O, KEYCODE_P,
            ],
            vec![
                KEYCODE_A, KEYCODE_S, KEYCODE_D, KEYCODE_F, KEYCODE_G, KEYCODE_H, KEYCODE_J,
                KEYCODE_K, KEYCODE_L,
            ],
            vec![
                KEYCODE_Z, KEYCODE_X, KEYCODE_C, KEYCODE_V, KEYCODE_B, KEYCODE_N, KEYCODE_M,
            ],
        ];

        for (row_idx, row) in qwerty_rows.iter().enumerate() {
            for (col_idx, &keycode) in row.iter().enumerate() {
                if keycode == key {
                    return Some(GridCoord {
                        x: col_idx as u32,
                        y: row_idx as u32,
                    });
                }
            }
        }

        None
    }

    fn name(&self) -> &'static str {
        "QWERTY Grid"
    }

    fn reset(&mut self) {
        self.selected_region = None;
        self.refinement_level = 0;
        self.key_map.clear();
    }
}
