/// Screen information from X11 display
#[derive(Debug, Clone, Copy)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    pub root_window: u64,
}

/// Screen pixel coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenCoord {
    pub x: i32,
    pub y: i32,
}

impl ScreenCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Rectangle definition for screen areas
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center(&self) -> ScreenCoord {
        ScreenCoord {
            x: self.x + (self.width as i32) / 2,
            y: self.y + (self.height as i32) / 2,
        }
    }

    pub fn contains(&self, point: ScreenCoord) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
    }
}
