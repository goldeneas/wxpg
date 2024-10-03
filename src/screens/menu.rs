use std::process::exit;

use egui::{Align2, Button};

use crate::{resources::screen_server::GameState, DrawContext, RendererContext, ServerContext};

use super::screen::Screen;

#[derive(Default)]
pub struct MenuScreen {}

impl Screen for MenuScreen {
    fn start(&mut self,
        draw_ctx: &mut DrawContext,
        renderer_ctx: &mut RendererContext,
        server_ctx: &mut ServerContext
    ) {
        let egui_renderer = &mut renderer_ctx.egui_renderer;
        egui_renderer.add_window(GameState::Menu, |ctx, screen_server| {
            egui::Window::new("Main Menu")
                .default_open(true)
                .default_size([200.0, 85.0])
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    if ui.add_sized([200.0, 30.0], Button::new("Play")).clicked() {
                        screen_server.set_state(GameState::Game);
                    }
    
                    if ui.add_sized([200.0, 30.0], Button::new("Quit")).clicked() {
                        exit(0);
                    }
    
                    ui.end_row();
                    ui.allocate_space(ui.available_size());
                });
        });
    }

    fn game_state(&self) -> GameState {
        GameState::Menu
    }
}
