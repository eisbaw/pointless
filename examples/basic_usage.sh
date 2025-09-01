#!/bin/bash
# Basic usage examples for pointless - keyboard-driven cursor control

echo "=== Pointless Basic Usage Examples ==="
echo

# Example 1: Default usage
echo "1. Default homogeneous grid (yellow, Super key activation):"
echo "   pointless"
echo "   → Hold Super key, press column letter + row number (e.g., 'B3')"
echo

# Example 2: Alternative modifier key
echo "2. Using Alt key instead of Super (useful if Super conflicts with WM):"
echo "   pointless --modifier-key alt"
echo "   → Hold Alt key instead of Super"
echo

# Example 3: QWERTY layout
echo "3. QWERTY keyboard projection (intuitive for typists):"
echo "   pointless --grid-type qwerty"
echo "   → Hold Super, press any key (Q=top-left, P=top-right, etc.)"
echo

# Example 4: Arrow key navigation
echo "4. Arrow key navigation (simple directional control):"
echo "   pointless --grid-type arrow"
echo "   → Hold Super, use arrow keys, Enter to subdivide, Escape to go up"
echo

# Example 5: Vim-style navigation
echo "5. Vim-style navigation (for vim users):"
echo "   pointless --grid-type vim"
echo "   → Hold Super, use h/j/k/l, w/b for jumps, H/M/L for screen positions"
echo

# Example 6: Custom appearance
echo "6. Custom grid color and opacity:"
echo "   pointless --color FF0000 --opacity 0.5"
echo "   → Red grid with 50% transparency"
echo

# Example 7: High precision
echo "7. High precision grid (fine cursor control):"
echo "   pointless --grid-type homogeneous --grid-density 5"
echo "   → More subdivision levels for precise positioning"
echo

# Example 8: Debug mode
echo "8. Debug mode (troubleshooting and development):"
echo "   pointless --debug"
echo "   → Additional logging for debugging issues"
echo

echo "=== Quick Start Workflow ==="
echo "1. Run: pointless"
echo "2. Hold Super key (Windows key)"
echo "3. Grid appears - navigate using your chosen method"
echo "4. Release Super key - cursor positioned, grid disappears"
echo "5. Use normal clicking, typing, or keyboard shortcuts"
echo

echo "=== Common Combinations ==="
echo "Web browsing:     pointless --grid-type qwerty --opacity 0.6"
echo "Code editing:     pointless --grid-type homogeneous --grid-density 4"  
echo "Design work:      pointless --grid-type vim --color 00FFFF"
echo "Presentations:    pointless --modifier-key ctrl --color FFFFFF --opacity 0.9"
echo

echo "Try these examples and find what works best for your workflow!"
echo "For more details, see: man pointless"