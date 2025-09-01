use thiserror::Error;

#[derive(Debug, Error)]
pub enum PointlessError {
    #[error("X11 connection failed")]
    X11ConnectionFailed,
    
    #[error("X11 operation failed: {operation}")]
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
}

pub type Result<T> = std::result::Result<T, PointlessError>;

impl From<std::num::ParseIntError> for PointlessError {
    fn from(_: std::num::ParseIntError) -> Self {
        PointlessError::InvalidColorFormat {
            color: "unknown".to_string(),
        }
    }
}