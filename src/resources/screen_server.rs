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
    commands: Commands,
    state: Option<GameState>,
    next_state: Option<GameState>,
    registered_screens: HashMap<GameState, Vec<Box<dyn Screen>>>,
}

impl ScreenServer {
    pub fn execute_commands(&mut self, engine_internal: &mut EngineInternal) {
        let new_state = self.commands.new_state();
        self.state = new_state;

        self.commands.funcs()
            .iter()
            .for_each(|func| { func(engine_internal); });

        self.commands = Commands::default();
    }

    pub fn draw(&mut self) {
        if self.should_state_change() {
            self.update_state();
            self.emit_event(Cycle::Start);
        }

        self.emit_event(Cycle::Draw);
        self.emit_event(Cycle::Ui);
    }

    pub fn update(&mut self) {
        if self.should_state_change() {
            self.update_state();
            self.emit_event(Cycle::Start);
        }

        self.emit_event(Cycle::Update);
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

    fn emit_event(&mut self, cycle: Cycle) {
        let state = self.state.unwrap();
        let screens_opt = self.registered_screens
            .get_mut(&state);

        if screens_opt.is_none() {
            debug!("Game state has no screens registered.");
            return;
        }

        let screens = screens_opt.unwrap();
        screens.iter_mut()
            .for_each(|screen| {
                let commands = &mut self.commands;
                match cycle {
                    Cycle::Start => screen.start(commands),
                    Cycle::Draw => screen.draw(commands),
                    Cycle::Update => screen.update(commands),
                    Cycle::Ui => screen.ui(commands),
                }
            });
    }

    pub fn commands(&mut self) -> &mut Commands {
        &mut self.commands
    }

    pub fn state(&self) -> Option<GameState> {
        self.state
    }

    fn update_state(&mut self) {
        self.state = self.next_state;
        self.next_state = None;
    }

    fn should_state_change(&self) -> bool {
        self.next_state.is_some()
    }
}
