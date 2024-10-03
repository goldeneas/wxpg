use crate::EngineInternal;

use super::screen_server::GameState;

type Command = dyn Fn(&mut EngineInternal);

#[derive(Default)]
pub struct Commands {
    new_state: Option<GameState>,
    commands: Vec<Box<Command>>,
}

impl Commands {
    pub fn add(&mut self, func: impl Fn(&mut EngineInternal) + 'static) {
        let func = Box::new(func);
        self.commands.push(func);
    }

    pub fn funcs(&self) -> &Vec<Box<Command>> {
        &self.commands
    }

    pub fn clear_funcs(&mut self) {
        self.commands.clear();
    }

    pub fn new_state(&self) -> Option<GameState> {
        self.new_state
    }
}
