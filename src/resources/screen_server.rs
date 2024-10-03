use crate::{screens::screen::Screen, DrawContext, RendererContext, ServerContext};

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Cycle {
    Start,
    Ui,
    Draw,
    Update,
}

// TODO make this generic like Action

#[derive(Default)]
pub struct ScreenServer {
    state: GameState,
    registered_screens: Vec<Box<dyn Screen>>,
}

impl ScreenServer {
    pub fn draw(&mut self,
        draw_ctx: &mut DrawContext,
        renderer_ctx: &mut RendererContext,
        server_ctx: &mut ServerContext
    ) {
        self.emit_event(draw_ctx, renderer_ctx, server_ctx, Cycle::Draw);
        self.emit_event(draw_ctx, renderer_ctx, server_ctx, Cycle::Ui);
    }

    pub fn update(&mut self,
        draw_ctx: &mut DrawContext,
        renderer_ctx: &mut RendererContext,
        server_ctx: &mut ServerContext
    ) {
        self.emit_event(draw_ctx, renderer_ctx, server_ctx, Cycle::Update);
    }

    pub fn register_screen(&mut self, screen: impl Screen) {
        self.registered_screens.push(Box::new(screen));
    }

    fn emit_event(&mut self,
        draw_ctx: &mut DrawContext,
        renderer_ctx: &mut RendererContext,
        server_ctx: &mut ServerContext,
        cycle: Cycle
    ) {
        self.registered_screens
            .iter_mut()
            .for_each(|screen| {
                if screen.game_state() != self.state {
                    return;
                }

                match cycle {
                    Cycle::Start => screen.start(
                        draw_ctx, renderer_ctx, server_ctx, self),

                    Cycle::Update => screen.update(
                        draw_ctx, renderer_ctx, server_ctx),

                    Cycle::Ui => screen.ui(
                        draw_ctx, renderer_ctx, server_ctx),

                    Cycle::Draw => screen.draw(
                        draw_ctx, renderer_ctx, server_ctx),
                }
            });
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
        self.emit_event(Cycle::Start);
    }

    pub fn state(&self) -> GameState {
        self.state
    }
}
