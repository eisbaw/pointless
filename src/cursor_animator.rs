//! Cursor movement animation effects
//!
//! This module provides various visual effects for cursor movement transitions,
//! making the cursor movement more visually appealing and easier to track.

use crate::config::MoveStyle;
use crate::types::ScreenCoord;
use crate::x11_client::X11Client;
use std::time::{Duration, Instant};
use rand::Rng;

/// Represents a single fire particle
#[derive(Debug, Clone)]
struct FireParticle {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    age: f32,        // 0.0 to 1.0
    max_age: f32,    // Maximum age in seconds
}

impl FireParticle {
    /// Create a new fire particle at the given position
    fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        
        Self {
            x,
            y,
            velocity_x: rng.gen_range(-30.0..30.0), // Random horizontal spread
            velocity_y: rng.gen_range(-80.0..-20.0), // Upward movement (negative Y)
            age: 0.0,
            max_age: rng.gen_range(0.3..0.8), // Particles live 0.3-0.8 seconds
        }
    }
    
    /// Update particle physics
    fn update(&mut self, dt: f32) {
        self.x += self.velocity_x * dt;
        self.y += self.velocity_y * dt;
        
        // Add some gravity/air resistance
        self.velocity_y += 50.0 * dt; // Slight gravity
        self.velocity_x *= 0.98; // Air resistance
        
        self.age += dt;
    }
    
    /// Check if particle is still alive
    fn is_alive(&self) -> bool {
        self.age < self.max_age
    }
    
    /// Get the current color based on age (fire gradient)
    fn get_color(&self) -> (u8, u8, u8) {
        let life_ratio = self.age / self.max_age;
        
        // Fire color gradient: white/yellow -> orange -> red -> dark red
        match life_ratio {
            t if t < 0.2 => {
                // Hot white/yellow core
                let mix = t / 0.2;
                (255, 255, (255.0 * (1.0 - mix * 0.3)) as u8)
            }
            t if t < 0.5 => {
                // Orange phase  
                let mix = (t - 0.2) / 0.3;
                (255, (255.0 * (1.0 - mix * 0.5)) as u8, 0)
            }
            t if t < 0.8 => {
                // Red phase
                let mix = (t - 0.5) / 0.3;
                ((255.0 * (1.0 - mix * 0.3)) as u8, 0, 0)
            }
            _ => {
                // Dark red/fade out
                let mix = (life_ratio - 0.8) / 0.2;
                ((180.0 * (1.0 - mix)) as u8, 0, 0)
            }
        }
    }
}

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
    fn move_firetrail(&self, x11_client: &X11Client, target: ScreenCoord) -> Result<(), Box<dyn std::error::Error>> {
        // Get actual current cursor position
        let start = x11_client.get_cursor_position()?;
        
        // Create overlay window for fire particles
        let overlay_window = x11_client.create_overlay_window(0, 0, 
                                                            x11_client.screen_info().width, 
                                                            x11_client.screen_info().height)?;
        let gc = x11_client.create_graphics_context(overlay_window)?;
        
        // Map the overlay window
        x11_client.map_window(overlay_window)?;
        
        let mut particles = Vec::new();
        let animation_duration = Duration::from_millis(400); // 400ms for fire trail
        let frame_duration = Duration::from_millis(16); // ~60 FPS
        let start_time = Instant::now();
        
        
        // Move cursor with animated fire trail
        while start_time.elapsed() < animation_duration {
            let progress = start_time.elapsed().as_secs_f32() / animation_duration.as_secs_f32();
            let progress = progress.min(1.0);
            
            // Smoothed cursor movement (using sigmoid)
            let smoothed_progress = self.sigmoid_ease(progress);
            let current_x = start.x + ((target.x - start.x) as f32 * smoothed_progress) as i32;
            let current_y = start.y + ((target.y - start.y) as f32 * smoothed_progress) as i32;
            
            // Move cursor
            x11_client.warp_cursor(current_x, current_y)?;
            
            // Spawn new fire particles along the path
            if progress < 0.8 { // Stop spawning particles near the end
                for _ in 0..3 { // Spawn 3 particles per frame
                    particles.push(FireParticle::new(current_x as f32, current_y as f32));
                }
            }
            
            // Update existing particles
            let dt = frame_duration.as_secs_f32();
            for particle in &mut particles {
                particle.update(dt);
            }
            
            // Remove dead particles
            particles.retain(|p| p.is_alive());
            
            // Clear and draw particles
            x11_client.clear_window(overlay_window);
            
            
            for particle in &particles {
                let (r, g, b) = particle.get_color();
                let color = ((r as u64) << 16) | ((g as u64) << 8) | (b as u64);
                
                unsafe {
                    x11_client.set_foreground_color(gc, color);
                    // Draw particle as a small rectangle (3x3 pixels)
                    x11_client.draw_rectangle(overlay_window, gc, 
                                            particle.x as i32, particle.y as i32, 3, 3);
                }
            }
            
            x11_client.flush();
            
            if progress >= 1.0 {
                break;
            }
            
            std::thread::sleep(frame_duration);
        }
        
        // Ensure cursor ends at target
        x11_client.warp_cursor(target.x, target.y)?;
        
        // Let remaining particles finish their animation
        let fade_duration = Duration::from_millis(500);
        let fade_start = Instant::now();
        
        while fade_start.elapsed() < fade_duration && !particles.is_empty() {
            let dt = frame_duration.as_secs_f32();
            
            // Update particles
            for particle in &mut particles {
                particle.update(dt);
            }
            
            // Remove dead particles
            particles.retain(|p| p.is_alive());
            
            // Draw remaining particles
            x11_client.clear_window(overlay_window);
            
            for particle in &particles {
                let (r, g, b) = particle.get_color();
                let color = ((r as u64) << 16) | ((g as u64) << 8) | (b as u64);
                
                unsafe {
                    x11_client.set_foreground_color(gc, color);
                    x11_client.draw_rectangle(overlay_window, gc, 
                                            particle.x as i32, particle.y as i32, 3, 3);
                }
            }
            
            x11_client.flush();
            std::thread::sleep(frame_duration);
        }
        
        // Clean up overlay
        x11_client.destroy_window(overlay_window);
        
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