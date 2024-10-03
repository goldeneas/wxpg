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
    registered_screens: Vec<Box<dyn Screen>>,
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

    pub fn register_screen(&mut self, screen: impl Screen) {
        self.registered_screens.push(Box::new(screen));
    }

    fn emit_event(&mut self, cycle: Cycle) {
        self.registered_screens
            .iter_mut()
            .for_each(|screen| {
                if screen.game_state() != self.state.unwrap() {
                    return;
                }

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
