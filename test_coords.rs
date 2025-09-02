// Simple test to verify coordinate calculations
fn main() {
    // Screen: 1920x1200, density: 3
    let screen_width = 1920;
    let screen_height = 1200;
    let density = 3;
    
    let cell_width = screen_width / density;
    let cell_height = screen_height / density;
    
    println!("Screen: {}x{}, Density: {}", screen_width, screen_height, density);
    println!("Cell size: {}x{}", cell_width, cell_height);
    println!();
    
    // Test coordinates from debug logs
    let test_coords = [
        (0, 0, "A1"), // Top-left
        (1, 1, "B2"), // Center  
        (2, 2, "C3"), // Bottom-right
    ];
    
    for (x, y, label) in test_coords {
        let expected_x = x * cell_width + cell_width / 2;
        let expected_y = y * cell_height + cell_height / 2;
        
        println!("{}: grid({}, {}) -> expected screen({}, {})", 
                label, x, y, expected_x, expected_y);
    }
}