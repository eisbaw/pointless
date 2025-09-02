use thiserror::Error;

pub type X11Result<T> = Result<T, X11Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum X11Error {
    #[error("Failed to connect to X11 display")]
    ConnectionFailed,

    #[error("Failed to warp cursor to position ({x}, {y})")]
    PointerWarpFailed { x: i32, y: i32 },

    #[error("Failed to grab key: {keycode} with modifiers: {modifiers}")]
    KeyGrabFailed { keycode: u32, modifiers: u32 },

    #[error("Failed to ungrab key: {keycode} with modifiers: {modifiers}")]
    KeyUngrabFailed { keycode: u32, modifiers: u32 },

    #[error("X11 operation failed: {operation}")]
    OperationFailed { operation: String },

    #[cfg(feature = "vision-grids")]
    #[error("Screenshot capture failed")]
    ScreenshotCaptureFailed,
}
