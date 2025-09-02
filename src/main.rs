// Allow u32->i32 casts in coordinate calculations - safe in X11 display context
#![allow(clippy::cast_possible_wrap)]

mod config;
mod constants;
mod coordinate_resolver;
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
use x11_client::{ScreenInfo, X11Client};

/// Application state holding all initialized components
struct ApplicationState {
    config: Config,
    x11_client: X11Client,
    input_handler: InputHandler,
    renderer: Renderer,
    grid_manager: GridManager,
    screen_info: ScreenInfo,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    initialize_logging(&config);
    initialize_and_run_application(config)
}

/// Initialize logging based on configuration
fn initialize_logging(config: &Config) {
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
}

/// Initialize and run the complete application
fn initialize_and_run_application(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let app_state = initialize_application_components(config)?;
    run_event_loop(app_state)
}

/// Initialize all application components
fn initialize_application_components(
    config: Config,
) -> Result<ApplicationState, Box<dyn std::error::Error>> {
    // Validate configuration early - fail fast on invalid parameters
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(Box::new(e));
    }

    // Initialize X11 connection
    let x11_client = X11Client::new()?;
    let screen_info = *x11_client.screen_info(); // Copy the value

    debug!("Connected to X11 display");
    debug!("Screen info: {:#?}", screen_info);

    info!("Pointless X11 cursor utility initialized");
    info!("Screen: {}x{}", screen_info.width, screen_info.height);
    info!("Grid type: {:?}", config.grid_type);
    info!("Modifier key: {:?}", config.modifier_key);

    // Test basic cursor control in debug mode
    if config.debug {
        test_cursor_control(&x11_client, &screen_info)?;
    }

    // Initialize components
    let mut input_handler = InputHandler::new(config.modifier_key.clone(), &x11_client)?;
    input_handler.start_capture(&x11_client)?;

    let mut renderer = Renderer::new();
    renderer.create_overlay(&x11_client, &config)?;

    let grid_manager = GridManager::new(config.grid_type.clone());

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

    Ok(ApplicationState {
        config,
        x11_client,
        input_handler,
        renderer,
        grid_manager,
        screen_info,
    })
}

/// Test basic cursor control functionality in debug mode
fn test_cursor_control(
    x11_client: &X11Client,
    screen_info: &ScreenInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Testing cursor control...");

    // Move cursor to center of screen
    let center_x = screen_info.width as i32 / 2;
    let center_y = screen_info.height as i32 / 2;

    match x11_client.warp_cursor(center_x, center_y) {
        Ok(()) => debug!("Cursor moved to center: ({}, {})", center_x, center_y),
        Err(e) => warn!("Failed to move cursor: {}", e),
    }

    // Wait a moment then move to corner
    std::thread::sleep(std::time::Duration::from_millis(
        crate::constants::DEBUG_SLEEP_MS,
    ));

    match x11_client.warp_cursor(
        crate::constants::DEBUG_TEST_X,
        crate::constants::DEBUG_TEST_Y,
    ) {
        Ok(()) => debug!(
            "Cursor moved to corner: ({}, {})",
            crate::constants::DEBUG_TEST_X,
            crate::constants::DEBUG_TEST_Y
        ),
        Err(e) => warn!("Failed to move cursor: {}", e),
    }

    Ok(())
}

/// Run the main application event loop
fn run_event_loop(mut app_state: ApplicationState) -> Result<(), Box<dyn std::error::Error>> {
    // Set up signal handling for graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    setup_signal_handling(running.clone());

    // Main event loop with graceful shutdown
    while running.load(Ordering::Relaxed) {
        let event = app_state.x11_client.next_event();
        let input_event = app_state.input_handler.process_event(&event);

        handle_input_event(input_event, &mut app_state)?;
    }

    // Proper cleanup on shutdown
    cleanup_application(&mut app_state);
    Ok(())
}

/// Set up signal handling for graceful shutdown
fn setup_signal_handling(running: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        let mut signals = Signals::new([SIGINT, SIGTERM]).expect("Failed to create signal handler");
        if signals.forever().next().is_some() {
            running.store(false, Ordering::Relaxed);
        }
    });
}

/// Handle a single input event
fn handle_input_event(
    input_event: InputEvent,
    app_state: &mut ApplicationState,
) -> Result<(), Box<dyn std::error::Error>> {
    match input_event {
        InputEvent::ModifierPressed => {
            debug!("Modifier key pressed - showing grid");

            // Calculate and show grid
            // Note: Grid calculations are performed on each modifier press rather than cached
            // because: 1) Basic grids are computationally simple (O(n) where n=density^2)
            // 2) Vision grids would need different caching strategies when fully implemented
            // 3) Current performance meets <20ms latency requirement without caching
            // 4) Caching would add complexity for cache invalidation and memory overhead
            let cells = app_state
                .grid_manager
                .calculate_grid(&app_state.screen_info, app_state.config.grid_density);
            app_state
                .renderer
                .draw_grid(&app_state.x11_client, &cells)?;
            app_state.renderer.show_overlay(&app_state.x11_client)?;
        }
        InputEvent::ModifierReleased => {
            debug!("Modifier key released - hiding grid");

            // Hide grid and reset state
            app_state.renderer.hide_overlay(&app_state.x11_client)?;
            app_state.grid_manager.reset();
        }
        InputEvent::NavigationKey(keycode) => {
            debug!("Navigation key pressed: {}", keycode);

            // Process navigation and move cursor
            if let Some(target_pos) = app_state.grid_manager.process_key(
                keycode,
                &app_state.screen_info,
                app_state.config.grid_density,
            ) {
                if let Err(e) = app_state.x11_client.warp_cursor(target_pos.x, target_pos.y) {
                    error!(
                        "Critical cursor movement failure: {} - target was ({}, {})",
                        e, target_pos.x, target_pos.y
                    );
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
    Ok(())
}

/// Perform cleanup on application shutdown
fn cleanup_application(app_state: &mut ApplicationState) {
    info!("Shutting down gracefully...");
    app_state.input_handler.stop_capture(&app_state.x11_client);
    app_state.renderer.destroy_overlay(&app_state.x11_client);
}
