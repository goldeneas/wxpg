use std::process::exit;

use egui::{Align2, Button};

use crate::{modules::{commands::Commands, screen_server::GameState}, EngineInternal};

use super::screen::Screen;

#[derive(Default)]
pub struct MenuScreen {}

impl Screen for MenuScreen {
    fn start(&mut self, commands: &mut Commands) {
        spawn_egui(&mut commands.engine_internal);
    }
}

fn spawn_egui(engine_internal: &mut EngineInternal) {
    let egui_renderer = &mut engine_internal.egui_renderer;
    egui_renderer.add_window(GameState::Menu, |ctx| {
        egui::Window::new("Main Menu")
            .default_open(true)
            .default_size([200.0, 85.0])
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                if ui.add_sized([200.0, 30.0], Button::new("Play")).clicked() {
                    //commands.sta;
                }
    
                if ui.add_sized([200.0, 30.0], Button::new("Quit")).clicked() {
                    exit(0);
                }
    
                ui.end_row();
                ui.allocate_space(ui.available_size());
            });
    });
}

