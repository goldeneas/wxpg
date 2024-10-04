use std::collections::HashMap;

use winit::{event::ElementState, keyboard::KeyCode};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum KeyState {
    #[default]
    Released,
    Pressed,
    JustPressed,
    JustReleased,
}

impl From<ElementState> for KeyState {
    fn from(value: ElementState) -> Self {
        if value.is_pressed() {
            Self::Pressed
        } else {
            Self::Released
        }
    }
}

#[derive(Debug, Default)]
pub struct InputServer {
    mouse_delta: (f64, f64),
    action_map: HashMap<String, KeyCode>,
    key_states: HashMap<KeyCode, KeyState>,
}

impl InputServer {
    pub fn get_state(&self, action_name: &str) -> KeyState {
        let keycode = self.action_map.get(action_name)
            .expect("Tried getting state for an unknown action");

        match self.key_states.get(&keycode) {
            None => KeyState::Released,
            Some(state) => *state,
        }
    }

    pub fn mouse_delta(&mut self, delta: (f64, f64)) {
        self.mouse_delta = delta;
    }

    pub fn keyboard_input(&mut self, keycode: KeyCode, state: ElementState) {
        let state = KeyState::from(state);
        self.key_states.insert(keycode, state);
    }

    pub fn register_action(&mut self, action_name: &str, keycode: KeyCode) {
        self.action_map.insert(action_name.to_string(), keycode);
    }

    pub fn unregister_action(&mut self, action_name: &str) {
        self.action_map.remove(action_name);
    }
}
