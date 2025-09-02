use crate::config::GridType;
use crate::grids::{ArrowGrid, GridLayout, HomogeneousGrid, QwertyGrid, VimGrid};
use crate::types::{GridCell, KeyCode, ScreenCoord};
use crate::x11_client::ScreenInfo;

pub struct GridManager {
    current_grid: Box<dyn GridLayout>,
    current_level: u32,
}

impl GridManager {
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

    pub fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell> {
        // Always compute fresh - following MPED principle of computing derived state on demand
        self.current_grid.calculate_grid(screen, density)
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
            // Use efficient coordinate transformation without full grid recalculation
            return Some(
                self.current_grid
                    .grid_to_screen_coord(coord, screen, density),
            );
        }
        None
    }

    pub fn reset(&mut self) {
        self.current_grid.reset();
        self.current_level = 0;
    }
}
