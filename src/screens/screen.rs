use bevy_ecs::world::World;

use crate::{resources::screen_server::GameState, EngineResources};

#[allow(unused_variables)]
pub trait Screen
where Self: Send + Sync + 'static {
    fn game_state(&self) -> GameState;

    fn start(&mut self, resources: &mut EngineResources) {}
    fn ui(&mut self, resources: &mut EngineResources) {}
    fn draw(&mut self, resources: &mut EngineResources) {}
    fn update(&mut self, resources: &mut EngineResources) {}
}
