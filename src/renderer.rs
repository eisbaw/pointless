use crate::config::Config;
use crate::constants::*;
use crate::error::Result;
use crate::types::GridCell;
use crate::x11_client::X11Client;
use x11::xlib::{Window, GC};

pub struct Renderer {
    overlay_window: Option<Window>,
    gc: Option<GC>,
    is_visible: bool,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            overlay_window: None,
            gc: None,
            is_visible: false,
        }
    }

    /// Create X11 overlay window for grid rendering
    ///
    /// # Errors
    /// Returns `PointlessError::RenderingFailed` if X11 overlay window creation fails
    pub fn create_overlay(&mut self, x11_client: &X11Client, config: &Config) -> Result<()> {
        // Idempotency guard - avoid creating multiple overlays
        if self.overlay_window.is_some() {
            return Ok(());
        }

        let screen_info = x11_client.screen_info();

        let overlay_window =
            x11_client.create_overlay_window(0, 0, screen_info.width, screen_info.height)?;

        // Create graphics context
        let gc = x11_client.create_graphics_context(overlay_window)?;

        // Set foreground color - fail fast if invalid color
        let color_rgb = config.color_rgb()?;
        unsafe {
            x11_client.set_foreground_color(gc, color_rgb as u64);

            self.overlay_window = Some(overlay_window);
            self.gc = Some(gc);

            Ok(())
        }
    }

    /// Show the overlay window
    ///
    /// # Errors
    /// Currently always returns `Ok(())` but declared as Result for future X11 error handling
    pub fn show_overlay(&mut self, x11_client: &X11Client) -> Result<()> {
        if let Some(window) = self.overlay_window {
            if !self.is_visible {
                x11_client.map_window(window)?;
                x11_client.flush();
                self.is_visible = true;
            }
        }
        Ok(())
    }

    /// Hide the overlay window
    ///
    /// # Errors
    /// Currently always returns `Ok(())` but declared as Result for future X11 error handling
    pub fn hide_overlay(&mut self, x11_client: &X11Client) -> Result<()> {
        if let Some(window) = self.overlay_window {
            if self.is_visible {
                x11_client.unmap_window(window)?;
                x11_client.flush();
                self.is_visible = false;
            }
        }
        Ok(())
    }

    /// Draw grid cells on the overlay window
    ///
    /// # Errors
    /// Currently always returns `Ok(())` but declared as Result for future X11 error handling
    pub fn draw_grid(&mut self, x11_client: &X11Client, cells: &[GridCell]) -> Result<()> {
        if let (Some(window), Some(gc)) = (self.overlay_window, self.gc) {
            // Clear the window
            x11_client.clear_window(window);

            // Draw each grid cell
            for cell in cells {
                let rect = &cell.screen_rect;

                // Draw the cell border (1-pixel yellow lines)
                unsafe {
                    x11_client.draw_rectangle(window, gc, rect.x, rect.y, rect.width, rect.height);
                }

                // Draw cell label at center
                let center = rect.center();

                // Position text approximately at center using constants
                let text_x = center.x - ((cell.label.len() * CHAR_WIDTH_ESTIMATE) as i32 / 2);
                let text_y = center.y + TEXT_VERTICAL_OFFSET;

                unsafe {
                    x11_client.draw_text(window, gc, text_x, text_y, &cell.label);
                }
            }

            x11_client.flush();
        }
        Ok(())
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn destroy_overlay(&mut self, x11_client: &X11Client) {
        if let Some(window) = self.overlay_window.take() {
            x11_client.destroy_window(window);
        }

        // Note: GC is automatically freed when window is destroyed
        self.gc = None;
        self.is_visible = false;
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // MPED fail-fast principle: programmer error to drop without cleanup
        if self.overlay_window.is_some() || self.gc.is_some() {
            log::error!(
                "CRITICAL: Renderer dropped without calling destroy_overlay() - X11 resource leak! \
                window: {:?}, gc: {:?}. This is a programmer error.",
                self.overlay_window.is_some(),
                self.gc.is_some()
            );

            // In debug builds, panic to catch this error immediately during development
            debug_assert!(
                false,
                "Renderer must call destroy_overlay() before drop to prevent X11 resource leaks"
            );
        }
    }
}
