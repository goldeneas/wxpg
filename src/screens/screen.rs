use crate::{resources::screen_server::GameState, DrawContext, RendererContext};

#[allow(unused_variables)]
pub trait Screen
where Self: Send + Sync + 'static {
    fn game_state(&self) -> GameState;

    fn start(&mut self, renderer_ctx: &mut RendererContext) {}
    fn ui(&mut self, draw_ctx: &mut DrawContext) {}
    fn draw(&mut self, draw_ctx: &mut DrawContext) {}
    fn update(&mut self) {}
}
