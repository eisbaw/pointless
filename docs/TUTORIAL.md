# Pointless Tutorial: From Mouse to Keyboard-Driven Cursor Control

*A step-by-step guide to mastering pointless for keyboard-driven productivity*

## Table of Contents
1. [Installation and Setup](#installation-and-setup)
2. [Your First Grid](#your-first-grid)
3. [Mastering Grid Types](#mastering-grid-types)
4. [Customization](#customization)
5. [Advanced Workflows](#advanced-workflows)
6. [Troubleshooting](#troubleshooting)

## Installation and Setup

### Prerequisites
- Linux with X11 display server
- Rust toolchain (for building from source)
- X11 development libraries

### Quick Installation
```bash
# Clone and build
git clone https://github.com/user/pointless
cd pointless
nix-shell --run 'just build'

# Or with vision grids
nix-shell --run 'just build-vision'
```

### Verify Installation
```bash
# Test basic functionality
./target/release/pointless --help

# Run quick test (press Super key, then Escape to exit)
./target/release/pointless
```

## Your First Grid

Let's start with the simplest possible workflow.

### Step 1: Launch Pointless
```bash
pointless
```
You should see a brief startup message, then the program waits in the background.

### Step 2: Activate the Grid
**Hold down the Super key** (Windows key on most keyboards).

You should see a yellow grid overlay appear on your screen with letters along the top (A, B, C...) and numbers along the side (1, 2, 3...).

### Step 3: Navigate the Grid
While holding Super:
1. **Press 'C'** (you should see the C column highlight)
2. **Press '2'** (cursor jumps to column C, row 2)
3. **Release Super** (grid disappears, cursor stays positioned)

### Step 4: Success!
You just positioned your cursor without touching the mouse! The cursor is now at the intersection of column C and row 2.

### Practice Exercise
Try clicking on different areas of your screen:
- **Top-left corner:** Hold Super → Press 'A' → Press '1' → Release Super
- **Center of screen:** Hold Super → Press 'D' → Press '3' → Release Super  
- **Bottom-right corner:** Hold Super → Press 'H' → Press '6' → Release Super

## Mastering Grid Types

Now let's explore different grid types for different situations.

### Homogeneous Grid (Default)
**Best for:** Precise positioning, learning the system

**How it works:**
- Screen divided into uniform cells
- Two-step selection: column + row
- Can subdivide cells for finer control

**Practice:** Try these exercises:
1. Click on your browser's address bar
2. Position cursor at the end of a text line
3. Click on a small button or icon

### QWERTY Grid
**Best for:** Speed and intuition

```bash
pointless --grid-type qwerty
```

**How it works:**
- Your keyboard layout projected onto the screen
- Single key press for quick jumps
- 'Q' = top-left, 'P' = top-right, 'Z' = bottom-left, 'M' = bottom-right

**Practice exercises:**
1. **Quick corner navigation:**
   - Top-left: Hold Super → Press 'Q' → Release Super
   - Top-right: Hold Super → Press 'P' → Release Super
   - Bottom-left: Hold Super → Press 'Z' → Release Super
   - Bottom-right: Hold Super → Press 'M' → Release Super

2. **Middle areas:**
   - Center-left: Hold Super → Press 'A' → Release Super
   - Center: Hold Super → Press 'G' → Release Super
   - Center-right: Hold Super → Press 'L' → Release Super

### Arrow Key Grid
**Best for:** Simple, directional thinking

```bash
pointless --grid-type arrow
```

**How it works:**
- 3x3 grid centered on current cursor
- Arrow keys move selection
- Enter subdivides, Escape goes up a level

**Practice exercises:**
1. **Basic navigation:**
   - Hold Super → Press Down Arrow → Press Right Arrow → Release Super
   - Hold Super → Press Up Arrow → Press Left Arrow → Release Super

2. **Subdivision for precision:**
   - Hold Super → Press Right Arrow → Press Enter (subdivides) → Press Down Arrow → Release Super

3. **Multi-level navigation:**
   - Hold Super → Press Down → Enter → Right → Enter → Up → Release Super

### Vim Grid
**Best for:** Vim users, advanced patterns

```bash
pointless --grid-type vim
```

**How it works:**
- h/j/k/l movement (left/down/up/right)
- Number prefixes (5h = move left 5 times)
- Jump commands (w/b for patterns, f{1-9} for targets)
- Screen positioning (H/M/L, 0/$, gg/G)

**Practice exercises:**

1. **Basic movement:**
   - Hold Super → Press 'j' 5 times (move down) → Press 'l' 3 times (move right) → Release Super
   - Hold Super → Type '10h' (move left 10 steps) → Release Super

2. **Screen positioning:**
   - Top of screen: Hold Super → Press 'H' → Release Super
   - Middle: Hold Super → Press 'M' → Release Super
   - Bottom: Hold Super → Press 'L' → Release Super
   - Left edge: Hold Super → Press '0' → Release Super
   - Right edge: Hold Super → Press '$' → Release Super

3. **Jump patterns:**
   - Hold Super → Press 'w' (forward jump mode) → Press '3' (jump to target 3) → Release Super
   - Hold Super → Press 'b' (backward jump mode) → Press '1' (jump to target 1) → Release Super

## Customization

### Change the Modifier Key
If Super key conflicts with your window manager:

```bash
# Use Alt key instead
pointless --modifier-key alt

# Use Ctrl key (be careful with conflicts)
pointless --modifier-key ctrl
```

### Adjust Visual Appearance

**Grid Color:**
```bash
# Red grid
pointless --color FF0000

# Blue grid
pointless --color 0000FF

# Green grid
pointless --color 00FF00
```

**Grid Opacity:**
```bash
# More transparent (easier to see underlying content)
pointless --opacity 0.3

# More opaque (easier to see grid)
pointless --opacity 0.9
```

**Grid Density:**
```bash
# Finer grid (more precision)
pointless --grid-density 5

# Coarser grid (simpler, faster)
pointless --grid-density 2
```

### Combine Options
```bash
# Perfect for web browsing
pointless --grid-type qwerty --opacity 0.4 --color FFFF00

# Great for code editing
pointless --grid-type homogeneous --grid-density 4 --opacity 0.7

# Ideal for presentations
pointless --modifier-key ctrl --color FFFFFF --opacity 0.9
```

## Advanced Workflows

### Web Browsing Workflow
1. **Setup:**
   ```bash
   pointless --grid-type qwerty --opacity 0.5 &
   ```

2. **Click links quickly:**
   - See a link you want? Hold Super → Press key near link position → Release Super → Click

3. **Navigate pages:**
   - Top of page: Hold Super → Press 'Q' → Release Super
   - Bottom of page: Hold Super → Press 'M' → Release Super
   - Scroll area: Hold Super → Press 'G' (center) → Release Super

### Code Editing Workflow
1. **Setup for precision:**
   ```bash
   pointless --grid-type homogeneous --grid-density 4 &
   ```

2. **Position cursor precisely:**
   - Hold Super → Navigate to exact character position → Release Super → Start typing

3. **Quick method navigation:**
   - Hold Super → Jump to function/method location → Release Super → Navigate with keyboard

### Design Work Workflow
1. **Use vision grids (if available):**
   ```bash
   pointless --grid-type rectangle-fitting --opacity 0.8 &
   ```

2. **Element selection:**
   - Vision grid automatically detects UI elements
   - Hold Super → Press number for element → Release Super

### Multi-Application Workflow
Run multiple instances with different settings:

```bash
# Terminal 1: General use
pointless --modifier-key super &

# Terminal 2: Precise work  
pointless --modifier-key alt --grid-density 5 &

# Terminal 3: Quick navigation
pointless --modifier-key ctrl --grid-type qwerty &
```

## Real-World Practice Scenarios

### Scenario 1: Email Management
**Goal:** Quickly navigate and click through email interface

1. Start pointless: `pointless --grid-type qwerty`
2. Open your email client
3. Practice:
   - Hold Super → Click on email in list → Release Super → Read
   - Hold Super → Click reply button → Release Super → Compose
   - Hold Super → Click send button → Release Super → Send

### Scenario 2: Form Filling
**Goal:** Navigate complex forms without mouse

1. Start pointless: `pointless --grid-type homogeneous --grid-density 3`
2. Open a web form
3. Practice:
   - Hold Super → Click first field → Release Super → Type
   - Tab to next field OR Hold Super → Click next field → Release Super
   - Repeat until form complete
   - Hold Super → Click submit → Release Super

### Scenario 3: Image Editing
**Goal:** Precise tool selection and canvas positioning

1. Start pointless: `pointless --grid-type homogeneous --grid-density 5`
2. Open image editor (GIMP, etc.)
3. Practice:
   - Hold Super → Click tool in toolbar → Release Super
   - Hold Super → Position precisely on canvas → Release Super → Perform action
   - Hold Super → Click different tool → Release Super

### Scenario 4: Code Review
**Goal:** Navigate large codebases efficiently

1. Start pointless: `pointless --grid-type vim`
2. Open code editor with large file
3. Practice:
   - Hold Super → Type 'H' (jump to top) → Release Super
   - Hold Super → Type '10j' (move down 10 lines) → Release Super  
   - Hold Super → Type 'w' → Type '2' (jump to target 2) → Release Super

## Troubleshooting

### Grid Doesn't Appear
**Problem:** Hold Super key but no grid shows up

**Solutions:**
1. Check if Super key conflicts with window manager
2. Try different modifier: `pointless --modifier-key alt`
3. Verify pointless is running: `ps aux | grep pointless`
4. Check X11 connection: `echo $DISPLAY`

### Grid Appears but Key Presses Don't Work
**Problem:** Grid shows but navigation keys don't respond

**Solutions:**
1. Make sure you're holding the modifier key while pressing navigation keys
2. Check keyboard layout matches expected (QWERTY vs DVORAK, etc.)
3. Try with debug mode: `pointless --debug`
4. Verify no other application is capturing keystrokes

### Cursor Positioning is Inaccurate
**Problem:** Cursor doesn't end up where expected

**Solutions:**
1. Increase grid density: `--grid-density 4`
2. Use homogeneous grid for more precision
3. Check screen resolution and DPI scaling
4. Try subdivision with arrow grid (Enter key)

### Performance Issues
**Problem:** Grid appears slowly or system becomes sluggish

**Solutions:**
1. Reduce opacity: `--opacity 0.3`
2. Use simpler grid type: `--grid-type arrow`
3. Lower grid density: `--grid-density 2`  
4. Avoid vision grids on slower hardware
5. Close other resource-intensive applications

### Vision Grids Not Available
**Problem:** Vision grid types not recognized

**Solutions:**
1. Rebuild with vision feature: `cargo build --features vision-grids`
2. Check OpenCV installation: `pkg-config --libs opencv4`
3. Verify environment variables: `echo $LIBCLANG_PATH`
4. Use basic grid types as fallback

## Next Steps

### Practice Routine
Spend 10-15 minutes daily practicing:
1. **Week 1:** Master homogeneous grid
2. **Week 2:** Learn QWERTY grid  
3. **Week 3:** Try arrow key grid
4. **Week 4:** Explore vim grid (if applicable)
5. **Week 5+:** Customize and optimize for your workflows

### Integration Tips
- **Start gradually:** Use pointless for one application at a time
- **Build muscle memory:** Practice the same movements repeatedly
- **Customize per task:** Different grid types for different work
- **Combine tools:** Use pointless with xdotool, window managers, etc.

### Community and Development
- Report issues and request features
- Share your workflows and configurations
- Contribute to documentation and examples
- Help improve vision grid implementations

Congratulations! You now have the knowledge to replace your mouse with keyboard-driven cursor control. Practice these techniques and develop your own workflows for maximum productivity.

**Remember:** The goal isn't to never use a mouse, but to have efficient keyboard alternatives when they're faster or more convenient.

Happy cursor positioning! 🎯