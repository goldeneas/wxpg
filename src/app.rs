use crate::modules::screen_server::ScreenServer;

pub trait App {
    fn start(&mut self, screen_server: &mut ScreenServer);
}
