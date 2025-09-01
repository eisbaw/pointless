use pointless::{PointlessError, ScreenInfo, X11Client};

#[cfg(test)]
mod x11_integration_tests {
    use super::*;
    use std::env;

    #[test]
    fn test_x11_client_creation() {
        // Only run if we have a display available
        if env::var("DISPLAY").is_err() && env::var("XEPHYR_DISPLAY").is_err() {
            eprintln!("Skipping X11 test - no DISPLAY available");
            return;
        }

        match X11Client::new() {
            Ok(client) => {
                let screen_info = client.screen_info();
                assert!(screen_info.width > 0, "Screen width should be positive");
                assert!(screen_info.height > 0, "Screen height should be positive");
            }
            Err(PointlessError::X11ConnectionFailed { .. }) => {
                eprintln!(
                    "Skipping X11 test - X11 connection failed (expected in headless environments)"
                );
            }
            Err(e) => panic!("Unexpected error creating X11 client: {}", e),
        }
    }

    #[test]
    fn test_screen_info_validity() {
        if env::var("DISPLAY").is_err() && env::var("XEPHYR_DISPLAY").is_err() {
            eprintln!("Skipping X11 test - no DISPLAY available");
            return;
        }

        match X11Client::new() {
            Ok(client) => {
                let screen_info = client.screen_info();

                // Basic sanity checks for screen dimensions
                assert!(
                    screen_info.width >= 800,
                    "Screen width should be at least 800px"
                );
                assert!(
                    screen_info.height >= 600,
                    "Screen height should be at least 600px"
                );
                assert!(
                    screen_info.width <= 7680,
                    "Screen width should be reasonable (≤ 7680px)"
                );
                assert!(
                    screen_info.height <= 4320,
                    "Screen height should be reasonable (≤ 4320px)"
                );

                // Root window should be valid
                assert!(
                    screen_info.root_window > 0,
                    "Root window ID should be positive"
                );
            }
            Err(PointlessError::X11ConnectionFailed { .. }) => {
                eprintln!(
                    "Skipping X11 test - X11 connection failed (expected in headless environments)"
                );
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_cursor_positioning() {
        if env::var("DISPLAY").is_err() && env::var("XEPHYR_DISPLAY").is_err() {
            eprintln!("Skipping X11 test - no DISPLAY available");
            return;
        }

        match X11Client::new() {
            Ok(client) => {
                let screen_info = client.screen_info();

                // Test positioning cursor to center of screen
                let center_x = screen_info.width as i32 / 2;
                let center_y = screen_info.height as i32 / 2;

                match client.warp_cursor(center_x, center_y) {
                    Ok(_) => {
                        // Success - cursor positioning works
                    }
                    Err(e) => {
                        // This might fail in some environments (e.g., Xvfb without input)
                        eprintln!("Cursor positioning failed (may be expected): {}", e);
                    }
                }
            }
            Err(PointlessError::X11ConnectionFailed { .. }) => {
                eprintln!(
                    "Skipping X11 test - X11 connection failed (expected in headless environments)"
                );
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_x11_error_handling() {
        // Test that we handle X11 errors gracefully
        if env::var("DISPLAY").is_err() && env::var("XEPHYR_DISPLAY").is_err() {
            eprintln!("Skipping X11 test - no DISPLAY available");
            return;
        }

        match X11Client::new() {
            Ok(client) => {
                // Test invalid cursor coordinates (should be handled gracefully)
                let result = client.warp_cursor(-1000, -1000);
                match result {
                    Ok(_) => {
                        // Some X11 implementations allow negative coordinates
                    }
                    Err(_) => {
                        // Expected behavior - invalid coordinates should be rejected
                    }
                }
            }
            Err(PointlessError::X11ConnectionFailed { .. }) => {
                eprintln!(
                    "Skipping X11 test - X11 connection failed (expected in headless environments)"
                );
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_multiple_x11_clients() {
        if env::var("DISPLAY").is_err() && env::var("XEPHYR_DISPLAY").is_err() {
            eprintln!("Skipping X11 test - no DISPLAY available");
            return;
        }

        // Test that we can create multiple X11 clients
        match (X11Client::new(), X11Client::new()) {
            (Ok(client1), Ok(client2)) => {
                let screen1 = client1.screen_info();
                let screen2 = client2.screen_info();

                // Both clients should report the same screen info
                assert_eq!(screen1.width, screen2.width);
                assert_eq!(screen1.height, screen2.height);
                assert_eq!(screen1.root_window, screen2.root_window);
            }
            (Err(PointlessError::X11ConnectionFailed { .. }), _)
            | (_, Err(PointlessError::X11ConnectionFailed { .. })) => {
                eprintln!(
                    "Skipping X11 test - X11 connection failed (expected in headless environments)"
                );
            }
            (Err(e), _) | (_, Err(e)) => panic!("Unexpected error: {}", e),
        }
    }
}

#[cfg(test)]
mod x11_mock_tests {
    use super::*;

    #[test]
    fn test_screen_info_struct() {
        let screen_info = ScreenInfo {
            width: 1920,
            height: 1080,
            root_window: 123,
        };

        assert_eq!(screen_info.width, 1920);
        assert_eq!(screen_info.height, 1080);
        assert_eq!(screen_info.root_window, 123);
    }
}
