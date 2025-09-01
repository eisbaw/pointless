# Pointless Usage Guide

*Making pointing devices pointless through keyboard-driven cursor control*

## Quick Start

1. **Install and run with default settings:**
   ```bash
   pointless
   ```

2. **Hold the Super key** (Windows key) to display the grid overlay

3. **Navigate using the default homogeneous grid:**
   - Press a column letter (A, B, C...)
   - Press a row number (1, 2, 3...)
   - Example: Press 'B' then '3' to jump to column B, row 3

4. **Release the Super key** to hide the grid and keep cursor positioned

## Grid Types Explained

### Homogeneous Grid (Default)
Best for: Precise positioning, consistent spatial mapping

```bash
pointless --grid-type homogeneous
```

**How it works:**
- Screen divided into uniform rectangular cells
- Cells labeled with A-Z coordinates (A1, B2, C3, etc.)
- Two-key selection: column letter + row number
- Selected cells can subdivide for finer control

**Navigation:**
1. Hold Super key → Grid appears
2. Press column letter (A-Z)
3. Press row number (1-9)
4. Release Super key → Cursor positioned

**Example:** To click on something in the middle-right area:
- Hold Super → Press 'F' → Press '3' → Release Super

### QWERTY Grid
Best for: Intuitive navigation using keyboard muscle memory

```bash
pointless --grid-type qwerty
```

**How it works:**
- Physical keyboard layout projected onto screen
- Each key corresponds to its screen position
- Single key press selects region, second press refines

**Navigation:**
1. Hold Super key → QWERTY grid appears
2. Press any letter key (Q, W, E, R, T, Y, etc.)
3. Optionally press another key to refine position
4. Release Super key

**Example:** To click top-left corner:
- Hold Super → Press 'Q' → Release Super

### Arrow Key Grid
Best for: Simple directional navigation

```bash
pointless --grid-type arrow
```

**How it works:**
- 3x3 grid with center as starting position
- Arrow keys move between cells
- Enter key subdivides selected cell
- Escape key returns to higher level

**Navigation:**
1. Hold Super key → 3x3 grid appears with center selected
2. Use arrow keys to move selection
3. Press Enter to subdivide for finer control
4. Press Escape to go back up a level
5. Release Super key to position cursor

**Example:** To navigate to bottom-right:
- Hold Super → Press Down → Press Right → Release Super

### Vim Grid
Best for: Vim users, advanced navigation patterns

```bash
pointless --grid-type vim
```

**How it works:**
- Vim-like movement commands
- Number prefixes for repetition
- Jump patterns and screen positioning
- Coordinate-based movement without OCR

**Basic Movement:**
- `h` = left, `j` = down, `k` = up, `l` = right
- Number prefixes: `5h` moves left 5 times
- Distances in pixels (default: 10px per movement)

**Jump Commands:**
- `w` = Enter forward jump mode (shows numbered targets)
- `b` = Enter backward jump mode (shows numbered targets)
- `f{1-9}` = Jump to numbered target (f1, f2, f3, etc.)

**Screen Positioning:**
- `H` = Jump to top of screen
- `M` = Jump to middle of screen
- `L` = Jump to bottom of screen
- `0` = Jump to left edge
- `$` = Jump to right edge
- `gg` = Jump to top-left corner
- `G` = Jump to bottom-right corner

**Example Vim workflow:**
- Hold Super → Type `10j` (move down 10 steps) → Type `w` (show jump targets) → Type `3` (jump to target 3) → Release Super

## Vision-Based Grids (Optional)

*Requires building with `--features vision-grids`*

These grids analyze screen content to provide intelligent jump targets:

### Hough Lines Grid
Detects line intersections for precise navigation:
```bash
pointless --grid-type hough-lines
```

### Rectangle Fitting Grid  
Identifies UI elements and windows:
```bash
pointless --grid-type rectangle-fitting
```

### Edge Density Grid
Finds areas with high visual activity:
```bash
pointless --grid-type edge-density
```

### Color Clustering Grid
Groups similar colors for region-based navigation:
```bash
pointless --grid-type color-clustering
```

### Feature Detection Grid
Uses computer vision to find interesting points:
```bash
pointless --grid-type feature-detection
```

## Customization Options

### Change Modifier Key
```bash
# Use Alt instead of Super
pointless --modifier-key alt

# Use Ctrl instead of Super  
pointless --modifier-key ctrl
```

### Adjust Grid Density
```bash
# Finer grid (more subdivisions)
pointless --grid-density 5

# Coarser grid (fewer subdivisions)
pointless --grid-density 2
```

### Customize Visual Appearance
```bash
# Red grid color
pointless --color FF0000

# Blue grid with high opacity
pointless --color 0000FF --opacity 0.9

# Semi-transparent green grid
pointless --color 00FF00 --opacity 0.3
```

### Debug Mode
```bash
# Enable detailed logging
pointless --debug
```

## Common Workflows

### Web Browsing
1. **Click links with homogeneous grid:**
   ```bash
   pointless --grid-type homogeneous --opacity 0.6
   ```
   - Hold Super, identify link location (e.g., C4), release Super, click

2. **Quick page navigation with vim grid:**
   ```bash
   pointless --grid-type vim
   ```
   - Hold Super, press `H` for top of page, `G` for bottom, etc.

### Code Editing
1. **Precise cursor placement:**
   ```bash
   pointless --grid-type homogeneous --grid-density 4
   ```
   - High density grid for character-level precision

2. **Quick jumping between functions:**
   ```bash
   pointless --grid-type vim
   ```
   - Use `w`/`b` jump patterns to navigate between code sections

### Design Work
1. **Vision-based element selection:**
   ```bash
   pointless --grid-type rectangle-fitting --debug
   ```
   - Automatically detects UI elements and design components

2. **Color-based region selection:**
   ```bash  
   pointless --grid-type color-clustering
   ```
   - Groups similar colors for easy selection

## Troubleshooting

### Grid Not Appearing
1. Check if Super key conflicts with window manager
2. Try different modifier key: `--modifier-key alt`
3. Verify X11 permissions and DISPLAY variable

### Performance Issues
1. Reduce grid density: `--grid-density 2`
2. Lower opacity: `--opacity 0.5`  
3. Use simpler grid types (avoid vision-based grids on slow hardware)

### Vision Grids Not Available
1. Ensure compiled with vision-grids feature
2. Check OpenCV installation
3. Verify LIBCLANG_PATH environment variable

### Cursor Positioning Inaccuracy
1. Increase grid density for finer control
2. Use recursive subdivision with homogeneous grid
3. Check screen resolution and scaling settings

## Tips and Best Practices

### Efficiency Tips
- **Learn one grid type well** rather than switching frequently
- **Use muscle memory** - QWERTY grid leverages existing keyboard familiarity
- **Combine with keyboard shortcuts** - pointless moves cursor, then use Ctrl+C, Enter, etc.
- **Practice common patterns** - develop workflows for your most frequent tasks

### Accessibility
- **High contrast** - use bright colors on dark backgrounds: `--color FFFF00`
- **Large grid cells** - lower density for easier targeting: `--grid-density 2` 
- **Alternative modifiers** - if Super key is difficult: `--modifier-key alt`

### Integration with Other Tools
- **Use with xdotool** - pointless positions, xdotool clicks:
  ```bash
  pointless & xdotool click 1
  ```
- **Combine with window managers** - i3, sway, etc. for complete keyboard control
- **Script common actions** - shell scripts combining pointless with other commands

## Advanced Usage

### Multiple Displays
Pointless works per-display. Run separate instances for multi-monitor setups:
```bash
DISPLAY=:0.0 pointless &
DISPLAY=:0.1 pointless --modifier-key alt &
```

### Custom Color Schemes
Match your desktop theme:
```bash
# Dark theme - bright yellow grid
pointless --color FFFF00 --opacity 0.8

# Light theme - dark blue grid  
pointless --color 0000AA --opacity 0.7

# High contrast - white grid
pointless --color FFFFFF --opacity 0.9
```

### Performance Optimization
For resource-constrained systems:
```bash
# Minimal resource usage
pointless --grid-type homogeneous --grid-density 2 --opacity 0.5

# Maximum performance
pointless --grid-type arrow --opacity 0.3
```

This usage guide covers the essential workflows and customization options for pointless. Experiment with different grid types and settings to find what works best for your specific use cases!