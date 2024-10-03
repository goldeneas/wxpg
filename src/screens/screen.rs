use crate::{resources::screen_server::{GameState, ScreenContext, ScreenServer}, DrawContext, RendererContext, ServerContext};

#[allow(unused_variables)]
pub trait Screen where Self: 'static {
    fn game_state(&self) -> GameState;

    fn start(&mut self, screen_ctx: &mut ScreenContext) {}
    fn ui(&mut self, screen_ctx: &mut ScreenContext) {}
    fn draw(&mut self, screen_ctx: &mut ScreenContext) {}
    fn update(&mut self, screen_ctx: &mut ScreenContext) {}
}
