mod config;
mod constants;
mod error;
mod grid_manager;
mod grids;
mod input_handler;
mod renderer;
mod types;
mod x11_client;

use clap::Parser;
use config::Config;
use grid_manager::GridManager;
use input_handler::{InputEvent, InputHandler};
use log::{debug, error, info, warn};
use renderer::Renderer;
use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use x11_client::X11Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    // Initialize logger based on debug flag
    if config.debug {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
        debug!("Debug mode enabled");
        debug!("Configuration: {:#?}", config);
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    // Validate configuration early - fail fast on invalid parameters
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(Box::new(e));
    }

    // Initialize X11 connection
    let x11_client = X11Client::new()?;
    let screen_info = x11_client.screen_info();

    debug!("Connected to X11 display");
    debug!("Screen info: {:#?}", screen_info);

    info!("Pointless X11 cursor utility initialized");
    info!("Screen: {}x{}", screen_info.width, screen_info.height);
    info!("Grid type: {:?}", config.grid_type);
    info!("Modifier key: {:?}", config.modifier_key);

    // Test basic cursor control in debug mode
    if config.debug {
        debug!("Testing cursor control...");

        // Move cursor to center of screen
        let center_x = screen_info.width as i32 / 2;
        let center_y = screen_info.height as i32 / 2;

        match x11_client.warp_cursor(center_x, center_y) {
            Ok(()) => debug!("Cursor moved to center: ({}, {})", center_x, center_y),
            Err(e) => warn!("Failed to move cursor: {}", e),
        }

        // Wait a moment then move to corner
        std::thread::sleep(std::time::Duration::from_millis(crate::constants::DEBUG_SLEEP_MS));

        match x11_client.warp_cursor(crate::constants::DEBUG_TEST_X, crate::constants::DEBUG_TEST_Y) {
            Ok(()) => debug!("Cursor moved to corner: ({}, {})", crate::constants::DEBUG_TEST_X, crate::constants::DEBUG_TEST_Y),
            Err(e) => warn!("Failed to move cursor: {}", e),
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

    debug!(
        "Input handler initialized for modifier key: {:?}",
        config.modifier_key
    );
    debug!("Renderer and grid manager initialized");
    info!(
        "Hold {} key to show grid, press keys 1-4, Q-R, A-F, Z-V to navigate. Ctrl+C to exit.",
        match config.modifier_key {
            config::ModifierKey::Super => "Super/Windows",
            config::ModifierKey::Alt => "Alt",
            config::ModifierKey::Ctrl => "Ctrl",
        }
    );

    // Set up signal handling for graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    std::thread::spawn(move || {
        let mut signals =
            Signals::new(&[SIGINT, SIGTERM]).expect("Failed to create signal handler");
        if let Some(_) = signals.forever().next() {
            r.store(false, Ordering::Relaxed);
        }
    });

    // Main event loop with graceful shutdown
    while running.load(Ordering::Relaxed) {
        let event = x11_client.next_event();
        let input_event = input_handler.process_event(&event);

        match input_event {
            InputEvent::ModifierPressed => {
                debug!("Modifier key pressed - showing grid");

                // Calculate and show grid
                let cells = grid_manager.calculate_grid(screen_info, config.grid_density);
                renderer.draw_grid(&x11_client, &cells)?;
                renderer.show_overlay(&x11_client)?;
            }
            InputEvent::ModifierReleased => {
                debug!("Modifier key released - hiding grid");

                // Hide grid and reset state
                renderer.hide_overlay(&x11_client)?;
                grid_manager.reset();
            }
            InputEvent::NavigationKey(keycode) => {
                debug!("Navigation key pressed: {}", keycode);

                // Process navigation and move cursor
                if let Some(target_pos) =
                    grid_manager.process_key(keycode, screen_info, config.grid_density)
                {
                    if let Err(e) = x11_client.warp_cursor(target_pos.x, target_pos.y) {
                        error!("Critical cursor movement failure: {} - target was ({}, {})", e, target_pos.x, target_pos.y);
                        return Err(Box::new(e));
                    }
                    debug!("Cursor moved to: ({}, {})", target_pos.x, target_pos.y);
                } else {
                    debug!("Unknown navigation key: {}", keycode);
                }
            }
            InputEvent::Unknown => {
                // Ignore unknown events
            }
        }
    }

    // Proper cleanup on shutdown
    info!("Shutting down gracefully...");
    input_handler.stop_capture(&x11_client);
    renderer.destroy_overlay(&x11_client);

    Ok(())
}
