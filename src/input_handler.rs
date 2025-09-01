use crate::config::ModifierKey;
use crate::error::{PointlessError, Result};
use crate::types::KeyCode;
use crate::x11_client::X11Client;
use std::ffi::CString;
use x11::keysym;
use x11::xlib::{
    ControlMask, KeyPress, KeyPressMask, KeyRelease, KeyReleaseMask, Mod1Mask, Mod4Mask, XEvent,
    XKeysymToKeycode, XStringToKeysym,
};

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    ModifierPressed,
    ModifierReleased,
    NavigationKey(KeyCode),
    Unknown,
}

#[derive(Debug)]
pub struct InputHandler {
    modifier_key: ModifierKey,
    modifier_keycode: KeyCode,
    modifier_mask: u32,
    is_modifier_pressed: bool,
}

impl InputHandler {
    pub fn new(modifier_key: ModifierKey, x11_client: &X11Client) -> Result<Self> {
        let (modifier_keycode, modifier_mask) = Self::get_modifier_info(&modifier_key, x11_client)?;

        Ok(InputHandler {
            modifier_key,
            modifier_keycode,
            modifier_mask,
            is_modifier_pressed: false,
        })
    }

    fn get_modifier_info(
        modifier_key: &ModifierKey,
        x11_client: &X11Client,
    ) -> Result<(KeyCode, u32)> {
        unsafe {
            let (keysym_name, modifier_mask) = match modifier_key {
                ModifierKey::Super => ("Super_L", Mod4Mask),
                ModifierKey::Alt => ("Alt_L", Mod1Mask),
                ModifierKey::Ctrl => ("Control_L", ControlMask),
            };

            let keysym_cstring = CString::new(keysym_name).unwrap();
            let keysym = XStringToKeysym(keysym_cstring.as_ptr());

            if keysym == 0 {
                return Err(PointlessError::ModifierKeyNotFound {
                    key_name: keysym_name.to_string(),
                });
            }

            let keycode = XKeysymToKeycode(x11_client.display(), keysym) as KeyCode;

            if keycode == 0 {
                return Err(PointlessError::InvalidKeycode { keycode });
            }

            Ok((keycode, modifier_mask))
        }
    }

    pub fn start_capture(&mut self, x11_client: &X11Client) -> Result<()> {
        // Select key press and release events
        x11_client.select_input(KeyPressMask | KeyReleaseMask);

        // Grab the modifier key
        x11_client.grab_key(self.modifier_keycode, 0)?;

        Ok(())
    }

    pub fn stop_capture(&mut self, x11_client: &X11Client) {
        // Ungrab the modifier key
        x11_client.ungrab_key(self.modifier_keycode, 0);
    }

    pub fn process_event(&mut self, event: &XEvent) -> InputEvent {
        unsafe {
            match event.type_ {
                KeyPress => {
                    let key_event = event.key;
                    let keycode = key_event.keycode as KeyCode;

                    if keycode == self.modifier_keycode {
                        self.is_modifier_pressed = true;
                        return InputEvent::ModifierPressed;
                    } else if self.is_modifier_pressed {
                        // Navigation key pressed while modifier is held
                        return InputEvent::NavigationKey(keycode);
                    }
                }
                KeyRelease => {
                    let key_event = event.key;
                    let keycode = key_event.keycode as KeyCode;

                    if keycode == self.modifier_keycode {
                        self.is_modifier_pressed = false;
                        return InputEvent::ModifierReleased;
                    }
                }
                _ => {}
            }
        }

        InputEvent::Unknown
    }

    pub fn is_modifier_pressed(&self) -> bool {
        self.is_modifier_pressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ModifierKey;

    // Note: These tests require X11 connection and are integration tests
    // They should be run with `cargo test --test integration`
}
