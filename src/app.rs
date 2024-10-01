use winit::{event::ElementState, keyboard::KeyCode};

use crate::EngineResources;

pub trait App {
    fn start(&mut self, resources: &mut EngineResources);
    fn mouse_moved(&mut self, resources: &mut EngineResources, delta: (f64, f64));
    fn input(&mut self,
        resources: &mut EngineResources,
        keycode: &KeyCode,
        key_state: &ElementState
    );
}
