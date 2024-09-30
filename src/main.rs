use bevy_ecs::world::World;
use wxpg::{app::App, resources::{input::{InputRes, KeyState}, mouse::MouseRes}, run};
use winit::{event::ElementState, keyboard::KeyCode};

#[derive(Default)]
struct AppExample {}

impl App for AppExample {
    fn start(&mut self) {
        println!("hi");
    }

    fn input(&mut self,
        world: &mut World,
        keycode: &KeyCode,
        key_state: &ElementState
    ) {
        let mut input_res = world
            .resource_mut::<InputRes>();
        
        match keycode {
            KeyCode::KeyW => input_res.forward = KeyState::from(key_state),
            KeyCode::KeyA => input_res.left = KeyState::from(key_state),
            KeyCode::KeyS => input_res.backward = KeyState::from(key_state),
            KeyCode::KeyD => input_res.right = KeyState::from(key_state),
            _ => {},
        }
    }

    fn mouse_moved(&mut self, world: &mut World, delta: (f64, f64)) {
        let mut mouse_res = world
            .resource_mut::<MouseRes>();

        mouse_res.pos.0 += delta.0;
        mouse_res.pos.1 += delta.1;
    }
}

fn main() {
    let app = AppExample::default();
    run(app);
}
