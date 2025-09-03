use crate::config::{Config, GridType};
use crate::grids::{ArrowGrid, GridLayout, HomogeneousGrid, QwertyGrid, VimGrid};
use crate::types::{GridCell, KeyCode, ScreenCoord};
use crate::x11_client::ScreenInfo;

pub struct GridManager {
    current_grid: Box<dyn GridLayout>,
    current_level: u32,
    is_localized_mode: bool,
    localized_cursor_pos: Option<(i32, i32)>,
}

impl GridManager {
    pub fn new(config: &Config) -> Result<Self, crate::error::PointlessError> {
        let current_grid: Box<dyn GridLayout> = match config.grid_type {
            GridType::Homogeneous => Box::new(HomogeneousGrid::new()),
            GridType::Qwerty => Box::new(QwertyGrid::new()),
            GridType::Arrow => {
                let (cols, rows) = config.parse_arrow_dimensions()?;
                Box::new(ArrowGrid::new_with_dimensions(cols, rows))
            }
            GridType::Vim => Box::new(VimGrid::new()),
        };

        Ok(Self {
            current_grid,
            current_level: 0,
            is_localized_mode: false,
            localized_cursor_pos: None,
        })
    }

    #[must_use]
    pub fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        // Always compute fresh - following MPED principle of computing derived state on demand
        self.current_grid.calculate_grid(screen, density)
    }

    pub fn calculate_localized_grid(
        &mut self,
        screen: &ScreenInfo,
        cursor_x: i32,
        cursor_y: i32,
    ) -> Vec<GridCell> {
        // For now, only support localized grids for qwerty
        if let Some(qwerty_grid) = self.current_grid.as_any().downcast_ref::<QwertyGrid>() {
            self.is_localized_mode = true;
            self.localized_cursor_pos = Some((cursor_x, cursor_y));
            qwerty_grid.calculate_localized_grid(screen, cursor_x, cursor_y)
        } else {
            // For other grid types, fall back to regular grid
            self.is_localized_mode = false;
            self.localized_cursor_pos = None;
            self.calculate_grid(screen, 3)
        }
    }

    pub fn set_full_grid_mode(&mut self) {
        self.is_localized_mode = false;
        self.localized_cursor_pos = None;
    }

    /// Process a key press using the currently displayed grid cells
    pub fn process_key_with_cells(
        &mut self,
        key: KeyCode,
        displayed_cells: &[GridCell],
        screen: &ScreenInfo,
    ) -> Option<ScreenCoord> {
        // Delegate to the current grid to find the coordinate for this key
        if let Some(coord) = self
            .current_grid
            .process_key(key, self.current_level, screen, 0)
        {
            // Find the cell with matching coordinate from the deepest level
            // Search in reverse order since deeper levels are added later
            for cell in displayed_cells.iter().rev() {
                if cell.coord == coord {
                    let center_x = cell.screen_rect.x + (cell.screen_rect.width / 2) as i32;
                    let center_y = cell.screen_rect.y + (cell.screen_rect.height / 2) as i32;

                    log::debug!(
                        "Found cell for key {}: grid({}, {}) -> screen({}, {}) [{}]",
                        key,
                        coord.x,
                        coord.y,
                        center_x,
                        center_y,
                        cell.label
                    );

                    return Some(ScreenCoord {
                        x: center_x,
                        y: center_y,
                    });
                }
            }

            log::warn!(
                "Could not find displayed cell for grid coordinate ({}, {})",
                coord.x,
                coord.y
            );
        } else {
            log::debug!("No grid coordinate returned for keycode {}", key);
        }

        None
    }

    pub fn reset(&mut self) {
        self.current_grid.reset();
        self.current_level = 0;
        self.is_localized_mode = false;
        self.localized_cursor_pos = None;
    }
}
