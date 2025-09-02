#include <X11/Xlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    Display *display;
    Window root;
    GC gc;
    XColor yellow;
    Colormap colormap;
    int screen;
    
    // Open connection to X server
    display = XOpenDisplay(NULL);
    if (!display) {
        fprintf(stderr, "Cannot open display\n");
        return 1;
    }
    
    screen = DefaultScreen(display);
    root = RootWindow(display, screen);
    colormap = DefaultColormap(display, screen);
    
    // Create graphics context
    gc = XCreateGC(display, root, 0, NULL);
    
    // Set up yellow color
    yellow.red = 65535;   // Max intensity
    yellow.green = 65535; // Max intensity  
    yellow.blue = 0;      // No blue
    XAllocColor(display, colormap, &yellow);
    XSetForeground(display, gc, yellow.pixel);
    
    // Set line width to 1 pixel
    XSetLineAttributes(display, gc, 1, LineSolid, CapButt, JoinMiter);
    
    // Get screen dimensions
    int width = DisplayWidth(display, screen);
    int height = DisplayHeight(display, screen);
    
    // Draw 3x3 grid in center of screen
    int grid_size = 300;
    int start_x = (width - grid_size) / 2;
    int start_y = (height - grid_size) / 2;
    int cell_size = grid_size / 3;
    
    printf("Drawing 3x3 grid at (%d, %d) with cell size %d\n", start_x, start_y, cell_size);
    
    // Draw vertical lines
    for (int i = 0; i <= 3; i++) {
        int x = start_x + i * cell_size;
        XDrawLine(display, root, gc, x, start_y, x, start_y + grid_size);
    }
    
    // Draw horizontal lines  
    for (int i = 0; i <= 3; i++) {
        int y = start_y + i * cell_size;
        XDrawLine(display, root, gc, start_x, y, start_x + grid_size, y);
    }
    
    // Flush to ensure drawing is visible
    XFlush(display);
    
    printf("Grid drawn! Press Enter to clear and exit...\n");
    getchar();
    
    // Clear the grid by drawing in background color
    XSetForeground(display, gc, BlackPixel(display, screen));
    
    // Redraw lines in black to "erase" them
    for (int i = 0; i <= 3; i++) {
        int x = start_x + i * cell_size;
        XDrawLine(display, root, gc, x, start_y, x, start_y + grid_size);
    }
    for (int i = 0; i <= 3; i++) {
        int y = start_y + i * cell_size;
        XDrawLine(display, root, gc, start_x, y, start_x + grid_size, y);
    }
    
    XFlush(display);
    
    // Cleanup
    XFreeGC(display, gc);
    XCloseDisplay(display);
    
    return 0;
}