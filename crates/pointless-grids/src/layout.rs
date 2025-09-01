use crate::types::{GridCell, GridCoord, KeyCode};
use pointless_x11::ScreenInfo;

/// Trait for pluggable grid layout implementations
pub trait GridLayout {
    /// Calculate grid cells for given screen dimensions
    fn calculate_grid(&self, screen: &ScreenInfo, density: u32) -> Vec<GridCell>;

    /// Process key input and return target coordinate
    ///
    /// # Arguments
    /// * `key` - X11 keycode from user input
    /// * `screen` - Current screen dimensions and properties
    /// * `current_level` - Current subdivision level (for hierarchical grids)
    ///
    /// # Returns
    /// * `Some(GridCoord)` - Target coordinate if key produces valid movement
    /// * `None` - If key is invalid or enters a mode without movement
    fn process_key(
        &mut self,
        key: KeyCode,
        screen: &ScreenInfo,
        current_level: u32,
    ) -> Option<GridCoord>;

    /// Get human-readable name for this grid type
    fn name(&self) -> &'static str;

    /// Reset navigation state
    fn reset(&mut self);
}
