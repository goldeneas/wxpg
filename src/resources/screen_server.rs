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

#[derive(Default)]
pub struct ScreenServer {
    curr_state: GameState,
    registered_screens: Vec<Box<dyn Screen>>,
    has_state_changed: bool,
}

impl ScreenServer {
    pub fn draw(&mut self) {
        if self.should_run_start_systems() {
            self.emit_event(Cycle::Start);
            self.has_state_changed = false;
        }

        self.emit_event(Cycle::Draw);
        self.emit_event(Cycle::Ui);
    }

    pub fn update(&mut self) {
        if self.should_run_start_systems() {
            self.emit_event(Cycle::Start);
            self.has_state_changed = false;
        }

        self.emit_event(Cycle::Update);
    }

    pub fn register_screen(&mut self, screen: impl Screen) {
        self.registered_screens.push(Box::new(screen));
    }

    fn emit_event(&mut self, cycle: Cycle) {
        self.registered_screens
            .iter_mut()
            .for_each(|screen| {
                if screen.game_state() != self.curr_state {
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

    pub fn set_active_state(&mut self, state: GameState) {
        self.curr_state = state;
        self.has_state_changed = true;
    }

    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.curr_state
    }

    fn should_run_start_systems(&self) -> bool {
        self.has_state_changed
    }
}
