use std::collections::HashMap;

use glutin::event::{ElementState, VirtualKeyCode};

/// Keeps track of which keys have been pressed.
pub struct KeyboardState {
    state: HashMap<VirtualKeyCode, ElementState>,
}
impl KeyboardState {
    /// Constructs a new KeyboardState with all the keys released.
    pub fn new() -> KeyboardState {
        KeyboardState {
            state: HashMap::new(),
        }
    }

    /// Returns true if `key` is pressed.
    pub fn is_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.state
            .get(key)
            .map(|&s| s == ElementState::Pressed)
            .unwrap_or(false)
    }
    /// Returns true if `key` is released.
    // pub fn is_released(&self, key: &VirtualKeyCode) -> bool {
    //     !self.is_pressed(key)
    // }

    /// Processes a keyboard event and updated the internal state.
    pub fn process_event(&mut self, key_state: ElementState, code: VirtualKeyCode) {
        match key_state {
            ElementState::Pressed => {
                self.state.insert(code, key_state);
            }
            ElementState::Released => {
                self.state.remove(&code);
            }
        }
    }
}
