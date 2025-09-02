#include <X11/Xlib.h>
#include <X11/extensions/Xfixes.h>
#include <X11/extensions/shape.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main() {
    Display *display;
    Window root, overlay;
    GC gc;
    XVisualInfo vinfo;
    Colormap colormap;
    XSetWindowAttributes attr;
    XColor yellow;
    int screen;
    
    // Open connection to X server
    display = XOpenDisplay(NULL);
    if (!display) {
        fprintf(stderr, "Cannot open display\n");
        return 1;
    }
    
    screen = DefaultScreen(display);
    root = RootWindow(display, screen);
    
    // Find 32-bit ARGB visual for transparency
    if (!XMatchVisualInfo(display, screen, 32, TrueColor, &vinfo)) {
        // Fallback to 24-bit if 32-bit not available
        if (!XMatchVisualInfo(display, screen, 24, TrueColor, &vinfo)) {
            fprintf(stderr, "No suitable visual found\n");
            XCloseDisplay(display);
            return 1;
        }
    }
    
    printf("Using visual: depth=%d, class=%d\n", vinfo.depth, vinfo.class);
    
    // Create colormap for the visual
    colormap = XCreateColormap(display, root, vinfo.visual, AllocNone);
    
    // Set up window attributes for transparency
    attr.colormap = colormap;
    attr.border_pixel = 0;
    attr.background_pixel = 0; // Transparent
    attr.override_redirect = True; // Bypass window manager
    
    // Create overlay window
    int width = DisplayWidth(display, screen);
    int height = DisplayHeight(display, screen);
    
    overlay = XCreateWindow(display, root, 0, 0, width, height, 0,
                           vinfo.depth, InputOutput, vinfo.visual,
                           CWColormap | CWBorderPixel | CWBackPixel | CWOverrideRedirect,
                           &attr);
    
    if (!overlay) {
        fprintf(stderr, "Failed to create overlay window\n");
        XCloseDisplay(display);
        return 1;
    }
    
    // Make window input-transparent (clicks pass through)
    XserverRegion region = XFixesCreateRegion(display, NULL, 0);
    XFixesSetWindowShapeRegion(display, overlay, ShapeInput, 0, 0, region);
    XFixesDestroyRegion(display, region);
    
    // Create graphics context
    gc = XCreateGC(display, overlay, 0, NULL);
    
    // Set up yellow color
    yellow.red = 65535;
    yellow.green = 65535;  
    yellow.blue = 0;
    if (!XAllocColor(display, colormap, &yellow)) {
        // Fallback to a direct pixel value for yellow
        XSetForeground(display, gc, 0xFFFF00);
        printf("Using fallback yellow color\n");
    } else {
        XSetForeground(display, gc, yellow.pixel);
        printf("Allocated yellow color: pixel=%lu\n", yellow.pixel);
    }
    
    // Set line attributes
    XSetLineAttributes(display, gc, 2, LineSolid, CapButt, JoinMiter);
    
    // Map the window to make it visible
    XMapWindow(display, overlay);
    XRaiseWindow(display, overlay);
    
    printf("Overlay window created and mapped\n");
    
    // Draw 3x3 grid in center of screen
    int grid_size = 300;
    int start_x = (width - grid_size) / 2;
    int start_y = (height - grid_size) / 2;
    int cell_size = grid_size / 3;
    
    printf("Drawing 3x3 grid at (%d, %d) with cell size %d\n", start_x, start_y, cell_size);
    
    // Clear window first
    XClearWindow(display, overlay);
    
    // Draw vertical lines
    for (int i = 0; i <= 3; i++) {
        int x = start_x + i * cell_size;
        XDrawLine(display, overlay, gc, x, start_y, x, start_y + grid_size);
        printf("Drew vertical line at x=%d\n", x);
    }
    
    // Draw horizontal lines  
    for (int i = 0; i <= 3; i++) {
        int y = start_y + i * cell_size;
        XDrawLine(display, overlay, gc, start_x, y, start_x + grid_size, y);
        printf("Drew horizontal line at y=%d\n", y);
    }
    
    // Flush all drawing operations
    XFlush(display);
    XSync(display, False);
    
    printf("Grid drawn! Staying on top for 10 seconds...\n");
    
    // Keep window on top for 10 seconds
    for (int i = 0; i < 10; i++) {
        XRaiseWindow(display, overlay);  // Keep it on top
        XFlush(display);
        sleep(1);
        printf("Seconds remaining: %d\n", 10-i);
    }
    
    // Cleanup
    XDestroyWindow(display, overlay);
    XFreeColormap(display, colormap);
    XFreeGC(display, gc);
    XCloseDisplay(display);
    
    return 0;
}