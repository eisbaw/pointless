use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum GridType {
    Homogeneous,
    Qwerty,
    Arrow,
    Vim,
    #[cfg(feature = "vision-grids")]
    HoughLines,
    #[cfg(feature = "vision-grids")]
    RectangleFitting,
    #[cfg(feature = "vision-grids")]
    EdgeDensity,
    #[cfg(feature = "vision-grids")]
    ColorClustering,
    #[cfg(feature = "vision-grids")]
    FeatureDetection,
}

#[derive(Debug, Clone, ValueEnum)]
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
    #[arg(long, default_value_t = 0.8)]
    pub opacity: f32,

    /// Debug mode for additional logging
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}

impl Config {
    pub fn color_rgb(&self) -> crate::error::Result<u32> {
        // Parse hex color string to RGB value - fail fast on invalid input
        u32::from_str_radix(&self.color, 16)
            .map_err(|_| crate::error::PointlessError::InvalidColorFormat {
                color: self.color.clone(),
            })
    }
    
    pub fn validate(&self) -> crate::error::Result<()> {
        // Validate all configuration parameters
        self.color_rgb()?; // Ensure color is valid
        
        if self.grid_density == 0 || self.grid_density > 10 {
            return Err(crate::error::PointlessError::InvalidColorFormat {
                color: format!("grid_density must be 1-10, got {}", self.grid_density),
            });
        }
        
        if !(0.0..=1.0).contains(&self.opacity) {
            return Err(crate::error::PointlessError::InvalidColorFormat {
                color: format!("opacity must be 0.0-1.0, got {}", self.opacity),
            });
        }
        
        Ok(())
    }
}
