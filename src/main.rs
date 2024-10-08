use egui::Align2;

use winit::keyboard::KeyCode;
use wxpg::{app::App, modules::{commands::Commands, default_pipeline::{self, DefaultPipeline}, egui_renderer::{EguiWidget, EguiWindow}, input_server, render_storage::RenderStorage, screen_server::{GameState, ScreenServer}}, primitives::cube::Cube, render::{camera::FpsCamera, pipeline_system::Pipeline, texture::Texture}, run, screens::screen::Screen, widgets::fps_visualizer::FpsGraph};

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
                self.fps_graph.show("fps-counter", ui);
            });
    }
}

#[derive(Default)]
pub struct TestScreen {
    render_storage: RenderStorage,
    camera: Option<FpsCamera>,
    default_pipeline: Option<DefaultPipeline>,
}

impl Screen for TestScreen {
    fn start(&mut self, commands: &mut Commands) {
        let device = &commands.engine_internal.device;
        let config = &commands.engine_internal.config;
        let queue = &commands.engine_internal.queue;
        let input_server = &mut commands.engine_internal.input_server;
        let asset_server = &mut commands.engine_internal.asset_server;

        let texture = Texture::debug(asset_server, device, queue);
        self.render_storage.push_material(texture, device);

        let cube = Cube::default();
        self.render_storage.push_mesh(&cube, device);

        input_server.register_action("camera_up", KeyCode::Space);
        input_server.register_action("camera_right", KeyCode::ShiftLeft);
        input_server.register_action("camera_down", KeyCode::ArrowDown);
        input_server.register_action("camera_left", KeyCode::ArrowLeft);
        input_server.register_action("camera_front", KeyCode::ArrowUp);
        input_server.register_action("camera_back", KeyCode::ArrowDown);

        let default_pipeline = DefaultPipeline::new(device, config);
        let camera = FpsCamera::new(config.width as f32,
            config.height as f32,
            1.0
        );

        self.camera = Some(camera);
        self.default_pipeline = Some(default_pipeline);
    }

    fn update(&mut self, commands: &mut Commands) {
        let queue = &commands.engine_internal.queue;
        let input_server = &commands.engine_internal.input_server;
        let camera = self.camera.as_mut().unwrap();
        let pipeline = self.default_pipeline.as_mut().unwrap();

        camera.update(input_server);
        pipeline.update(queue, &camera.transform().uniform());
    }

    // TODO can this process be automated?
    fn draw(&mut self, commands: &mut Commands) {
        let device = &commands.engine_internal.device;
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Object Encoder"),
        });
        let view = &commands.engine_internal.

        let pass = self.default_pipeline.unwrap()
            .pass(encoder, view, depth_texture_view)
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
