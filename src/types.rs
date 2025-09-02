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
    #[must_use]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Safe conversion of width to i32 with overflow checking
    pub fn width_as_i32(&self) -> Result<i32, crate::error::PointlessError> {
        i32::try_from(self.width).map_err(|_| {
            crate::error::PointlessError::InvalidCoordinate(format!(
                "width {} exceeds i32::MAX",
                self.width
            ))
        })
    }

    /// Safe conversion of height to i32 with overflow checking  
    pub fn height_as_i32(&self) -> Result<i32, crate::error::PointlessError> {
        i32::try_from(self.height).map_err(|_| {
            crate::error::PointlessError::InvalidCoordinate(format!(
                "height {} exceeds i32::MAX",
                self.height
            ))
        })
    }

    #[must_use]
    pub fn center(&self) -> ScreenCoord {
        // Use safe conversion - in practice this cannot fail for display dimensions
        // but maintains architectural honesty about potential overflow
        let width_i32 = self.width_as_i32().unwrap_or_else(|_| {
            log::error!(
                "Rectangle width {} exceeds i32::MAX, using fallback",
                self.width
            );
            i32::MAX
        });
        let height_i32 = self.height_as_i32().unwrap_or_else(|_| {
            log::error!(
                "Rectangle height {} exceeds i32::MAX, using fallback",
                self.height
            );
            i32::MAX
        });

        ScreenCoord {
            x: self.x + width_i32 / 2,
            y: self.y + height_i32 / 2,
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, point: ScreenCoord) -> bool {
        let width_i32 = self.width_as_i32().unwrap_or(i32::MAX);
        let height_i32 = self.height_as_i32().unwrap_or(i32::MAX);

        point.x >= self.x
            && point.x < self.x + width_i32
            && point.y >= self.y
            && point.y < self.y + height_i32
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
    #[must_use]
    pub fn new(coord: GridCoord, screen_rect: Rectangle, label: String) -> Self {
        Self {
            coord,
            screen_rect,
            label,
        }
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn center(&self) -> ScreenCoord {
        self.screen_rect.center()
    }
}

/// Key code for input handling
pub type KeyCode = u32;
