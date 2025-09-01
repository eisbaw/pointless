use pointless_x11::{Rectangle, ScreenCoord};

/// Grid coordinate system
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridCoord {
    pub x: u32,
    pub y: u32,
}

/// Grid cell definition
#[derive(Debug, Clone)]
pub struct GridCell {
    pub coord: GridCoord,
    pub screen_rect: Rectangle,
    pub label: String,
}

impl GridCell {
    pub fn new(coord: GridCoord, screen_rect: Rectangle, label: String) -> Self {
        Self {
            coord,
            screen_rect,
            label,
        }
    }

    pub fn center(&self) -> ScreenCoord {
        self.screen_rect.center()
    }
}

/// Key code for input handling
pub type KeyCode = u32;