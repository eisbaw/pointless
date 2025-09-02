use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PointlessError {
    #[error("X11 connection failed")]
    X11ConnectionFailed,

    #[error("X11 operation failed: {operation}")]
    #[allow(dead_code)]
    X11OperationFailed { operation: String },

    #[error("Key grab failed for keycode {keycode}")]
    KeyGrabFailed { keycode: u32 },

    #[error("Pointer warp failed to position ({x}, {y})")]
    PointerWarpFailed { x: i32, y: i32 },

    #[error("Window creation failed")]
    WindowCreationFailed,

    #[error("Graphics context creation failed")]
    GraphicsContextFailed,

    #[error("Invalid color format: {color}. Expected hex format like 'FFFF00'")]
    InvalidColorFormat { color: String },

    #[error("Invalid keycode: {keycode}")]
    InvalidKeycode { keycode: u32 },

    #[error("Modifier key not found: {key_name}")]
    ModifierKeyNotFound { key_name: String },

    #[error("Invalid grid density: {density}. Must be between 1 and 10")]
    InvalidGridDensity { density: u32 },

    #[error("Invalid opacity: {opacity}. Must be between 0.0 and 1.0")]
    InvalidOpacity { opacity: f32 },

    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(String),

    #[error("Rendering failed: {0}")]
    RenderingFailed(String),
}

pub type Result<T> = std::result::Result<T, PointlessError>;
