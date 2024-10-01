use crate::resources::screen_server::GameState;

#[allow(unused_variables)]
pub trait Screen
where Self: Send + Sync + 'static {
    fn game_state(&self) -> GameState;

    fn start(&mut self) {}
    fn ui(&mut self) {}
    fn draw(&mut self) {}
    fn update(&mut self) {}
}
