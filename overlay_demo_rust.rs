extern crate x11;

use x11::xlib::*;
use std::ptr;

// Simple types for the demo
#[derive(Debug, Clone, Copy)]
struct GridCoord {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
struct GridCell {
    coord: GridCoord,
    screen_rect: Rectangle,
    label: String,
}

impl Rectangle {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

impl GridCell {
    fn new(coord: GridCoord, screen_rect: Rectangle, label: String) -> Self {
        Self { coord, screen_rect, label }
    }
}

// Simple grid generation function
fn generate_grid(screen_width: u32, screen_height: u32, density: u32) -> Vec<GridCell> {
    let mut cells = Vec::new();
    let cols = density;
    let rows = density;
    
    let cell_width = screen_width / cols;
    let cell_height = screen_height / rows;
    
    for row in 0..rows {
        for col in 0..cols {
            let coord = GridCoord { x: col, y: row };
            let rect = Rectangle::new(
                (col * cell_width) as i32,
                (row * cell_height) as i32,
                cell_width,
                cell_height,
            );
            
            // Create label like A1, B1, C1, A2, B2, C2, etc.
            let col_char = (b'A' + (col % 26) as u8) as char;
            let row_num = row + 1;
            let label = format!("{}{}", col_char, row_num);
            
            cells.push(GridCell::new(coord, rect, label));
        }
    }
    
    cells
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Open connection to X server
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            eprintln!("Cannot open display");
            return Err("Failed to open X display".into());
        }

        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);
        
        // Find 32-bit ARGB visual for transparency
        let mut vinfo: XVisualInfo = std::mem::zeroed();
        let visual_found = XMatchVisualInfo(display, screen, 32, TrueColor, &mut vinfo) != 0;
        
        if !visual_found {
            println!("32-bit ARGB visual not found, falling back to 24-bit");
            if XMatchVisualInfo(display, screen, 24, TrueColor, &mut vinfo) == 0 {
                eprintln!("No suitable visual found");
                XCloseDisplay(display);
                return Err("No suitable visual".into());
            }
        }

        println!("Using visual: depth={}, class={}", vinfo.depth, vinfo.class);

        // Create colormap for the visual
        let colormap = XCreateColormap(display, root, vinfo.visual, AllocNone);

        // Set up window attributes for transparency
        let mut attr: XSetWindowAttributes = std::mem::zeroed();
        attr.colormap = colormap;
        attr.border_pixel = 0;
        attr.background_pixel = 0; // Transparent background
        attr.override_redirect = True; // Bypass window manager

        // Create overlay window
        let width = XDisplayWidth(display, screen) as u32;
        let height = XDisplayHeight(display, screen) as u32;

        let overlay = XCreateWindow(
            display,
            root,
            0, 0,
            width, height,
            0, // border_width
            vinfo.depth,
            InputOutput as u32,
            vinfo.visual,
            CWColormap | CWBorderPixel | CWBackPixel | CWOverrideRedirect,
            &mut attr,
        );

        if overlay == 0 {
            eprintln!("Failed to create overlay window");
            XCloseDisplay(display);
            return Err("Failed to create window".into());
        }

        // Skip input passthrough for now - focus on getting visual overlay working

        // Create graphics context
        let gc = XCreateGC(display, overlay, 0, ptr::null_mut());
        if gc.is_null() {
            eprintln!("Failed to create graphics context");
            XDestroyWindow(display, overlay);
            XCloseDisplay(display);
            return Err("Failed to create GC".into());
        }

        // Set up yellow color
        let mut yellow: XColor = std::mem::zeroed();
        yellow.red = 65535;
        yellow.green = 65535;
        yellow.blue = 0;

        if XAllocColor(display, colormap, &mut yellow) == 0 {
            println!("Color allocation failed, using fallback");
            XSetForeground(display, gc, 0xFFFF00);
        } else {
            XSetForeground(display, gc, yellow.pixel);
            println!("Allocated yellow color: pixel={}", yellow.pixel);
        }

        // Set line attributes (2 pixel width for visibility)
        XSetLineAttributes(display, gc, 2, LineSolid, CapButt, JoinMiter);

        // Map the window to make it visible
        XMapWindow(display, overlay);
        XRaiseWindow(display, overlay);

        println!("Overlay window created and mapped");

        // Generate grid cells using pointless logic
        let grid_density = 3;
        let grid_cells = generate_grid(width, height, grid_density);
        println!("Generated {} grid cells", grid_cells.len());

        // Clear window first
        XClearWindow(display, overlay);

        // Draw grid cells
        for cell in &grid_cells {
            let rect = &cell.screen_rect;
            
            // Draw cell border using 4 lines
            // Top line
            XDrawLine(display, overlay, gc, rect.x, rect.y, rect.x + rect.width as i32, rect.y);
            // Bottom line  
            XDrawLine(display, overlay, gc, rect.x, rect.y + rect.height as i32, rect.x + rect.width as i32, rect.y + rect.height as i32);
            // Left line
            XDrawLine(display, overlay, gc, rect.x, rect.y, rect.x, rect.y + rect.height as i32);
            // Right line
            XDrawLine(display, overlay, gc, rect.x + rect.width as i32, rect.y, rect.x + rect.width as i32, rect.y + rect.height as i32);
            
            // Draw cell label at center  
            let center_x = rect.x + (rect.width / 2) as i32;
            let center_y = rect.y + (rect.height / 2) as i32;
            
            // Draw text - position it roughly at center
            let label_cstr = std::ffi::CString::new(cell.label.clone()).unwrap();
            XDrawString(display, overlay, gc, center_x - 10, center_y + 5, label_cstr.as_ptr(), cell.label.len() as i32);
            
            println!("Drew grid cell {} at ({}, {}) size {}x{}", cell.label, rect.x, rect.y, rect.width, rect.height);
        }

        // Flush all drawing operations
        XFlush(display);
        XSync(display, False);

        println!("Grid drawn! Staying on top for 10 seconds...");

        // Keep window on top for 10 seconds
        for i in 0..10 {
            XRaiseWindow(display, overlay);
            XFlush(display);
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("Seconds remaining: {}", 10 - i);
        }

        // Cleanup
        XFreeGC(display, gc);
        XDestroyWindow(display, overlay);
        XFreeColormap(display, colormap);
        XCloseDisplay(display);

        println!("Demo completed successfully!");
    }

    Ok(())
}