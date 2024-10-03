use crate::{resources::screen_server::{GameState, ScreenServer}, DrawContext, RendererContext, ServerContext};

// TODO maybe remove action anum and use strings instead

#[allow(unused_variables)]
pub trait Screen
where Self: Send + Sync + 'static {
    fn game_state(&self) -> GameState;

    fn start(&mut self,
        draw_ctx: &mut DrawContext,
        renderer_ctx: &mut RendererContext,
        server_ctx: &mut ServerContext,
        screen_server: &mut ScreenServer,
    ) {}

    fn ui(&mut self,
        draw_ctx: &mut DrawContext,
        renderer_ctx: &mut RendererContext,
        server_ctx: &mut ServerContext
    ) {}

    fn draw(&mut self,
        draw_ctx: &mut DrawContext,
        renderer_ctx: &mut RendererContext,
        server_ctx: &mut ServerContext
    ) {}

    fn update(&mut self,
        draw_ctx: &mut DrawContext,
        renderer_ctx: &mut RendererContext,
        server_ctx: &mut ServerContext
    ) {}
}
