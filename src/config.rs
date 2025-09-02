use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
#[non_exhaustive]
pub enum GridType {
    Homogeneous,
    Qwerty,
    Arrow,
    Vim,
}

#[derive(Debug, Clone, ValueEnum)]
#[non_exhaustive]
pub enum ModifierKey {
    Super,
    Alt,
    Ctrl,
}

#[derive(Debug, Clone, Parser)]
#[command(name = "pointless")]
#[command(about = "A keyboard-driven X11 cursor utility that makes pointing devices pointless")]
pub struct Config {
    /// Grid type to use
    #[arg(long, default_value = "homogeneous")]
    pub grid_type: GridType,

    /// Modifier key for activation
    #[arg(long, default_value = "super")]
    pub modifier_key: ModifierKey,

    /// Grid density (subdivision levels)
    #[arg(long, default_value_t = 3)]
    pub grid_density: u32,

    /// Grid color in RGB hex format
    #[arg(long, default_value = "FFFF00")]
    pub color: String,

    /// Grid opacity (0.0 to 1.0)
    #[arg(long, default_value_t = crate::constants::DEFAULT_OPACITY)]
    pub opacity: f32,

    /// Debug mode for additional logging
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Path to PNG output file instead of X11 display
    #[arg(long)]
    pub png_output: Option<String>,

    /// PNG resolution in WIDTHxHEIGHT format
    #[arg(long, default_value = "1920x1080")]
    pub png_resolution: String,
}

impl Config {
    /// Parse color string as RGB hex value
    ///
    /// # Errors
    /// Returns `PointlessError::InvalidColorFormat` if color string is not a valid hex value
    pub fn color_rgb(&self) -> crate::error::Result<u32> {
        // Parse hex color string to RGB value - fail fast on invalid input
        u32::from_str_radix(&self.color, 16).map_err(|_| {
            crate::error::PointlessError::InvalidColorFormat {
                color: self.color.clone(),
            }
        })
    }

    /// Get color as RGB tuple for PNG rendering
    ///
    /// # Errors
    /// Returns `PointlessError::InvalidColorFormat` if color string is not a valid hex value
    pub fn color_as_rgb_tuple(&self) -> crate::error::Result<(u8, u8, u8)> {
        let rgb = self.color_rgb()?;
        Ok((
            ((rgb >> 16) & 0xFF) as u8,
            ((rgb >> 8) & 0xFF) as u8,
            (rgb & 0xFF) as u8,
        ))
    }

    /// Parse PNG resolution string as (width, height)
    ///
    /// # Errors
    /// Returns `PointlessError::InvalidCoordinate` if resolution format is invalid
    pub fn parse_png_resolution(&self) -> crate::error::Result<(u32, u32)> {
        let parts: Vec<&str> = self.png_resolution.split('x').collect();
        if parts.len() != 2 {
            return Err(crate::error::PointlessError::InvalidCoordinate(format!(
                "Invalid resolution format '{}'. Expected WIDTHxHEIGHT (e.g., 1920x1080)",
                self.png_resolution
            )));
        }

        let width: u32 = parts[0].parse().map_err(|_| {
            crate::error::PointlessError::InvalidCoordinate(format!(
                "Invalid width '{}' in resolution '{}'",
                parts[0], self.png_resolution
            ))
        })?;

        let height: u32 = parts[1].parse().map_err(|_| {
            crate::error::PointlessError::InvalidCoordinate(format!(
                "Invalid height '{}' in resolution '{}'",
                parts[1], self.png_resolution
            ))
        })?;

        Ok((width, height))
    }

    /// Validate all configuration parameters
    ///
    /// # Errors
    /// Returns `PointlessError::InvalidColorFormat` for invalid color format,
    /// `PointlessError::InvalidGridDensity` for invalid density values,
    /// or `PointlessError::InvalidOpacity` for opacity outside 0.0-1.0 range
    pub fn validate(&self) -> crate::error::Result<()> {
        // Validate all configuration parameters
        self.color_rgb()?; // Ensure color is valid

        if self.grid_density == 0 || self.grid_density > 10 {
            return Err(crate::error::PointlessError::InvalidGridDensity {
                density: self.grid_density,
            });
        }

        if !(0.0..=1.0).contains(&self.opacity) {
            return Err(crate::error::PointlessError::InvalidOpacity {
                opacity: self.opacity,
            });
        }

        Ok(())
    }
}
