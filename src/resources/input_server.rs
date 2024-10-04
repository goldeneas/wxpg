use std::collections::HashMap;

use winit::{event::ElementState, keyboard::KeyCode};

// TODO: is this static fine?
type Action = &'static str;

#[derive(Debug, Default)]
pub struct InputServer {
    mouse_delta: (f64, f64),
    action_map: HashMap<Action, KeyCode>,
    key_map: HashMap<KeyCode, ElementState>,
}

impl InputServer {
    pub fn action(&self, action: Action) -> bool {
        let keycode = self.action_map.get(&action)
            .unwrap();

        match self.key_map.get(&keycode) {
            None => false,
            Some(state) => state.is_pressed()
        }
    }

    pub fn mouse_delta(&mut self, delta: (f64, f64)) {
        self.mouse_delta = delta;
    }

    pub fn keyboard_input(&mut self, keycode: KeyCode, state: ElementState) {
        self.key_map.insert(keycode, state);
    }

    pub fn register_action(&mut self, action: Action, keycode: KeyCode) {
        self.action_map.insert(action, keycode);
    }

    pub fn unregister_action(&mut self, action: Action) {
        self.action_map.remove(&action);
    }
}
