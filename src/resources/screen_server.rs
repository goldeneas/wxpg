use crate::screens::screen::Screen;

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
    pub fn draw(&mut self) {
        self.emit_event(Cycle::Draw);
        self.emit_event(Cycle::Ui);
    }

    pub fn update(&mut self) {
        self.emit_event(Cycle::Update);
    }

    pub fn register_screen(&mut self, screen: impl Screen) {
        self.registered_screens.push(Box::new(screen));
    }

    fn emit_event(&mut self, cycle: Cycle) {
        self.registered_screens
            .iter_mut()
            .for_each(|screen| {
                if screen.game_state() != self.state {
                    return;
                }

                match cycle {
                    Cycle::Start => screen.start(),
                    Cycle::Update => screen.update(),
                    Cycle::Ui => screen.ui(),
                    Cycle::Draw => screen.draw(),
                }
            });
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
        self.emit_event(Cycle::Start);
    }

    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }
}
