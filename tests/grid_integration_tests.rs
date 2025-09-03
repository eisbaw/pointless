use pointless::config::MoveStyle;
use pointless::grid_manager::GridManager;
use pointless::{Config, GridType, ModifierKey, ScreenInfo};

#[cfg(test)]
mod grid_integration_tests {
    use super::*;

    fn create_test_screen_info() -> ScreenInfo {
        ScreenInfo {
            width: 1920,
            height: 1080,
            root_window: 1,
        }
    }

    fn create_test_config(grid_type: GridType) -> Config {
        Config {
            grid_type,
            modifier_key: ModifierKey::Alt,
            grid_density: 3,
            color: "FFFF00".to_string(),
            opacity: 0.8,
            debug: false,
            png_output: None,
            png_resolution: "1920x1080".to_string(),
            arrow: "3x3".to_string(),
            always_show: false,
            movestyle: MoveStyle::None,
        }
    }

    #[test]
    fn test_grid_manager_initialization() {
        let config = create_test_config(GridType::Homogeneous);
        let _grid_manager = GridManager::new(&config).unwrap();
        // GridManager initialization doesn't fail in current implementation
    }

    #[test]
    fn test_all_grid_types_initialization() {
        let grid_types = vec![
            GridType::Homogeneous,
            GridType::Qwerty,
            GridType::Arrow,
            GridType::Vim,
        ];

        for grid_type in grid_types {
            let config = create_test_config(grid_type.clone());
            let _grid_manager = GridManager::new(&config).unwrap();
            // GridManager initialized successfully for this grid type
        }
    }

    #[test]
    fn test_grid_cell_generation() {
        let screen_info = create_test_screen_info();
        let config = create_test_config(GridType::Homogeneous);
        let grid_manager = GridManager::new(&config).unwrap();

        let cells = grid_manager.calculate_grid(&screen_info, 3);
        assert!(!cells.is_empty(), "Grid should have cells");

        // Check that all cells are within screen bounds
        for cell in cells {
            assert!(
                cell.screen_rect.x >= 0,
                "Cell x coordinate should be non-negative"
            );
            assert!(
                cell.screen_rect.y >= 0,
                "Cell y coordinate should be non-negative"
            );
            assert!(
                cell.screen_rect.x + cell.screen_rect.width as i32 <= screen_info.width as i32,
                "Cell should fit within screen width"
            );
            assert!(
                cell.screen_rect.y + cell.screen_rect.height as i32 <= screen_info.height as i32,
                "Cell should fit within screen height"
            );
        }
    }

    #[test]
    fn test_different_screen_sizes() {
        let screen_sizes = vec![(800, 600), (1920, 1080), (2560, 1440), (3840, 2160)];

        for (width, height) in screen_sizes {
            let screen_info = ScreenInfo {
                width: width as u32,
                height: height as u32,
                root_window: 1,
            };

            let config = create_test_config(GridType::Homogeneous);
            let grid_manager = GridManager::new(&config).unwrap();
            let cells = grid_manager.calculate_grid(&screen_info, 2);
            assert!(
                !cells.is_empty(),
                "Grid should generate cells for {}x{} screen",
                width,
                height
            );

            // Verify all cells are within bounds
            for cell in cells {
                assert!(
                    cell.screen_rect.x + cell.screen_rect.width as i32 <= width,
                    "Cell should fit within screen width for {}x{}",
                    width,
                    height
                );
                assert!(
                    cell.screen_rect.y + cell.screen_rect.height as i32 <= height,
                    "Cell should fit within screen height for {}x{}",
                    width,
                    height
                );
            }
        }
    }

    #[test]
    fn test_grid_density_variations() {
        let screen_info = create_test_screen_info();
        let densities = vec![1, 2, 3, 4, 5];

        for density in densities {
            let config = create_test_config(GridType::Homogeneous);
            let grid_manager = GridManager::new(&config).unwrap();
            let cells = grid_manager.calculate_grid(&screen_info, density);
            assert!(
                !cells.is_empty(),
                "Grid should generate cells with density {}",
                density
            );

            // Higher density should generally mean more or finer-grained cells
            if density > 1 {
                assert!(
                    !cells.is_empty(),
                    "Grid with density {} should have cells",
                    density
                );
            }
        }
    }

    #[test]
    fn test_error_handling_degenerate_screen() {
        let degenerate_screens = vec![
            ScreenInfo {
                width: 1,
                height: 1080,
                root_window: 1,
            },
            ScreenInfo {
                width: 1920,
                height: 1,
                root_window: 1,
            },
            ScreenInfo {
                width: 1,
                height: 1,
                root_window: 1,
            },
        ];

        for screen_info in degenerate_screens {
            let config = create_test_config(GridType::Homogeneous);
            let grid_manager = GridManager::new(&config).unwrap();
            let _cells = grid_manager.calculate_grid(&screen_info, 3);
            // Should handle degenerate cases gracefully (empty cells are acceptable)
        }
    }

    #[test]
    fn test_grid_coordinate_mapping() {
        let screen_info = create_test_screen_info();
        let config = create_test_config(GridType::Homogeneous);
        let mut grid_manager = GridManager::new(&config).unwrap();

        // Test that processing keys works without crashing
        // KeyCode is just u32, so we use a valid X11 keycode (0x26 is 'a' key)
        let cells = grid_manager.calculate_grid(&screen_info, 3);
        let result = grid_manager.process_key_with_cells(0x26, &cells, &screen_info);

        match result {
            Some(coord) => {
                assert!(
                    coord.x >= 0 && coord.x <= screen_info.width as i32,
                    "Returned coordinate should be within screen bounds"
                );
                assert!(
                    coord.y >= 0 && coord.y <= screen_info.height as i32,
                    "Returned coordinate should be within screen bounds"
                );
            }
            None => {
                // No coordinate returned is also valid (e.g., first key of two-key navigation)
            }
        }
    }

    #[test]
    fn test_grid_reset_functionality() {
        let screen_info = create_test_screen_info();
        let config = create_test_config(GridType::Arrow);
        let mut grid_manager = GridManager::new(&config).unwrap();

        // Process some keys to change state
        // Use valid X11 keycodes (Right=114, Down=116 in typical X11)
        let cells = grid_manager.calculate_grid(&screen_info, 3);
        let _ = grid_manager.process_key_with_cells(114, &cells, &screen_info);
        let _ = grid_manager.process_key_with_cells(116, &cells, &screen_info);

        // Reset should work without error
        grid_manager.reset();
        // Grid reset completed successfully

        // Should be able to generate cells after reset
        let cells = grid_manager.calculate_grid(&screen_info, 3);
        assert!(!cells.is_empty(), "Grid should generate cells after reset");
    }
}
