use crate::types::{GridCell, GridCoord, KeyCode};
use pointless_x11::{ScreenCoord, ScreenInfo};

/// Trait for pluggable grid layout implementations
pub trait GridLayout {
    /// Calculate grid cells for given screen dimensions
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell>;

    /// Process key input and return target coordinate
    ///
    /// # Arguments
    /// * `key` - X11 keycode from user input
    /// * `current_level` - Current subdivision level (for hierarchical grids)
    /// * `screen` - Current screen dimensions and properties
    /// * `density` - Grid density/subdivision level for calculations
    ///
    /// # Returns
    /// * `Some(GridCoord)` - Target coordinate if key produces valid movement
    /// * `None` - If key is invalid or enters a mode without movement
    fn process_key(
        &mut self,
        key: KeyCode,
        current_level: u32,
        screen: &ScreenInfo,
        density: u32,
    ) -> Option<GridCoord>;

    /// Get human-readable name for this grid type
    fn name(&self) -> &'static str;

    /// Reset navigation state
    fn reset(&mut self);

    /// Convert grid coordinate to screen coordinate efficiently (without full grid calculation)
    fn grid_to_screen_coord(
        &self,
        coord: GridCoord,
        screen: &ScreenInfo,
        density: u32,
    ) -> ScreenCoord;
}
