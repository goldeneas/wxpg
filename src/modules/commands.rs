use crate::EngineInternal;

use super::{egui_renderer::EguiWindow, screen_server::GameState};

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
    
    pub fn set_state(&mut self, state: GameState) {
        self.new_state = Some(state);
    }

    pub fn register_egui_window(&mut self,
        window: impl EguiWindow + 'static,
        required_state: GameState
    ) {
        self.engine_internal.egui_renderer
            .register_window(window, required_state);
    }

    pub fn new_state(&self) -> Option<GameState> {
        self.new_state
    }
}
