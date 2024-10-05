use std::process::exit;

use egui::{Align2, Button};

use crate::{app::App, modules::{commands::Commands, egui_renderer::EguiWindow, screen_server::{GameState, ScreenServer}}, run, screens::screen::Screen};

#[derive(Default)]
pub struct TestWindow {}
impl EguiWindow for TestWindow {
    fn ui(&mut self, ctx: &egui::Context) {
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
    }
}

#[derive(Default)]
pub struct TestScreen {}
impl Screen for TestScreen {
    fn start(&mut self, commands: &mut Commands) {
        println!("HI2");
    }

    fn update(&mut self, commands: &mut Commands) {
    
    }
}

#[derive(Default)]
pub struct AppTest {}
impl App for AppTest {
    fn start(&mut self, screen_server: &mut ScreenServer) {
        let test = TestScreen::default();
        screen_server.register_screen(test, GameState::Menu);
    }
}

#[test]
fn main() {
    let mut app = AppTest::default();
    run(&mut app);
}
