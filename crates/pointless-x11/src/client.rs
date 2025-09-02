use crate::error::{X11Error, X11Result};
use crate::types::ScreenInfo;
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

/// X11 client for managing display connection and basic operations
#[derive(Debug)]
pub struct X11Client {
    display: *mut Display,
    root_window: u64,
    screen_info: ScreenInfo,
}

// SAFETY: X11Client can be safely sent between threads because:
// 1. The X11 Display pointer (*mut Display) is only accessed through XLib functions
// 2. XLib functions are thread-safe when each thread has its own Display connection
// 3. This X11Client owns its Display connection exclusively - no sharing between instances
// 4. The XLib documentation states that Display connections can be used from different
//    threads as long as they are not used concurrently (which Rust's Send ensures)
// 5. When X11Client is moved to another thread, all subsequent X11 operations will
//    occur on that thread, maintaining XLib's threading requirements
// 6. The Drop implementation properly closes the Display connection on the owning thread
//
// IMPORTANT: This assumes that X11Client instances are not used concurrently from
// multiple threads. For concurrent access, use Arc<Mutex<X11Client>> instead.
unsafe impl Send for X11Client {}

impl X11Client {
    /// Create a new X11 client connection
    pub fn new() -> X11Result<Self> {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err(X11Error::ConnectionFailed);
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

    /// Get screen information
    pub fn screen_info(&self) -> &ScreenInfo {
        &self.screen_info
    }

    /// Get the raw display pointer (for use by other libraries)
    pub fn display(&self) -> *mut Display {
        self.display
    }

    /// Get the root window
    pub fn root_window(&self) -> u64 {
        self.root_window
    }

    /// Warp the cursor to the specified position
    pub fn warp_cursor(&mut self, x: i32, y: i32) -> X11Result<()> {
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
                Err(X11Error::PointerWarpFailed { x, y })
            } else {
                Ok(())
            }
        }
    }

    /// Grab a key with modifiers
    pub fn grab_key(&self, keycode: u32, modifiers: u32) -> X11Result<()> {
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

            XSync(self.display, 0);

            let error_occurred = X11_ERROR_OCCURRED.with(|cell| cell.get());
            if error_occurred {
                Err(X11Error::KeyGrabFailed { keycode, modifiers })
            } else {
                log::debug!(
                    "Successfully grabbed key {} with modifiers {}",
                    keycode,
                    modifiers
                );
                Ok(())
            }
        }
    }

    /// Ungrab a key with modifiers
    pub fn ungrab_key(&self, keycode: u32, modifiers: u32) -> X11Result<()> {
        unsafe {
            XSetErrorHandler(Some(x11_error_handler));
            X11_ERROR_OCCURRED.with(|cell| cell.set(false));

            XUngrabKey(self.display, keycode as i32, modifiers, self.root_window);

            XSync(self.display, 0);

            let error_occurred = X11_ERROR_OCCURRED.with(|cell| cell.get());
            if error_occurred {
                Err(X11Error::KeyUngrabFailed { keycode, modifiers })
            } else {
                log::debug!(
                    "Successfully ungrabbed key {} with modifiers {}",
                    keycode,
                    modifiers
                );
                Ok(())
            }
        }
    }

    /// Check if there are pending events
    pub fn has_pending_events(&self) -> bool {
        unsafe { XPending(self.display) > 0 }
    }

    /// Get the next event (blocking)
    pub fn next_event(&self) -> X11Result<XEvent> {
        unsafe {
            let mut event = std::mem::zeroed();
            let result = XNextEvent(self.display, &mut event);
            if result == 0 {
                Ok(event)
            } else {
                Err(X11Error::OperationFailed {
                    operation: "XNextEvent".to_string(),
                })
            }
        }
    }

    /// Set input event mask for a window
    pub fn select_input(&self, window: u64, event_mask: i64) -> X11Result<()> {
        unsafe {
            let result = XSelectInput(self.display, window, event_mask);
            if result == 0 {
                Err(X11Error::OperationFailed {
                    operation: "XSelectInput".to_string(),
                })
            } else {
                Ok(())
            }
        }
    }

    /// Take a screenshot (vision-grids feature)
    #[cfg(feature = "vision-grids")]
    pub fn take_screenshot(&self) -> X11Result<Vec<u8>> {
        unsafe {
            let image = XGetImage(
                self.display,
                self.root_window,
                0, // x
                0, // y
                self.screen_info.width,
                self.screen_info.height,
                XAllPlanes(),
                ZPixmap,
            );

            if image.is_null() {
                return Err(X11Error::ScreenshotCaptureFailed);
            }

            // Convert XImage to RGB data
            let bytes_per_pixel = 4; // Assuming RGBA
                                     // Use checked arithmetic to prevent overflow on very large screens
            let total_bytes = self
                .screen_info
                .width
                .checked_mul(self.screen_info.height)
                .and_then(|size| size.checked_mul(bytes_per_pixel))
                .map(|size| size as usize)
                .ok_or(X11Error::ScreenshotCaptureFailed)?;
            let mut rgb_data = Vec::with_capacity(total_bytes);

            let data_ptr = (*image).data as *const u8;
            for i in 0..total_bytes {
                rgb_data.push(*data_ptr.add(i));
            }

            XDestroyImage(image);
            Ok(rgb_data)
        }
    }
}

impl Drop for X11Client {
    fn drop(&mut self) {
        unsafe {
            if !self.display.is_null() {
                XCloseDisplay(self.display);
                self.display = ptr::null_mut();
            }
        }
    }
}
