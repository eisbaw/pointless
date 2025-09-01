/// Grid coordinate system
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridCoord {
    pub x: u32,
    pub y: u32,
}

/// Screen pixel coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenCoord {
    pub x: i32,
    pub y: i32,
}

/// Rectangle definition
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

    #[allow(dead_code)]
    pub fn contains(&self, point: ScreenCoord) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
    }
}

/// Grid cell definition
#[derive(Debug, Clone)]
pub struct GridCell {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn center(&self) -> ScreenCoord {
        self.screen_rect.center()
    }
}

/// Key code for input handling
pub type KeyCode = u32;
