use winit::{event::ElementState, keyboard::KeyCode};

pub trait App {
    fn start(&mut self);
    fn input(&mut self, keycode: &KeyCode, key_state: &ElementState) {}
    fn mouse_moved(&mut self, delta: (f64, f64)) {}
}
