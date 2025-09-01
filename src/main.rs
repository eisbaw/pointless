mod config;
mod error;
mod grid_manager;
mod input_handler;
mod renderer;
mod types;
mod x11_client;

use clap::Parser;
use config::Config;
use grid_manager::GridManager;
use input_handler::{InputEvent, InputHandler};
use renderer::Renderer;
use x11_client::X11Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    
    // Validate configuration early - fail fast on invalid parameters
    config.validate()?;

    if config.debug {
        println!("Debug mode enabled");
        println!("Configuration: {:#?}", config);
    }

    // Initialize X11 connection
    let x11_client = X11Client::new()?;
    let screen_info = x11_client.screen_info();

    if config.debug {
        println!("Connected to X11 display");
        println!("Screen info: {:#?}", screen_info);
    }

    println!("Pointless X11 cursor utility initialized");
    println!("Screen: {}x{}", screen_info.width, screen_info.height);
    println!("Grid type: {:?}", config.grid_type);
    println!("Modifier key: {:?}", config.modifier_key);

    // Test basic cursor control
    if config.debug {
        println!("Testing cursor control...");

        // Move cursor to center of screen
        let center_x = screen_info.width as i32 / 2;
        let center_y = screen_info.height as i32 / 2;

        match x11_client.warp_cursor(center_x, center_y) {
            Ok(()) => println!("Cursor moved to center: ({}, {})", center_x, center_y),
            Err(e) => println!("Failed to move cursor: {}", e),
        }

        // Wait a moment then move to corner
        std::thread::sleep(std::time::Duration::from_millis(1000));

        match x11_client.warp_cursor(100, 100) {
            Ok(()) => println!("Cursor moved to corner: (100, 100)"),
            Err(e) => println!("Failed to move cursor: {}", e),
        }
    }

    // Initialize input handler
    let mut input_handler = InputHandler::new(config.modifier_key.clone(), &x11_client)?;
    input_handler.start_capture(&x11_client)?;

    // Initialize renderer
    let mut renderer = Renderer::new();
    renderer.create_overlay(&x11_client, &config)?;

    // Initialize grid manager
    let mut grid_manager = GridManager::new(config.grid_type.clone());

    if config.debug {
        println!(
            "Input handler initialized for modifier key: {:?}",
            config.modifier_key
        );
        println!("Renderer and grid manager initialized");
        println!(
            "Hold {} key to show grid, press keys 1-4, Q-R, A-F, Z-V to navigate. Ctrl+C to exit.",
            match config.modifier_key {
                config::ModifierKey::Super => "Super/Windows",
                config::ModifierKey::Alt => "Alt",
                config::ModifierKey::Ctrl => "Ctrl",
            }
        );
    }

    // Main event loop
    loop {
        let event = x11_client.next_event();
        let input_event = input_handler.process_event(&event);

        match input_event {
            InputEvent::ModifierPressed => {
                if config.debug {
                    println!("Modifier key pressed - showing grid");
                }

                // Calculate and show grid
                let cells = grid_manager.calculate_grid(screen_info, config.grid_density);
                renderer.draw_grid(&x11_client, cells)?;
                renderer.show_overlay(&x11_client)?;
            }
            InputEvent::ModifierReleased => {
                if config.debug {
                    println!("Modifier key released - hiding grid");
                }

                // Hide grid and reset state
                renderer.hide_overlay(&x11_client)?;
                grid_manager.reset();
            }
            InputEvent::NavigationKey(keycode) => {
                if config.debug {
                    println!("Navigation key pressed: {}", keycode);
                }

                // Process navigation and move cursor
                if let Some(target_pos) = grid_manager.process_key(keycode) {
                    match x11_client.warp_cursor(target_pos.x, target_pos.y) {
                        Ok(()) => {
                            if config.debug {
                                println!("Cursor moved to: ({}, {})", target_pos.x, target_pos.y);
                            }
                        }
                        Err(e) => {
                            if config.debug {
                                println!("Failed to move cursor: {}", e);
                            }
                        }
                    }
                } else {
                    if config.debug {
                        println!("Unknown navigation key: {}", keycode);
                    }
                }
            }
            InputEvent::Unknown => {
                // Ignore unknown events
            }
        }
    }

    // Cleanup (unreachable in current implementation due to infinite loop)
    // input_handler.stop_capture(&x11_client);
    // renderer.destroy_overlay(&x11_client);
}
