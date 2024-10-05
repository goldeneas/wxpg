use crate::{app::App, modules::{commands::Commands, screen_server::{GameState, ScreenServer}}, screens::screen::Screen};

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

fn main() {
    let mut app = AppTest::default();
    wxpg::run(&mut app);
}
