use crate::error::{PointlessError, Result};
use std::ptr;
use x11::xlib::{
    Display, GrabModeAsync, XCloseDisplay, XDefaultScreen, XDisplayHeight, XDisplayWidth,
    XErrorEvent, XEvent, XFlush, XGrabKey, XNextEvent, XOpenDisplay, XPending, XRootWindow,
    XSelectInput, XSetErrorHandler, XSync, XUngrabKey, XWarpPointer,
};

#[cfg(feature = "vision-grids")]
use x11::xlib::{XAllPlanes, XDestroyImage, XGetImage, ZPixmap};

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

    pub fn has_pending_events(&self) -> bool {
        unsafe { XPending(self.display) > 0 }
    }

    pub fn display(&self) -> *mut Display {
        self.display
    }

    pub fn root_window(&self) -> u64 {
        self.root_window
    }

    #[cfg(feature = "vision-grids")]
    pub fn take_screenshot(&self) -> Result<Vec<u8>> {
        unsafe {
            let image = XGetImage(
                self.display,
                self.root_window,
                0,
                0,
                self.screen_info.width,
                self.screen_info.height,
                XAllPlanes(),
                ZPixmap,
            );

            if image.is_null() {
                return Err(PointlessError::X11ScreenshotFailed);
            }

            let image_ref = &*image;
            let data_len = (image_ref.bytes_per_line * image_ref.height) as usize;
            
            // Validate data length to prevent buffer overruns
            if data_len == 0 || data_len > (self.screen_info.width * self.screen_info.height * 4) as usize {
                XDestroyImage(image);
                return Err(PointlessError::X11ScreenshotFailed);
            }
            
            // Use safe slice operations instead of raw pointer arithmetic
            let src_ptr = image_ref.data as *const u8;
            let src_slice = std::slice::from_raw_parts(src_ptr, data_len);
            let screenshot_data = src_slice.to_vec();

            XDestroyImage(image);
            Ok(screenshot_data)
        }
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
