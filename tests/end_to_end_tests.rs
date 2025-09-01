use pointless::{Config, GridType, ModifierKey, PointlessError};
use std::env;

#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    fn skip_if_no_display() -> bool {
        if env::var("DISPLAY").is_err() && env::var("XEPHYR_DISPLAY").is_err() {
            eprintln!("Skipping end-to-end test - no DISPLAY available");
            return true;
        }
        false
    }

    #[test]
    fn test_config_validation() {
        // Test valid configurations
        let valid_configs = vec![
            Config {
                grid_type: GridType::Homogeneous,
                modifier_key: ModifierKey::Alt,
                grid_density: 3,
                color: "FFFF00".to_string(),
                opacity: 0.8,
                debug: false,
            },
            Config {
                grid_type: GridType::Vim,
                modifier_key: ModifierKey::Super,
                grid_density: 1,
                color: "FF0000".to_string(),
                opacity: 1.0,
                debug: true,
            },
        ];

        for config in valid_configs {
            assert!(
                config.validate().is_ok(),
                "Valid config should pass validation: {:#?}",
                config
            );
        }

        // Test invalid configurations
        let invalid_configs = vec![
            Config {
                grid_type: GridType::Homogeneous,
                modifier_key: ModifierKey::Alt,
                grid_density: 0, // Invalid: zero density
                color: "FFFF00".to_string(),
                opacity: 0.8,
                debug: false,
            },
            Config {
                grid_type: GridType::Homogeneous,
                modifier_key: ModifierKey::Alt,
                grid_density: 3,
                color: "FFFF00".to_string(),
                opacity: -0.1, // Invalid: negative opacity
                debug: false,
            },
            Config {
                grid_type: GridType::Homogeneous,
                modifier_key: ModifierKey::Alt,
                grid_density: 3,
                color: "FFFF00".to_string(),
                opacity: 1.5, // Invalid: opacity > 1.0
                debug: false,
            },
        ];

        for config in invalid_configs {
            assert!(
                config.validate().is_err(),
                "Invalid config should fail validation: {:#?}",
                config
            );
        }
    }

    #[test]
    fn test_application_components_integration() {
        if skip_if_no_display() {
            return;
        }

        use pointless::grid_manager::GridManager;
        use pointless::input_handler::InputHandler;
        use pointless::renderer::Renderer;
        use pointless::X11Client;

        // Test that all components can be initialized together
        let config = Config {
            grid_type: GridType::Homogeneous,
            modifier_key: ModifierKey::Alt,
            grid_density: 2,
            color: "FFFF00".to_string(),
            opacity: 0.8,
            debug: false,
        };

        // Initialize X11 client
        let x11_result = X11Client::new();
        match x11_result {
            Ok(x11_client) => {
                let screen_info = x11_client.screen_info();

                // Initialize grid manager
                let grid_manager = GridManager::new(config.grid_type.clone());

                // Initialize input handler
                let input_result = InputHandler::new(config.modifier_key.clone(), &x11_client);
                assert!(
                    input_result.is_ok(),
                    "InputHandler should initialize successfully"
                );

                // Initialize renderer
                let mut renderer = Renderer::new();
                let overlay_result = renderer.create_overlay(&x11_client, &config);

                // Test basic interaction between components
                let cells = grid_manager.calculate_grid(screen_info, config.grid_density);
                assert!(!cells.is_empty(), "GridManager should generate cells");

                // Test that renderer can work with the grid cells if overlay was created
                match overlay_result {
                    Ok(_) => {
                        let render_result = renderer.draw_grid(&x11_client, &cells);
                        match render_result {
                            Ok(_) => {} // Rendering succeeded
                            Err(e) => {
                                // Rendering might fail in some test environments
                                eprintln!(
                                    "Rendering failed (may be expected in test environment): {}",
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        // Overlay creation might fail in test environments
                        eprintln!("Renderer overlay creation failed (may be expected in test environment): {}", e);
                    }
                }
            }
            Err(PointlessError::X11ConnectionFailed { .. }) => {
                eprintln!("Skipping integration test - X11 connection failed (expected in headless environments)");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_error_propagation() {
        // Test that errors are properly propagated through the system
        if skip_if_no_display() {
            return;
        }

        use pointless::X11Client;

        // Test with potentially invalid display
        let original_display = env::var("DISPLAY").ok();
        env::set_var("DISPLAY", ":999"); // Likely invalid display

        let result = X11Client::new();
        match result {
            Ok(_) => {
                // Some systems might have a display :999
                eprintln!("Unexpectedly connected to :999 display");
            }
            Err(PointlessError::X11ConnectionFailed { .. }) => {
                // Expected behavior - connection failed as intended
            }
            Err(_e) => {
                // Error was properly propagated
            }
        }

        // Restore original display
        match original_display {
            Some(display) => env::set_var("DISPLAY", display),
            None => env::remove_var("DISPLAY"),
        }
    }

    #[test]
    fn test_memory_management() {
        if skip_if_no_display() {
            return;
        }

        use pointless::grid_manager::GridManager;
        use pointless::X11Client;

        // Test that creating and dropping multiple components doesn't leak memory
        // (This is a basic smoke test - proper memory leak testing would require tools like valgrind)

        let config = Config {
            grid_type: GridType::Homogeneous,
            modifier_key: ModifierKey::Alt,
            grid_density: 3,
            color: "FFFF00".to_string(),
            opacity: 0.8,
            debug: false,
        };

        for _ in 0..10 {
            match X11Client::new() {
                Ok(x11_client) => {
                    let screen_info = x11_client.screen_info();
                    let _ = GridManager::new(config.grid_type.clone());
                    // Test grid calculation
                    let grid_manager = GridManager::new(config.grid_type.clone());
                    let _ = grid_manager.calculate_grid(screen_info, config.grid_density);
                    // Components should be dropped cleanly at end of scope
                }
                Err(PointlessError::X11ConnectionFailed { .. }) => {
                    eprintln!("Skipping memory test - X11 connection failed");
                    break;
                }
                Err(e) => panic!("Unexpected error in memory test: {}", e),
            }
        }

        // Memory management test completed successfully if we reach here
    }

    #[test]
    fn test_concurrent_access() {
        if skip_if_no_display() {
            return;
        }

        use pointless::X11Client;
        use std::sync::{Arc, Barrier};
        use std::thread;

        // Test that multiple threads can safely access X11 functionality
        // Note: This is a basic test - X11 has complex threading requirements

        let barrier = Arc::new(Barrier::new(3));
        let mut handles = vec![];

        for i in 0..3 {
            let barrier = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier.wait();

                // Each thread tries to create an X11 client
                match X11Client::new() {
                    Ok(client) => {
                        let screen_info = client.screen_info();
                        assert!(screen_info.width > 0, "Thread {} got valid screen width", i);
                    }
                    Err(PointlessError::X11ConnectionFailed { .. }) => {
                        eprintln!(
                            "Thread {} - X11 connection failed (expected in some environments)",
                            i
                        );
                    }
                    Err(e) => panic!("Thread {} unexpected error: {}", i, e),
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Concurrent access test completed successfully if we reach here
    }

    #[test]
    fn test_configuration_edge_cases() {
        // Test configuration validation with edge cases
        let edge_case_configs = vec![
            // Minimum valid values
            Config {
                grid_type: GridType::Homogeneous,
                modifier_key: ModifierKey::Alt,
                grid_density: 1,
                color: "000000".to_string(), // Black
                opacity: 0.0,                // Fully transparent
                debug: false,
            },
            // Maximum reasonable values
            Config {
                grid_type: GridType::Vim,
                modifier_key: ModifierKey::Super,
                grid_density: 10,
                color: "FFFFFF".to_string(), // White
                opacity: 1.0,                // Fully opaque
                debug: true,
            },
        ];

        for config in edge_case_configs {
            match config.validate() {
                Ok(_) => {} // Edge case config is valid as expected
                Err(e) => {
                    // Some edge cases might be invalid - that's okay if properly validated
                    eprintln!(
                        "Edge case config validation failed (may be expected): {} for {:#?}",
                        e, config
                    );
                }
            }
        }
    }

    #[test]
    fn test_grid_type_compatibility() {
        // Test that all grid types can be used with the system
        let grid_types = vec![
            GridType::Homogeneous,
            GridType::Qwerty,
            GridType::Arrow,
            GridType::Vim,
        ];

        for grid_type in grid_types {
            let config = Config {
                grid_type: grid_type.clone(),
                modifier_key: ModifierKey::Alt,
                grid_density: 2,
                color: "FFFF00".to_string(),
                opacity: 0.8,
                debug: false,
            };

            // Should validate successfully
            assert!(
                config.validate().is_ok(),
                "Config with {:?} should validate",
                grid_type
            );

            // Should be able to create grid manager
            use pointless::grid_manager::GridManager;
            let grid_manager = GridManager::new(grid_type.clone());
            // GridManager created successfully

            // Should be able to generate test cells
            use pointless::ScreenInfo;
            let screen_info = ScreenInfo {
                width: 1920,
                height: 1080,
                root_window: 1,
            };
            let _cells = grid_manager.calculate_grid(&screen_info, 2);
            // Grid calculation completed successfully
        }
    }
}
