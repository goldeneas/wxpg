use crate::{screens::screen::Screen, EngineResources};

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
    last_state: Option<GameState>,
    registered_screens: Vec<Box<dyn Screen>>,
}

impl ScreenServer {
    pub fn draw(&mut self, resources: &mut EngineResources) {
        let state = self.curr_state;

        if self.should_run_start_systems(state) {
            self.set_last_state(state);
            self.emit_event(resources, Cycle::Start);
        }

        self.emit_event(resources, Cycle::Draw);
        self.emit_event(resources, Cycle::Ui);
    }

    pub fn update(&mut self, resources: &mut EngineResources) {
        let state = self.curr_state;

        if self.should_run_start_systems(state) {
            self.set_last_state(state);
            self.emit_event(resources, Cycle::Start);
        }

        self.emit_event(resources, Cycle::Update);
    }

    pub fn register_screen(&mut self, screen: impl Screen) {
        self.registered_screens.push(Box::new(screen));
    }

    fn emit_event(&mut self, resources: &mut EngineResources, cycle: Cycle) {
        self.registered_screens
            .iter_mut()
            .for_each(|screen| {
                if screen.game_state() != self.last_state.unwrap() {
                    return;
                }

                match cycle {
                    Cycle::Start => screen.start(resources),
                    Cycle::Update => screen.update(resources),
                    Cycle::Ui => screen.ui(resources),
                    Cycle::Draw => screen.draw(resources),
                }
            });
    }

    fn set_last_state(&mut self, state: GameState) {
        self.last_state = Some(state);
    }

    fn should_run_start_systems(&self, state: GameState) -> bool {
        match self.last_state {
            Some(last_state) => last_state != state,
            None => true
        }
    }
}
