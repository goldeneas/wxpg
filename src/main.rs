use std::process::exit;

use egui::{Align2, Button};

use egui_plot::{Legend, Line, PlotPoints};
use wxpg::{app::App, modules::{commands::Commands, egui_renderer::{EguiWidget, EguiWindow}, screen_server::{GameState, ScreenServer}}, run, screens::screen::Screen, widgets::fps_visualizer::FpsGraph};

#[derive(Default)]
pub struct TestWindow {
    fps_graph: FpsGraph,
    counter: u32,
}

impl EguiWindow for TestWindow {
    fn ui(&mut self, ctx: &egui::Context) {
        self.counter += 1;
        if self.counter > 100 {
            self.counter = 0;
            self.fps_graph.add_fps(rand::random::<f32>() * 22.35);
        }

        egui::Window::new("Main Menu")
            .default_open(true)
            .default_size([960.0, 540.0])
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                self.fps_graph.show(ui);
            });
    }
}

#[derive(Default)]
pub struct TestScreen {}
impl Screen for TestScreen {
    fn start(&mut self, commands: &mut Commands) {
        println!("HI2");
        let window = TestWindow::default();
        commands.register_egui_window(window, GameState::Menu);
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

fn main() {
    let mut app = AppTest::default();
    run(&mut app);
}
