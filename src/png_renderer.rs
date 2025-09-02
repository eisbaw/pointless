use crate::config::Config;
use crate::error::{PointlessError, Result};
use crate::types::GridCell;
use crate::x11_client::ScreenInfo;
use image::{ImageBuffer, Rgb, RgbImage};
use std::path::Path;

pub struct PngRenderer;

impl PngRenderer {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Render grid cells to a PNG file
    ///
    /// # Errors
    /// Returns `PointlessError::RenderingFailed` if PNG creation or file writing fails
    pub fn render_grid_to_png(
        &self,
        cells: &[GridCell],
        screen_info: &ScreenInfo,
        config: &Config,
        output_path: &str,
    ) -> Result<()> {
        // Create a black background image
        let mut img: RgbImage = ImageBuffer::new(screen_info.width, screen_info.height);

        // Fill with black background
        for pixel in img.pixels_mut() {
            *pixel = Rgb([0, 0, 0]);
        }

        // Parse color from config
        let (red, green, blue) = config.color_as_rgb_tuple()?;
        let grid_color = Rgb([red, green, blue]);

        // Draw each grid cell
        for cell in cells {
            let rect = &cell.screen_rect;

            // Validate coordinates are within image bounds
            if rect.x < 0 || rect.y < 0 {
                continue;
            }

            let x_start = rect.x as u32;
            let y_start = rect.y as u32;
            let x_end =
                (rect.x + rect.width_as_i32().unwrap_or(0)).min(screen_info.width as i32) as u32;
            let y_end =
                (rect.y + rect.height_as_i32().unwrap_or(0)).min(screen_info.height as i32) as u32;

            // Draw rectangle border (1-pixel yellow lines as per spec)
            // Top border
            if y_start < screen_info.height {
                for x in x_start..x_end.min(screen_info.width) {
                    if let Some(pixel) = img.get_pixel_mut_checked(x, y_start) {
                        *pixel = grid_color;
                    }
                }
            }

            // Bottom border
            if y_end > 0 && y_end - 1 < screen_info.height {
                for x in x_start..x_end.min(screen_info.width) {
                    if let Some(pixel) = img.get_pixel_mut_checked(x, y_end - 1) {
                        *pixel = grid_color;
                    }
                }
            }

            // Left border
            if x_start < screen_info.width {
                for y in y_start..y_end.min(screen_info.height) {
                    if let Some(pixel) = img.get_pixel_mut_checked(x_start, y) {
                        *pixel = grid_color;
                    }
                }
            }

            // Right border
            if x_end > 0 && x_end - 1 < screen_info.width {
                for y in y_start..y_end.min(screen_info.height) {
                    if let Some(pixel) = img.get_pixel_mut_checked(x_end - 1, y) {
                        *pixel = grid_color;
                    }
                }
            }

            // Draw cell label at center (simple text rendering)
            let center = rect.center();
            if center.x >= 0 && center.y >= 0 {
                self.render_text(
                    &mut img,
                    &cell.label,
                    center.x as u32,
                    center.y as u32,
                    grid_color,
                    screen_info,
                );
            }
        }

        // Save PNG file
        img.save(Path::new(output_path))
            .map_err(|e| PointlessError::RenderingFailed(format!("Failed to save PNG: {}", e)))?;

        Ok(())
    }

    /// Render simple text at the given position
    /// This is a basic bitmap font implementation for labels
    fn render_text(
        &self,
        img: &mut RgbImage,
        text: &str,
        center_x: u32,
        center_y: u32,
        color: Rgb<u8>,
        screen_info: &ScreenInfo,
    ) {
        // Simple 5x7 bitmap font for basic characters
        let char_width = 5_u32;
        let char_height = 7_u32;
        let char_spacing = 1_u32;

        let total_width =
            (text.len() as u32 * (char_width + char_spacing)).saturating_sub(char_spacing);
        let start_x = center_x.saturating_sub(total_width / 2);
        let start_y = center_y.saturating_sub(char_height / 2);

        for (i, ch) in text.chars().enumerate() {
            let char_x = start_x + i as u32 * (char_width + char_spacing);
            self.render_char(img, ch, char_x, start_y, color, screen_info);
        }
    }

    /// Render a single character using a simple bitmap font
    fn render_char(
        &self,
        img: &mut RgbImage,
        ch: char,
        x: u32,
        y: u32,
        color: Rgb<u8>,
        screen_info: &ScreenInfo,
    ) {
        // Simple 5x7 bitmap patterns for common characters
        let pattern = match ch {
            'A' => [
                0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
            ],
            'B' => [
                0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
            ],
            'C' => [
                0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111,
            ],
            'D' => [
                0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
            ],
            '1' => [
                0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
            ],
            '2' => [
                0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111,
            ],
            '3' => [
                0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110,
            ],
            '4' => [
                0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
            ],
            _ => [
                0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111,
            ], // Default box pattern
        };

        // Draw the character
        for (row, &pattern_row) in pattern.iter().enumerate() {
            for col in 0..5 {
                if (pattern_row >> (4 - col)) & 1 == 1 {
                    let pixel_x = x + col;
                    let pixel_y = y + row as u32;

                    if pixel_x < screen_info.width && pixel_y < screen_info.height {
                        if let Some(pixel) = img.get_pixel_mut_checked(pixel_x, pixel_y) {
                            *pixel = color;
                        }
                    }
                }
            }
        }
    }
}

impl Default for PngRenderer {
    fn default() -> Self {
        Self::new()
    }
}
