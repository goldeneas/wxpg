pub mod render;
pub mod util;
pub mod components;
pub mod resources;
pub mod screens;
pub mod pass_ext;
pub mod device_ext;
pub mod app;

use app::App;
pub use bevy_ecs;
pub use egui;
pub use egui_wgpu;
pub use egui_winit;
use resources::input_server::InputServer;
use resources::screen_server::GameState;
use screens::screen::Screen;
pub use wgpu;
use winit::dpi::PhysicalSize;

use std::sync::Arc;
use std::time::Instant;

use bevy_ecs::world::World;
use resources::asset_server::AssetServer;
use resources::egui_renderer::EguiRenderer;
use resources::glyphon_renderer::GlyphonRenderer;
use resources::render_server::RenderStorage;
use resources::screen_server::ScreenServer;
use render::texture::*;
use render::instance_data::*;

use resources::default_pipeline::DefaultPipeline;
use resources::frame_context::FrameContext;
use wgpu::Features;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::window::WindowAttributes;
use winit::window::WindowId;
use winit::{
    event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};

pub struct EngineInternal {
    pub asset_server: AssetServer,
    pub input_server: InputServer,

    pub glyphon_renderer: GlyphonRenderer,
    pub egui_renderer: EguiRenderer,

    pub world: World,

    pub window: Arc<Window>,
    pub depth_texture: Arc<Texture>,
    pub device: wgpu::Device,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub queue: wgpu::Queue,
    pub window_size: PhysicalSize<u32>,
    pub render_storage: RenderStorage,
    pub default_pipeline: DefaultPipeline,
}

impl EngineInternal {
    pub async fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop.create_window(WindowAttributes::default())
            .unwrap();
        let window = Arc::new(window);
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())
            .unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: Features::POLYGON_MODE_LINE | Features::MULTI_DRAW_INDIRECT | Features::INDIRECT_FIRST_INSTANCE,
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
            label: None,
        }, None).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let depth_texture = Texture::depth_texture(&device, &config);

        let glyphon_renderer = GlyphonRenderer::new(&device, &queue);
        let egui_renderer = EguiRenderer::new(&device, &window);
        let default_pipeline = DefaultPipeline::new(&device, &config);
        let render_storage = RenderStorage::default();

        let world = World::default();

        let asset_server = AssetServer::default();
        let input_server = InputServer::default();

        Self {
            window,
            queue,
            device,
            config,
            surface,
            window_size,
            egui_renderer,
            depth_texture,
            render_storage,
            glyphon_renderer,
            default_pipeline,
            asset_server,
            input_server,
            world,
        }
    }
}

pub struct Engine {
    delta_time: Instant,
    time_accumulator: f32,
    update_dt: f32,

    engine_internal: Option<EngineInternal>,
    screen_server: ScreenServer,
}

impl Engine {
    fn new(app: &mut impl App) -> Self{
        let engine_internal = Option::default();
        let mut screen_server = ScreenServer::default();
        app.start(&mut screen_server);

        let delta_time = Instant::now();
        let time_accumulator = f32::default();
        let update_dt = 1.0/20.0;

        Self {
            delta_time,
            time_accumulator,
            update_dt,
            engine_internal,
            screen_server,
        }
    }
}

impl ApplicationHandler for Engine {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent
    ) {
        let engine_internal = self.engine_internal.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size);
            },
            WindowEvent::RedrawRequested => {
                self.redraw_requested();
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => {
                engine_internal.input_server.keyboard_input(keycode, state);
            },
            _ => {}
        }

        let engine_internal = self.engine_internal.as_mut().unwrap();
        engine_internal.egui_renderer
            .window_event(&engine_internal.window, &event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            let engine_internal = self.engine_internal.as_mut().unwrap();
            engine_internal.input_server
                .mouse_delta(delta);
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let engine_internal = pollster::block_on(EngineInternal::new(event_loop));
        self.engine_internal = Some(engine_internal);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let engine_internal = self.engine_internal.as_ref().unwrap();
        engine_internal.window.request_redraw();
    }
}

impl Engine {
    fn redraw_requested(&mut self) {
        self.time_accumulator += self.delta_time
            .elapsed()
            .as_secs_f32();
        self.delta_time = Instant::now();

        while self.time_accumulator >= self.update_dt {
            self.update();
            self.time_accumulator -= self.update_dt;
        }

        self.draw();
    }

    fn update(&mut self) {
        let engine_internal = self.engine_internal.as_mut().unwrap();
        self.screen_server.update(engine_internal);
    }

    fn draw(&mut self) {
        let screen_server = &mut self.screen_server;
        let engine_internal = self.engine_internal.as_mut().unwrap();
        screen_server.draw(engine_internal);

        let device = &engine_internal.device;
        let config = &engine_internal.config;
        let queue = &engine_internal.queue;
        let window = &engine_internal.window;
        let surface = &engine_internal.surface;

        let mut frame_ctx = FrameContext::new(surface);

        engine_internal.egui_renderer
            .draw(device, queue, config, window, screen_server, &mut frame_ctx);

        engine_internal.glyphon_renderer
            .draw(device, queue, config, &mut frame_ctx);

        let buffers = frame_ctx
            .encoders
            .into_iter()
            .map(|encoder| {
                encoder.finish()
            })
            .collect::<Vec<_>>();

        engine_internal.queue.submit(buffers);
        frame_ctx.output.present();
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        let engine_internal = self.engine_internal.as_mut().unwrap();
        if new_size.width > 0 && new_size.height > 0 {
            engine_internal.window_size = new_size;
            engine_internal.config.width = new_size.width;
            engine_internal.config.height = new_size.height;

            let device = &engine_internal.device;
            let config = &engine_internal.config;
            engine_internal.depth_texture = Texture::depth_texture(device, config);
            engine_internal.surface.configure(device, config);
        }
    }
}

pub fn run(app: &mut impl App) {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut engine = Engine::new(app);
    let _ = event_loop.run_app(&mut engine);
}
