use crate::resources::{commands::Commands, screen_server::GameState};

#[allow(unused_variables)]
pub trait Screen where Self: 'static {
    fn game_state(&self) -> GameState;

    fn start(&mut self, commands: &mut Commands) {}
    fn ui(&mut self, commands: &mut Commands) {}
    fn draw(&mut self, commands: &mut Commands) {}
    fn update(&mut self, commands: &mut Commands) {}
}
