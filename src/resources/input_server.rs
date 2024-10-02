use std::{collections::HashMap, default, fmt::Debug, hash::Hash};

use winit::{event::ElementState, keyboard::KeyCode};

pub trait Action: Debug + Eq + Hash + Send + Sync + Clone + Copy {}

#[derive(Debug)]
pub struct InputServer<T: Action> {
    mouse_delta: (f64, f64),
    action_map: HashMap<T, KeyCode>,
    key_map: HashMap<KeyCode, ElementState>,
}

impl<T: Action> Default for InputServer<T> {
    fn default() -> Self {
        let action_map = HashMap::default();
        let key_map = HashMap::default();
        let mouse_delta = <(f64, f64)>::default();

        Self {
            action_map,
            key_map,
            mouse_delta,
        }
    }
}

impl<T: Action> InputServer<T> {
    pub fn action(&self, action: T) -> bool {
        let keycode = self.action_map.get(&action)
            .unwrap();

        self.key_map.get(&keycode)
            .unwrap()
            .is_pressed()
    }

    pub fn mouse_delta(&mut self, delta: (f64, f64)) {
        self.mouse_delta = delta;
    }

    pub fn keyboard_input(&mut self, keycode: KeyCode, state: ElementState) {
        self.key_map.insert(keycode, state);
    }

    pub fn register_action(&mut self, action: T, keycode: KeyCode) {
        self.action_map.insert(action, keycode);
    }

    pub fn unregister_action(&mut self, action: T) {
        self.action_map.remove(&action);
    }
}
