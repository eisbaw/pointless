pub mod client;
pub mod error;
pub mod types;

pub use client::X11Client;
pub use error::{X11Error, X11Result};
pub use types::{Rectangle, ScreenCoord, ScreenInfo};
