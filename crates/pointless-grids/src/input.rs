use crate::KeyCode;
use log::warn;

/// X11 Keycode constants for consistent input handling across all grid implementations.
/// These are standard X11 keycodes that should work across most Linux systems.
pub mod keycodes {
    // Letter key mappings (A-Z)
    pub const A: u32 = 38;
    pub const B: u32 = 56;
    pub const C: u32 = 54;
    pub const D: u32 = 40;
    pub const E: u32 = 26;
    pub const F: u32 = 41;
    pub const G: u32 = 42;
    pub const H: u32 = 43;
    pub const I: u32 = 31;
    pub const J: u32 = 44;
    pub const K: u32 = 45;
    pub const L: u32 = 46;
    pub const M: u32 = 58;
    pub const N: u32 = 57;
    pub const O: u32 = 32;
    pub const P: u32 = 33;
    pub const Q: u32 = 24;
    pub const R: u32 = 27;
    pub const S: u32 = 39;
    pub const T: u32 = 28;
    pub const U: u32 = 30;
    pub const V: u32 = 55;
    pub const W: u32 = 25;
    pub const X: u32 = 53;
    pub const Y: u32 = 29;
    pub const Z: u32 = 52;

    // Number key mappings (0-9)
    pub const NUM_1: u32 = 10;
    pub const NUM_2: u32 = 11;
    pub const NUM_3: u32 = 12;
    pub const NUM_4: u32 = 13;
    pub const NUM_5: u32 = 14;
    pub const NUM_6: u32 = 15;
    pub const NUM_7: u32 = 16;
    pub const NUM_8: u32 = 17;
    pub const NUM_9: u32 = 18;
    pub const NUM_0: u32 = 19;

    // Special keys
    pub const ESCAPE: u32 = 9;
    pub const ENTER: u32 = 36;
    pub const SPACE: u32 = 65;

    // Arrow keys
    pub const UP: u32 = 111;
    pub const DOWN: u32 = 116;
    pub const LEFT: u32 = 113;
    pub const RIGHT: u32 = 114;

    // Additional special characters
    pub const DOLLAR: u32 = 21; // Shift+4
}

/// Maps X11 keycode to corresponding letter (A-Z).
/// 
/// # Arguments
/// * `keycode` - X11 keycode to convert
/// 
/// # Returns
/// * `Some(char)` - The corresponding uppercase letter if valid
/// * `None` - If keycode doesn't correspond to a letter
/// 
/// # Rationale
/// Centralized keycode-to-letter mapping eliminates duplication across grid implementations
/// and provides consistent behavior. Using uppercase letters follows Vim convention.
pub fn keycode_to_letter(keycode: KeyCode) -> Option<char> {
    match keycode {
        keycodes::A => Some('A'),
        keycodes::B => Some('B'),
        keycodes::C => Some('C'),
        keycodes::D => Some('D'),
        keycodes::E => Some('E'),
        keycodes::F => Some('F'),
        keycodes::G => Some('G'),
        keycodes::H => Some('H'),
        keycodes::I => Some('I'),
        keycodes::J => Some('J'),
        keycodes::K => Some('K'),
        keycodes::L => Some('L'),
        keycodes::M => Some('M'),
        keycodes::N => Some('N'),
        keycodes::O => Some('O'),
        keycodes::P => Some('P'),
        keycodes::Q => Some('Q'),
        keycodes::R => Some('R'),
        keycodes::S => Some('S'),
        keycodes::T => Some('T'),
        keycodes::U => Some('U'),
        keycodes::V => Some('V'),
        keycodes::W => Some('W'),
        keycodes::X => Some('X'),
        keycodes::Y => Some('Y'),
        keycodes::Z => Some('Z'),
        _ => {
            warn!("Unknown keycode {} cannot be mapped to letter", keycode);
            None
        }
    }
}

/// Maps X11 keycode to corresponding number (0-9).
/// 
/// # Arguments
/// * `keycode` - X11 keycode to convert
/// 
/// # Returns
/// * `Some(u32)` - The corresponding number if valid
/// * `None` - If keycode doesn't correspond to a number
/// 
/// # Special Cases
/// * Key '0' maps to number 0 (not 10), consistent with standard keyboard layout
/// 
/// # Rationale
/// Centralized number mapping with consistent behavior across all grid types.
/// Using u32 allows for multiplication operations in Vim-style numeric prefixes.
pub fn keycode_to_number(keycode: KeyCode) -> Option<u32> {
    match keycode {
        keycodes::NUM_1 => Some(1),
        keycodes::NUM_2 => Some(2),
        keycodes::NUM_3 => Some(3),
        keycodes::NUM_4 => Some(4),
        keycodes::NUM_5 => Some(5),
        keycodes::NUM_6 => Some(6),
        keycodes::NUM_7 => Some(7),
        keycodes::NUM_8 => Some(8),
        keycodes::NUM_9 => Some(9),
        keycodes::NUM_0 => Some(0),
        _ => {
            warn!("Unknown keycode {} cannot be mapped to number", keycode);
            None
        }
    }
}

/// Converts letter to column index for grid layouts.
/// 
/// # Arguments
/// * `letter` - Uppercase letter (A-Z)
/// * `max_columns` - Maximum valid column index
/// 
/// # Returns
/// * `Some(u32)` - Zero-based column index if valid
/// * `None` - If letter is invalid or exceeds max_columns
/// 
/// # Rationale
/// A=0, B=1, etc. follows standard grid indexing. The max_columns parameter
/// enables proper bounds checking with fail-fast error reporting.
pub fn letter_to_column(letter: char, max_columns: u32) -> Option<u32> {
    let col_index = (letter as u32).saturating_sub('A' as u32);
    if col_index < max_columns {
        Some(col_index)
    } else {
        warn!("Invalid column letter '{}' for grid with {} columns (max: {})", 
              letter, max_columns, max_columns - 1);
        None
    }
}

/// Converts number to row index for grid layouts.
/// 
/// # Arguments
/// * `number` - Number (0-9)
/// * `max_rows` - Maximum valid row index
/// 
/// # Returns
/// * `Some(u32)` - Zero-based row index if valid
/// * `None` - If number exceeds max_rows
/// 
/// # Special Cases
/// * Number 0 maps to row 9 (treating 0 as "row 10" in 1-indexed display)
/// * Numbers 1-9 map to rows 0-8 respectively
/// 
/// # Rationale
/// This mapping allows users to think in 1-indexed terms (A1, A2) while
/// internally using 0-indexed arrays. The 0->9 mapping follows Vim convention.
pub fn number_to_row(number: u32, max_rows: u32) -> Option<u32> {
    let row_index = if number == 0 { 9 } else { number - 1 };
    if row_index < max_rows {
        Some(row_index)
    } else {
        warn!("Invalid row number {} for grid with {} rows (max display: {})", 
              number, max_rows, max_rows);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keycode_to_letter() {
        assert_eq!(keycode_to_letter(keycodes::A), Some('A'));
        assert_eq!(keycode_to_letter(keycodes::Z), Some('Z'));
        assert_eq!(keycode_to_letter(999), None); // Invalid keycode
    }

    #[test]
    fn test_keycode_to_number() {
        assert_eq!(keycode_to_number(keycodes::NUM_1), Some(1));
        assert_eq!(keycode_to_number(keycodes::NUM_0), Some(0));
        assert_eq!(keycode_to_number(999), None); // Invalid keycode
    }

    #[test]
    fn test_letter_to_column() {
        assert_eq!(letter_to_column('A', 4), Some(0));
        assert_eq!(letter_to_column('D', 4), Some(3));
        assert_eq!(letter_to_column('E', 4), None); // Exceeds max_columns
    }

    #[test]
    fn test_number_to_row() {
        assert_eq!(number_to_row(1, 4), Some(0));
        assert_eq!(number_to_row(4, 4), Some(3));
        assert_eq!(number_to_row(0, 10), Some(9)); // Special case: 0 -> row 9
        assert_eq!(number_to_row(5, 4), None); // Exceeds max_rows
    }
}