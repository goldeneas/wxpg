use wxpg::{resources::{commands::{self, Commands}, screen_server}, run, screens::screen::Screen};

#[derive(Default)]
pub struct TestScreen {}
impl Screen for TestScreen {
    fn start(&mut self, commands: &mut Commands) {
        println!("HI");
    }
}

fn main() {
    let screen = TestScreen::default();
    wxpg::run(screen);
}
