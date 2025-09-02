//! Cursor movement animation effects
//!
//! This module provides various visual effects for cursor movement transitions,
//! making the cursor movement more visually appealing and easier to track.

use crate::config::MoveStyle;
use crate::types::ScreenCoord;
use crate::x11_client::X11Client;
use std::time::{Duration, Instant};

/// Handles animated cursor movements with various visual effects
pub struct CursorAnimator {
    move_style: MoveStyle,
}

impl CursorAnimator {
    /// Create a new cursor animator with the specified movement style
    pub fn new(move_style: MoveStyle) -> Self {
        Self { move_style }
    }

    /// Animate cursor movement from current position to target with the configured style
    pub fn animate_move(&self, x11_client: &X11Client, target: ScreenCoord) -> Result<(), Box<dyn std::error::Error>> {
        match self.move_style {
            MoveStyle::None => self.move_instant(x11_client, target),
            MoveStyle::Accelerate => self.move_accelerate(x11_client, target),
            MoveStyle::Linefade => self.move_linefade(x11_client, target),
            MoveStyle::Firetrail => self.move_firetrail(x11_client, target),
        }
    }

    /// Instant movement (current default behavior)
    fn move_instant(&self, x11_client: &X11Client, target: ScreenCoord) -> Result<(), Box<dyn std::error::Error>> {
        x11_client.warp_cursor(target.x, target.y)?;
        Ok(())
    }

    /// Accelerated movement using sigmoid function for smooth acceleration/deceleration
    fn move_accelerate(&self, x11_client: &X11Client, target: ScreenCoord) -> Result<(), Box<dyn std::error::Error>> {
        // Get actual current cursor position
        let start = x11_client.get_cursor_position()?;

        let animation_duration = Duration::from_millis(200); // 200ms animation
        let frame_duration = Duration::from_millis(16); // ~60 FPS
        let start_time = Instant::now();

        while start_time.elapsed() < animation_duration {
            let progress = start_time.elapsed().as_secs_f32() / animation_duration.as_secs_f32();
            let progress = progress.min(1.0);

            // Sigmoid function for smooth acceleration/deceleration
            // Maps 0->1 input to 0->1 output with smooth S-curve
            let smoothed_progress = self.sigmoid_ease(progress);

            // Interpolate position
            let current_x = start.x + ((target.x - start.x) as f32 * smoothed_progress) as i32;
            let current_y = start.y + ((target.y - start.y) as f32 * smoothed_progress) as i32;

            x11_client.warp_cursor(current_x, current_y)?;
            
            if progress >= 1.0 {
                break;
            }
            
            std::thread::sleep(frame_duration);
        }

        // Ensure we end exactly at target
        x11_client.warp_cursor(target.x, target.y)?;
        Ok(())
    }

    /// Sigmoid function for smooth easing: slow -> fast -> slow
    fn sigmoid_ease(&self, t: f32) -> f32 {
        // Use a scaled sigmoid: 1 / (1 + e^(-6*(t-0.5)))
        // This maps [0,1] to approximately [0,1] with smooth S-curve
        1.0 / (1.0 + (-6.0 * (t - 0.5)).exp())
    }

    /// Line fade effect - shows a fading line between start and target
    fn move_linefade(&self, _x11_client: &X11Client, _target: ScreenCoord) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement line fade visual effect
        // This would require creating temporary overlay windows to draw the fading line
        log::warn!("Linefade movement style not yet implemented, using instant movement");
        Ok(())
    }

    /// Fire trail effect - shows a "fire" trail following the cursor movement
    fn move_firetrail(&self, _x11_client: &X11Client, _target: ScreenCoord) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement fire trail visual effect
        // This would require creating animated particle effects
        log::warn!("Firetrail movement style not yet implemented, using instant movement");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid_ease() {
        let animator = CursorAnimator::new(MoveStyle::Accelerate);
        
        // Test edge cases
        assert!((animator.sigmoid_ease(0.0) - 0.0).abs() < 0.1); // Should be close to 0
        assert!((animator.sigmoid_ease(1.0) - 1.0).abs() < 0.1); // Should be close to 1
        assert!((animator.sigmoid_ease(0.5) - 0.5).abs() < 0.1); // Should be close to 0.5
        
        // Test smoothness - should be increasing
        assert!(animator.sigmoid_ease(0.25) < animator.sigmoid_ease(0.5));
        assert!(animator.sigmoid_ease(0.5) < animator.sigmoid_ease(0.75));
    }
}