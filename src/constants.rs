// X11 Keycode mappings for consistent input handling
// These are standard X11 keycodes that should work across most systems

// Letter key mappings (A-Z)
pub const KEYCODE_A: u32 = 38;
pub const KEYCODE_B: u32 = 56;
pub const KEYCODE_C: u32 = 54;
pub const KEYCODE_D: u32 = 40;
pub const KEYCODE_E: u32 = 26;
pub const KEYCODE_F: u32 = 41;
pub const KEYCODE_G: u32 = 42;
pub const KEYCODE_H: u32 = 43;
pub const KEYCODE_I: u32 = 31;
pub const KEYCODE_J: u32 = 44;
pub const KEYCODE_K: u32 = 45;
pub const KEYCODE_L: u32 = 46;
pub const KEYCODE_M: u32 = 58;
pub const KEYCODE_N: u32 = 57;
pub const KEYCODE_O: u32 = 32;
pub const KEYCODE_P: u32 = 33;
pub const KEYCODE_Q: u32 = 24;
pub const KEYCODE_R: u32 = 27;
pub const KEYCODE_S: u32 = 39;
pub const KEYCODE_T: u32 = 28;
pub const KEYCODE_U: u32 = 30;
pub const KEYCODE_V: u32 = 55;
pub const KEYCODE_W: u32 = 25;
pub const KEYCODE_X: u32 = 53;
pub const KEYCODE_Y: u32 = 29;
pub const KEYCODE_Z: u32 = 52;

// Number key mappings (0-9)
pub const KEYCODE_1: u32 = 10;
pub const KEYCODE_2: u32 = 11;
pub const KEYCODE_3: u32 = 12;
pub const KEYCODE_4: u32 = 13;
pub const KEYCODE_5: u32 = 14;
pub const KEYCODE_6: u32 = 15;
pub const KEYCODE_7: u32 = 16;
pub const KEYCODE_8: u32 = 17;
pub const KEYCODE_9: u32 = 18;
pub const KEYCODE_0: u32 = 19;

// Special keys
pub const KEYCODE_ESCAPE: u32 = 9;
pub const KEYCODE_ENTER: u32 = 36;
pub const KEYCODE_SPACE: u32 = 65;

// Arrow keys
pub const KEYCODE_UP: u32 = 111;
pub const KEYCODE_DOWN: u32 = 116;
pub const KEYCODE_LEFT: u32 = 113;
pub const KEYCODE_RIGHT: u32 = 114;

// Additional special characters
pub const KEYCODE_DOLLAR: u32 = 21; // Shift+4

// Grid configuration constants
pub const DEFAULT_GRID_SIZE: u32 = 4;
pub const MAX_GRID_SIZE: u32 = 6;
pub const MAX_SUBDIVISION_LEVELS: u32 = 3;

// Performance constants
pub const TARGET_UPDATE_LATENCY_MS: u64 = 20;
pub const MAX_CELLS_PER_GRID: usize = 100;

// Visual constants
pub const GRID_BORDER_WIDTH: u32 = 1;
pub const TEXT_VERTICAL_OFFSET: i32 = 6;
pub const CHAR_WIDTH_ESTIMATE: usize = 4;
