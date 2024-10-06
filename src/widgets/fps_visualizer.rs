use egui_plot::{Legend, Line, PlotPoints};

use crate::modules::egui_renderer::EguiWidget;

const FPS_COUNT: usize = 10;

#[derive(Default)]
pub struct FpsGraph {
    fps: [f32 ; FPS_COUNT],
}

impl FpsGraph {
    pub fn add_fps(&mut self, fps: f32) {
        self.move_oldest();
        self.fps[FPS_COUNT - 1] = fps;
    }

    // 1 2 3 4 5
    // 2 3 4 5 1
    fn move_oldest(&mut self) {
        self.fps.rotate_right(FPS_COUNT - 1);
    }
}

impl EguiWidget for FpsGraph {
    fn show(&mut self, ui: &mut egui::Ui) {
        egui_plot::Plot::new("plot")
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                let sine_points = PlotPoints::from_ys_f32(&self.fps);
                plot_ui.line(Line::new(sine_points));
            });
    }
}
