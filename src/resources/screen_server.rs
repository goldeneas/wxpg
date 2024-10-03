use crate::{screens::screen::Screen, ScreenContext};

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
    last_state: Option<GameState>,
    registered_screens: Vec<Box<dyn Screen>>,
}

impl ScreenServer {
    pub fn draw(&mut self, screen_ctx: &mut ScreenContext) {
        if self.should_run_start_systems(screen_ctx) {
            self.set_last_state(screen_ctx.state);
            self.emit_event(screen_ctx, Cycle::Start);
        }

        self.emit_event(screen_ctx, Cycle::Draw);
        self.emit_event(screen_ctx, Cycle::Ui);
    }

    pub fn update(&mut self, screen_ctx: &mut ScreenContext) {
        if self.should_run_start_systems(screen_ctx) {
            self.set_last_state(screen_ctx.state);
            self.emit_event(screen_ctx, Cycle::Start);
        }

        self.emit_event(screen_ctx, Cycle::Update);
    }

    pub fn register_screen(&mut self, screen: impl Screen) {
        self.registered_screens.push(Box::new(screen));
    }

    fn emit_event(&mut self, screen_ctx: &mut ScreenContext, cycle: Cycle) {
        self.registered_screens
            .iter_mut()
            .for_each(|screen| {
                if screen.game_state() != self.last_state.unwrap() {
                    return;
                }

                match cycle {
                    Cycle::Start => screen.start(screen_ctx),
                    Cycle::Draw => screen.draw(screen_ctx),
                    Cycle::Update => screen.update(screen_ctx),
                    Cycle::Ui => screen.ui(screen_ctx),
                }
            });
    }

    fn set_last_state(&mut self, state: GameState) {
        self.last_state = Some(state);
    }

    fn should_run_start_systems(&self, screen_ctx: &ScreenContext) -> bool {
        match self.last_state {
            Some(last_state) => last_state != screen_ctx.state,
            None => true
        }
    }
}
