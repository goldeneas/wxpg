use bevy_ecs::world::World;
use winit::{event::ElementState, keyboard::KeyCode};

#[allow(unused_variables)]
pub trait App {
    fn start(&mut self);
    fn input(&mut self,
        world: &mut World,
        keycode: &KeyCode,
        key_state: &ElementState) {}
    fn mouse_moved(&mut self, world: &mut World, delta: (f64, f64)) {}
}
