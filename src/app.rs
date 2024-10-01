use winit::{event::ElementState, keyboard::KeyCode};

use crate::EngineContext;

pub trait App {
    fn start(&mut self, engine_ctx: &mut EngineContext);
    fn mouse_moved(&mut self, engine_ctx: &mut EngineContext, delta: (f64, f64));
    fn input(&mut self,
        resources: &mut EngineContext,
        keycode: &KeyCode,
        key_state: &ElementState
    );
}
