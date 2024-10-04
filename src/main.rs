use winit::keyboard::KeyCode;
use wxpg::{resources::commands::Commands, run, screens::screen::Screen};

#[derive(Default)]
pub struct TestScreen {}
impl Screen for TestScreen {
    fn start(&mut self, commands: &mut Commands) {
        println!("HI");
    }

    fn update(&mut self, commands: &mut Commands) {
    
    }
}

fn main() {
    let screen = TestScreen::default();
    wxpg::run(screen);
}
