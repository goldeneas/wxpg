use crate::EngineInternal;

use super::screen_server::GameState;

pub struct Commands<'a> {
    pub new_state: Option<GameState>,
    pub engine_internal: &'a mut EngineInternal,
}

impl<'a> Commands<'a> {
    pub fn new(engine_internal: &'a mut EngineInternal) -> Self {
        let new_state = Option::default();
        
        Self {
            new_state,
            engine_internal,
        }
    }

    pub fn new_state(&self) -> Option<GameState> {
        self.new_state
    }
}
