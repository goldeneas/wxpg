use std::collections::HashMap;

use log::debug;

use crate::{screens::screen::Screen, EngineInternal};

use super::commands::Commands;

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Cycle {
    Start,
    Ui,
    Draw,
    Update,
}

// TODO make this generic like Action

pub struct ScreenServer {
    state: GameState,
    next_state: Option<GameState>,
    registered_screens: HashMap<GameState, Vec<Box<dyn Screen>>>,
}

impl Default for ScreenServer {
    fn default() -> Self {
        let next_state = Some(GameState::default());
        let state = GameState::default();
        let registered_screens = HashMap::default();

        Self {
            next_state,
            state,
            registered_screens,
        }
    }
}

impl ScreenServer {
    pub fn draw(&mut self, engine_internal: &mut EngineInternal) {
        if self.should_state_change() {
            self.update_state();
            self.emit_event(Cycle::Start, engine_internal);
        }

        self.emit_event(Cycle::Draw, engine_internal);
        self.emit_event(Cycle::Ui, engine_internal);
    }

    pub fn update(&mut self, engine_internal: &mut EngineInternal) {
        if self.should_state_change() {
            self.update_state();
            self.emit_event(Cycle::Start, engine_internal);
        }

        self.emit_event(Cycle::Update, engine_internal);
    }

    pub fn register_screen(&mut self, state: GameState, screen: impl Screen) {
        let screen = Box::new(screen);

        match self.registered_screens.get_mut(&state) {
            Some(vec) => vec.push(screen),
            None => {
                let vec: Vec<Box<dyn Screen>> = vec![screen];
                self.registered_screens.insert(state, vec);
            }
        }

    }

    fn emit_event(&mut self,
        cycle: Cycle,
        engine_internal: &mut EngineInternal
    ) {
        let screens_opt = self.registered_screens
            .get_mut(&self.state);

        if screens_opt.is_none() {
            debug!("Game state has no screens registered.");
            return;
        }

        let mut commands = Commands::new(engine_internal);

        let screens = screens_opt.unwrap();
        screens.iter_mut()
            .for_each(|screen| {
                match cycle {
                    Cycle::Start => screen.start(&mut commands),
                    Cycle::Draw => screen.draw(&mut commands),
                    Cycle::Update => screen.update(&mut commands),
                    Cycle::Ui => screen.ui(&mut commands),
                }
            });

        self.next_state = commands.new_state;
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    fn update_state(&mut self) {
        self.state = self.next_state.unwrap();
        self.next_state = None;
    }

    fn should_state_change(&self) -> bool {
        self.next_state.is_some()
    }
}
