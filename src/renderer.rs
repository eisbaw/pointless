use crate::config::Config;
use crate::constants::*;
use crate::error::{PointlessError, Result};
use crate::types::GridCell;
use crate::x11_client::X11Client;
use std::ffi::CString;
use std::ptr;
use x11::xlib::{
    CWBackPixel, CWBorderPixel, CWOverrideRedirect, InputOutput, Window, XClearWindow, XCreateGC,
    XCreateWindow, XDestroyWindow, XDrawRectangle, XDrawString, XFlush, XMapWindow, XSetForeground,
    XSetWindowAttributes, XStoreName, XUnmapWindow, GC,
};

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
    pub fn new() -> Self {
        Self {
            overlay_window: None,
            gc: None,
            is_visible: false,
        }
    }

    pub fn create_overlay(&mut self, x11_client: &X11Client, config: &Config) -> Result<()> {
        // Idempotency guard - avoid creating multiple overlays
        if self.overlay_window.is_some() {
            return Ok(());
        }

        let screen_info = x11_client.screen_info();

        unsafe {
            let display = x11_client.display();
            let root = x11_client.root_window();

            // Create overlay window with override_redirect
            let mut window_attrs: XSetWindowAttributes = std::mem::zeroed();
            window_attrs.override_redirect = 1; // Override window manager
            window_attrs.background_pixel = 0; // Transparent background
            window_attrs.border_pixel = 0;

            let overlay_window = XCreateWindow(
                display,
                root,
                0,
                0, // x, y
                screen_info.width,
                screen_info.height,                               // width, height
                0,                                                // border_width
                0,                                                // depth (CopyFromParent)
                InputOutput as u32,                               // class
                ptr::null_mut(),                                  // visual (CopyFromParent)
                CWOverrideRedirect | CWBackPixel | CWBorderPixel, // valuemask
                &mut window_attrs,
            );

            if overlay_window == 0 {
                return Err(PointlessError::WindowCreationFailed);
            }

            // Set window name for debugging
            let window_name = CString::new("Pointless Overlay").unwrap();
            XStoreName(display, overlay_window, window_name.as_ptr());

            // Create graphics context
            let gc = XCreateGC(display, overlay_window, 0, ptr::null_mut());
            if gc.is_null() {
                XDestroyWindow(display, overlay_window);
                return Err(PointlessError::GraphicsContextFailed);
            }

            // Set foreground color - fail fast if invalid color
            let color_rgb = config.color_rgb()?;
            XSetForeground(display, gc, color_rgb as u64);

            self.overlay_window = Some(overlay_window);
            self.gc = Some(gc);

            Ok(())
        }
    }

    pub fn show_overlay(&mut self, x11_client: &X11Client) -> Result<()> {
        if let Some(window) = self.overlay_window {
            if !self.is_visible {
                unsafe {
                    XMapWindow(x11_client.display(), window);
                    XFlush(x11_client.display());
                }
                self.is_visible = true;
            }
        }
        Ok(())
    }

    pub fn hide_overlay(&mut self, x11_client: &X11Client) -> Result<()> {
        if let Some(window) = self.overlay_window {
            if self.is_visible {
                unsafe {
                    XUnmapWindow(x11_client.display(), window);
                    XFlush(x11_client.display());
                }
                self.is_visible = false;
            }
        }
        Ok(())
    }

    pub fn draw_grid(&mut self, x11_client: &X11Client, cells: &[GridCell]) -> Result<()> {
        if let (Some(window), Some(gc)) = (self.overlay_window, self.gc) {
            unsafe {
                let display = x11_client.display();

                // Clear the window
                XClearWindow(display, window);

                // Draw each grid cell
                for cell in cells {
                    let rect = &cell.screen_rect;

                    // Draw the cell border (1-pixel yellow lines)
                    XDrawRectangle(display, window, gc, rect.x, rect.y, rect.width, rect.height);

                    // Draw cell label at center
                    let label_cstring = CString::new(cell.label.as_str()).unwrap();
                    let center = rect.center();

                    // Position text approximately at center using constants
                    let text_x = center.x - ((cell.label.len() * CHAR_WIDTH_ESTIMATE) as i32 / 2);
                    let text_y = center.y + TEXT_VERTICAL_OFFSET;

                    XDrawString(
                        display,
                        window,
                        gc,
                        text_x,
                        text_y,
                        label_cstring.as_ptr(),
                        cell.label.len() as i32,
                    );
                }

                XFlush(display);
            }
        }
        Ok(())
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn destroy_overlay(&mut self, x11_client: &X11Client) {
        unsafe {
            let display = x11_client.display();

            if let Some(window) = self.overlay_window.take() {
                XDestroyWindow(display, window);
            }

            // Note: GC is automatically freed when window is destroyed
            self.gc = None;
            self.is_visible = false;
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // Note: We can't call X11 functions in Drop without the X11Client
        // The caller should call destroy_overlay explicitly
    }
}
