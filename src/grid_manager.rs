use crate::config::GridType;
use crate::grids::{ArrowGrid, GridLayout, HomogeneousGrid, QwertyGrid, VimGrid};
use crate::types::{GridCell, KeyCode, ScreenCoord};
use crate::x11_client::ScreenInfo;

pub struct GridManager {
    current_grid: Box<dyn GridLayout>,
    current_level: u32,
}

impl GridManager {
    #[must_use]
    pub fn new(grid_type: GridType) -> Self {
        let current_grid: Box<dyn GridLayout> = match grid_type {
            GridType::Homogeneous => Box::new(HomogeneousGrid::new()),
            GridType::Qwerty => Box::new(QwertyGrid::new()),
            GridType::Arrow => Box::new(ArrowGrid::new()),
            GridType::Vim => Box::new(VimGrid::new()),
        };

        Self {
            current_grid,
            current_level: 0,
        }
    }

    #[must_use]
    pub fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        // Always compute fresh - following MPED principle of computing derived state on demand
        self.current_grid.calculate_grid(screen, density)
    }

    #[must_use]
    pub fn calculate_localized_grid(
        &self,
        screen: &ScreenInfo,
        cursor_x: i32,
        cursor_y: i32,
    ) -> Vec<GridCell> {
        // For now, only support localized grids for qwerty
        if let Some(qwerty_grid) = self.current_grid.as_any().downcast_ref::<QwertyGrid>() {
            qwerty_grid.calculate_localized_grid(screen, cursor_x, cursor_y)
        } else {
            // For other grid types, fall back to regular grid
            self.calculate_grid(screen, 3)
        }
    }

    pub fn process_key(
        &mut self,
        key: KeyCode,
        screen: &ScreenInfo,
        density: u32,
    ) -> Option<ScreenCoord> {
        if let Some(coord) = self
            .current_grid
            .process_key(key, self.current_level, screen, density)
        {
            log::debug!(
                "Grid returned coordinate: ({}, {}) for keycode {}",
                coord.x,
                coord.y,
                key
            );

            // Use efficient coordinate transformation without full grid recalculation
            let screen_coord = self
                .current_grid
                .grid_to_screen_coord(coord, screen, density);

            log::debug!(
                "Final screen coordinate: ({}, {}) for grid coordinate ({}, {})",
                screen_coord.x,
                screen_coord.y,
                coord.x,
                coord.y
            );

            return Some(screen_coord);
        }
        log::debug!("No grid coordinate returned for keycode {}", key);
        None
    }

    pub fn reset(&mut self) {
        self.current_grid.reset();
        self.current_level = 0;
    }
}
