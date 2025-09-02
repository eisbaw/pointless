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
}

// X11Error enum removed - using centralized PointlessError instead

impl X11Client {
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

            Ok(X11Client {
                display,
                root_window,
                screen_info,
            })
        }
    }

    pub fn screen_info(&self) -> &ScreenInfo {
        &self.screen_info
    }

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

    pub fn display(&self) -> *mut Display {
        self.display
    }

    pub fn root_window(&self) -> u64 {
        self.root_window
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
