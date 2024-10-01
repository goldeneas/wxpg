use wxpg::{app::{App, AppConfig}, run};

#[derive(Default)]
struct AppExample {}

impl App for AppExample {
    fn config(&self) -> AppConfig {
        AppConfig {
            update_dt: 1.0/20.0,
            cursor_locked: false,
            cursor_visible: false,
        }
    }
}

fn main() {
    let app = AppExample::default();
    run(app);
}
