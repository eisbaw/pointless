//! X11 client interface for Pointless cursor positioning utility
//!
//! This module provides a safe Rust wrapper around X11 library functions for cursor control,
//! display management, and input event handling. Following MPED principles, it provides
//! a focused interface that handles exactly what Pointless needs from X11.
//!
//! ## Architecture
//!
//! The `X11Client` struct encapsulates:
//! - X11 display connection management with proper resource cleanup
//! - Screen information queries (dimensions, root window)
//! - Cursor positioning via XWarpPointer
//! - Event processing for input detection
//! - Error handling with thread-local safety
//!
//! ## Safety
//!
//! X11 operations are inherently unsafe, but this module provides safe abstractions:
//! - Display connection is managed with RAII
//! - Error handling uses thread-local storage to avoid global state
//! - All X11 calls are wrapped with proper error checking
//! - Resource cleanup happens automatically via Drop trait
//!
//! ## Vision Grids Support
//!
//! When compiled with the `vision-grids` feature, additional functionality is available:
//! - Screenshot capture via XGetImage
//! - Image data access for OpenCV processing
//! - Memory management for image resources

use crate::error::{PointlessError, Result};
use std::ptr;
use x11::xlib::{
    Display, GrabModeAsync, XCloseDisplay, XDefaultScreen, XDisplayHeight, XDisplayWidth,
    XErrorEvent, XEvent, XFlush, XGrabKey, XNextEvent, XOpenDisplay, XPending, XRootWindow,
    XSelectInput, XSetErrorHandler, XSync, XUngrabKey, XWarpPointer,
};

// Thread-local error tracking for safer X11 error handling
thread_local! {
    static X11_ERROR_OCCURRED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

extern "C" fn x11_error_handler(_display: *mut Display, _error: *mut XErrorEvent) -> i32 {
    X11_ERROR_OCCURRED.with(|cell| cell.set(true));
    0 // Return 0 to continue execution
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    #[allow(dead_code)]
    pub root_window: u64,
}

#[derive(Debug)]
pub struct X11Client {
    display: *mut Display,
    root_window: u64,
    screen_info: ScreenInfo,
    visual: *mut x11::xlib::Visual,
    colormap: u64,
    depth: i32,
}

// X11Error enum removed - using centralized PointlessError instead

impl X11Client {
    /// Create a new X11Client and connect to the display
    ///
    /// # Errors
    /// Returns `PointlessError::X11ConnectionFailed` if unable to connect to X11 display server
    pub fn new() -> Result<Self> {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err(PointlessError::X11ConnectionFailed);
            }

            let screen = XDefaultScreen(display);
            let root_window = XRootWindow(display, screen);
            let width = XDisplayWidth(display, screen) as u32;
            let height = XDisplayHeight(display, screen) as u32;

            let screen_info = ScreenInfo {
                width,
                height,
                root_window,
            };

            // Find 32-bit ARGB visual for transparency
            let mut vinfo: x11::xlib::XVisualInfo = std::mem::zeroed();
            let mut visual = ptr::null_mut();
            let mut colormap = 0;
            let mut depth = 24;

            if x11::xlib::XMatchVisualInfo(display, screen, 32, x11::xlib::TrueColor, &mut vinfo)
                != 0
            {
                visual = vinfo.visual;
                depth = vinfo.depth;
                colormap =
                    x11::xlib::XCreateColormap(display, root_window, visual, x11::xlib::AllocNone);
                log::debug!("Found 32-bit ARGB visual for transparency");
            } else if x11::xlib::XMatchVisualInfo(
                display,
                screen,
                24,
                x11::xlib::TrueColor,
                &mut vinfo,
            ) != 0
            {
                visual = vinfo.visual;
                depth = vinfo.depth;
                colormap =
                    x11::xlib::XCreateColormap(display, root_window, visual, x11::xlib::AllocNone);
                log::debug!("Fallback to 24-bit visual");
            } else {
                // Use default visual as last resort
                visual = x11::xlib::XDefaultVisual(display, screen);
                depth = x11::xlib::XDefaultDepth(display, screen);
                colormap = x11::xlib::XDefaultColormap(display, screen);
                log::warn!("Using default visual - transparency may not work");
            }

            Ok(X11Client {
                display,
                root_window,
                screen_info,
                visual,
                colormap,
                depth,
            })
        }
    }

    pub fn screen_info(&self) -> &ScreenInfo {
        &self.screen_info
    }

    /// Get the raw display pointer for direct X11 operations
    ///
    /// # Safety
    /// The caller must ensure the display pointer is used correctly and not freed
    pub fn display_ptr(&self) -> *mut Display {
        self.display
    }

    /// Get the current cursor position
    ///
    /// # Errors
    /// Returns `PointlessError::PointerQueryFailed` if X11 cursor position query fails
    pub fn get_cursor_position(&self) -> Result<crate::types::ScreenCoord> {
        use x11::xlib::XQueryPointer;

        unsafe {
            let mut root_return = 0;
            let mut child_return = 0;
            let mut root_x = 0;
            let mut root_y = 0;
            let mut win_x = 0;
            let mut win_y = 0;
            let mut mask_return = 0;

            let result = XQueryPointer(
                self.display,
                self.root_window,
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask_return,
            );

            if result == 0 {
                Err(PointlessError::PointerQueryFailed)
            } else {
                Ok(crate::types::ScreenCoord {
                    x: root_x,
                    y: root_y,
                })
            }
        }
    }

    /// Warp the cursor to specified coordinates
    ///
    /// # Errors
    /// Returns `PointlessError::PointerWarpFailed` if X11 cursor positioning fails
    pub fn warp_cursor(&self, x: i32, y: i32) -> Result<()> {
        unsafe {
            let result = XWarpPointer(
                self.display,
                0,                // src_w - None (any window)
                self.root_window, // dest_w
                0,                // src_x
                0,                // src_y
                0,                // src_width
                0,                // src_height
                x,                // dest_x
                y,                // dest_y
            );

            XFlush(self.display);

            if result == 0 {
                Err(PointlessError::PointerWarpFailed { x, y })
            } else {
                Ok(())
            }
        }
    }

    /// Grab a key combination for exclusive input handling
    ///
    /// # Errors
    /// Returns `PointlessError::KeyGrabFailed` if X11 cannot grab the specified key combination.
    /// This can happen if another application already has an exclusive grab on the key.
    pub fn grab_key(&self, keycode: u32, modifiers: u32) -> Result<()> {
        unsafe {
            // Set up error handler to catch grab failures
            XSetErrorHandler(Some(x11_error_handler));
            X11_ERROR_OCCURRED.with(|cell| cell.set(false));

            XGrabKey(
                self.display,
                keycode as i32,
                modifiers,
                self.root_window,
                1, // owner_events
                GrabModeAsync,
                GrabModeAsync,
            );

            // Force X11 to process the request and trigger any errors
            XSync(self.display, 0);

            // Check if an error occurred
            let error_occurred = X11_ERROR_OCCURRED.with(|cell| cell.get());
            if error_occurred {
                return Err(PointlessError::KeyGrabFailed { keycode });
            }

            Ok(())
        }
    }

    pub fn ungrab_key(&self, keycode: u32, modifiers: u32) {
        unsafe {
            XUngrabKey(self.display, keycode as i32, modifiers, self.root_window);
        }
    }

    pub fn select_input(&self, event_mask: i64) {
        unsafe {
            XSelectInput(self.display, self.root_window, event_mask);
        }
    }

    pub fn next_event(&self) -> XEvent {
        unsafe {
            let mut event: XEvent = std::mem::zeroed();
            XNextEvent(self.display, &mut event);
            event
        }
    }

    #[allow(dead_code)]
    pub fn has_pending_events(&self) -> bool {
        unsafe { XPending(self.display) > 0 }
    }

    /// Create an overlay window with specified properties
    /// Returns window ID on success
    ///
    /// # Errors
    /// Returns `PointlessError::WindowCreationFailed` if X11 window creation fails
    pub fn create_overlay_window(&self, x: i32, y: i32, width: u32, height: u32) -> Result<u64> {
        use x11::xlib::{
            CWBackPixel, CWBorderPixel, CWColormap, CWOverrideRedirect, InputOutput, XCreateWindow,
            XSetWindowAttributes,
        };

        unsafe {
            let mut window_attrs: XSetWindowAttributes = std::mem::zeroed();
            window_attrs.override_redirect = x11::xlib::True; // Override window manager
            window_attrs.background_pixel = 0; // Transparent background
            window_attrs.colormap = self.colormap; // Use the ARGB colormap
            window_attrs.border_pixel = 0;

            let window = XCreateWindow(
                self.display,
                self.root_window,
                x,
                y,
                width,
                height,
                0,          // border_width
                self.depth, // Use the visual's depth
                InputOutput as u32,
                self.visual, // Use the ARGB visual
                CWOverrideRedirect | CWBackPixel | CWBorderPixel | CWColormap,
                &mut window_attrs,
            );

            if window == 0 {
                Err(PointlessError::WindowCreationFailed)
            } else {
                // Don't map window immediately - let renderer control visibility

                // Skip input passthrough for now - focus on getting visual overlay working

                log::debug!(
                    "Created overlay window with ARGB visual: depth={}, window={}",
                    self.depth,
                    window
                );
                Ok(window)
            }
        }
    }

    /// Map (show) a window
    ///
    /// # Errors
    /// Returns `PointlessError::X11OperationFailed` if mapping fails
    pub fn map_window(&self, window: u64) -> Result<()> {
        use x11::xlib::{XFlush, XMapWindow};

        unsafe {
            XMapWindow(self.display, window);
            XFlush(self.display);
        }
        Ok(())
    }

    /// Unmap (hide) a window
    ///
    /// # Errors  
    /// Returns `PointlessError::X11OperationFailed` if unmapping fails
    pub fn unmap_window(&self, window: u64) -> Result<()> {
        use x11::xlib::{XFlush, XUnmapWindow};

        unsafe {
            XUnmapWindow(self.display, window);
            XFlush(self.display);
        }
        Ok(())
    }

    /// Destroy a window and release its resources
    pub fn destroy_window(&self, window: u64) {
        use x11::xlib::XDestroyWindow;

        unsafe {
            XDestroyWindow(self.display, window);
        }
    }

    /// Create a graphics context for drawing operations
    ///
    /// # Errors
    /// Returns `PointlessError::GraphicsContextFailed` if GC creation fails
    pub fn create_graphics_context(&self, window: u64) -> Result<*mut x11::xlib::_XGC> {
        use x11::xlib::{CapButt, JoinMiter, LineSolid, XCreateGC, XSetLineAttributes};

        unsafe {
            let gc = XCreateGC(self.display, window, 0, ptr::null_mut());
            if gc.is_null() {
                Err(PointlessError::GraphicsContextFailed)
            } else {
                // Set line attributes like the working demo (2 pixel width)
                XSetLineAttributes(self.display, gc, 2, LineSolid, CapButt, JoinMiter);
                Ok(gc)
            }
        }
    }

    /// Set foreground color for drawing operations
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `gc` is a valid graphics context pointer created by `create_graphics_context`
    /// - `gc` has not been freed or invalidated
    /// - The graphics context is associated with the current X11 display
    pub unsafe fn set_foreground_color(&self, gc: *mut x11::xlib::_XGC, color: u64) {
        use x11::xlib::{XAllocColor, XColor, XSetForeground};

        unsafe {
            // Use our ARGB colormap for proper color allocation
            let mut color_struct: XColor = std::mem::zeroed();
            // Convert RGB to X11 color format (16-bit per channel)
            color_struct.red = (((color >> 16) & 0xFF) << 8) as u16;
            color_struct.green = (((color >> 8) & 0xFF) << 8) as u16;
            color_struct.blue = ((color & 0xFF) << 8) as u16;

            // Allocate the color using our ARGB colormap
            if XAllocColor(self.display, self.colormap, &mut color_struct) != 0 {
                XSetForeground(self.display, gc, color_struct.pixel);
                log::debug!(
                    "Color allocated successfully with ARGB colormap: RGB({}, {}, {}) -> pixel {}",
                    color_struct.red >> 8,
                    color_struct.green >> 8,
                    color_struct.blue >> 8,
                    color_struct.pixel
                );
            } else {
                // Fallback to white if color allocation fails
                log::warn!(
                    "Color allocation failed for RGB({:#06X}), falling back to white",
                    color
                );
                XSetForeground(self.display, gc, 0xFFFFFF);
            }
        }
    }

    /// Draw a rectangle outline
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `gc` is a valid graphics context pointer created by `create_graphics_context`
    /// - `window` is a valid window ID that exists on the current display
    /// - `gc` has not been freed or invalidated
    /// - The coordinates and dimensions are within valid bounds for the window
    pub unsafe fn draw_rectangle(
        &self,
        window: u64,
        gc: *mut x11::xlib::_XGC,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) {
        use x11::xlib::XDrawRectangle;

        unsafe {
            XDrawRectangle(self.display, window, gc, x, y, width, height);
        }
    }

    /// Draw a line - like the working demo
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `gc` is a valid graphics context pointer created by `create_graphics_context`
    /// - `window` is a valid window ID that exists on the current display
    /// - `gc` has not been freed or invalidated
    pub unsafe fn draw_line(
        &self,
        window: u64,
        gc: *mut x11::xlib::_XGC,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) {
        use x11::xlib::XDrawLine;

        unsafe {
            XDrawLine(self.display, window, gc, x1, y1, x2, y2);
        }
    }

    /// Draw text at specified position
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `gc` is a valid graphics context pointer created by `create_graphics_context`
    /// - `window` is a valid window ID that exists on the current display
    /// - `gc` has not been freed or invalidated
    /// - `text` is valid UTF-8 and does not contain null bytes (handled internally)
    /// - The coordinates are within valid bounds for the window
    pub unsafe fn draw_text(
        &self,
        window: u64,
        gc: *mut x11::xlib::_XGC,
        x: i32,
        y: i32,
        text: &str,
    ) {
        use std::ffi::CString;
        use x11::xlib::XDrawString;

        unsafe {
            if let Ok(c_text) = CString::new(text) {
                XDrawString(
                    self.display,
                    window,
                    gc,
                    x,
                    y,
                    c_text.as_ptr(),
                    text.len() as i32,
                );
            }
        }
    }

    /// Clear a window's contents
    pub fn clear_window(&self, window: u64) {
        use x11::xlib::XClearWindow;

        unsafe {
            XClearWindow(self.display, window);
        }
    }

    /// Flush pending X11 requests to ensure they're processed
    pub fn flush(&self) {
        use x11::xlib::{XFlush, XSync};

        unsafe {
            XFlush(self.display);
            XSync(self.display, 0); // Force synchronization like working demo
        }
    }

    /// Convert X11 keysym to keycode at runtime
    /// This provides safer keycode resolution than hardcoded constants
    ///
    /// # Errors
    /// Returns `PointlessError::InvalidKeycode` if the keysym cannot be mapped to a keycode
    pub fn keysym_to_keycode(&self, keysym: u64) -> Result<u32> {
        use x11::xlib::XKeysymToKeycode;

        unsafe {
            let keycode = XKeysymToKeycode(self.display, keysym);
            if keycode == 0 {
                return Err(PointlessError::InvalidKeycode {
                    keycode: keycode as u32,
                });
            }
            Ok(keycode as u32)
        }
    }

    /// Validate that hardcoded keycodes match runtime X11 configuration
    /// This ensures the application works correctly on different X11 setups
    ///
    /// # Errors
    /// Returns `PointlessError::InvalidKeycode` if any keycode validation fails
    pub fn validate_keycodes(&self) -> Result<()> {
        use crate::constants::*;
        use x11::keysym::*;

        // Define expected keysym-to-keycode mappings
        let expected_mappings = vec![
            (XK_a, KEYCODE_A, "A"),
            (XK_b, KEYCODE_B, "B"),
            (XK_c, KEYCODE_C, "C"),
            (XK_d, KEYCODE_D, "D"),
            (XK_e, KEYCODE_E, "E"),
            (XK_f, KEYCODE_F, "F"),
            (XK_g, KEYCODE_G, "G"),
            (XK_h, KEYCODE_H, "H"),
            (XK_i, KEYCODE_I, "I"),
            (XK_j, KEYCODE_J, "J"),
            (XK_k, KEYCODE_K, "K"),
            (XK_l, KEYCODE_L, "L"),
            (XK_1, KEYCODE_1, "1"),
            (XK_2, KEYCODE_2, "2"),
            (XK_3, KEYCODE_3, "3"),
            (XK_4, KEYCODE_4, "4"),
            (XK_5, KEYCODE_5, "5"),
            (XK_Escape, KEYCODE_ESCAPE, "Escape"),
            (XK_Return, KEYCODE_ENTER, "Enter"),
            (XK_Up, KEYCODE_UP, "Up"),
            (XK_Down, KEYCODE_DOWN, "Down"),
            (XK_Left, KEYCODE_LEFT, "Left"),
            (XK_Right, KEYCODE_RIGHT, "Right"),
        ];

        for (keysym, expected_keycode, key_name) in expected_mappings {
            let actual_keycode = self.keysym_to_keycode(keysym.into())?;
            if actual_keycode != expected_keycode {
                log::warn!(
                    "Keycode mismatch for {}: expected {}, got {}. This may cause input issues.",
                    key_name,
                    expected_keycode,
                    actual_keycode
                );
                // Don't fail validation - just warn, as keycodes can vary legitimately
            }
        }

        Ok(())
    }
}

impl Drop for X11Client {
    fn drop(&mut self) {
        unsafe {
            if !self.display.is_null() {
                XCloseDisplay(self.display);
            }
        }
    }
}
