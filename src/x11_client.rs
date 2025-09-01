use crate::error::{PointlessError, Result};
use std::ptr;
use x11::xlib::{
    Display, GrabModeAsync, KeyPress, KeyPressMask, KeyRelease, KeyReleaseMask,
    XCloseDisplay, XDefaultScreen, XDisplayHeight, XDisplayWidth, XEvent, XFlush,
    XGrabKey, XNextEvent, XOpenDisplay, XRootWindow, XSelectInput, XUngrabKey, XWarpPointer,
};

#[derive(Debug)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    pub root_window: u64,
}

#[derive(Debug)]
pub struct X11Client {
    display: *mut Display,
    screen: i32,
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
                screen,
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
            // X11 XGrabKey can fail but doesn't return error codes
            // We need to check via XSync or similar for proper error handling
            XGrabKey(
                self.display,
                keycode as i32,
                modifiers,
                self.root_window,
                1, // owner_events
                GrabModeAsync,
                GrabModeAsync,
            );

            // For now return success, but this is a known issue
            // Real implementation should use XSync and error handler
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
