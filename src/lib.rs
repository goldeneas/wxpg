pub mod render;
pub mod util;
pub mod components;
pub mod resources;
pub mod screens;
pub mod pass_ext;
pub mod device_ext;
pub mod app;

pub use bevy_ecs;
pub use egui;
pub use egui_wgpu;
pub use egui_winit;
use resources::screen_server::GameState;
pub use wgpu;

use std::sync::Arc;
use std::time::Instant;

use app::App;
use bevy_ecs::world::Mut;
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
use resources::render_context::RenderContext;
use resources::input::InputStorage;
use wgpu::Features;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::window::WindowAttributes;
use winit::window::WindowId;
use winit::{
    event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};

pub struct EngineContext {
    delta_time: Instant,
    time_accumulator: f32,
    update_dt: f32,

    input_storage: InputStorage,
    asset_server: AssetServer,
    render_storage: RenderStorage,
    screen_server: ScreenServer,

    glyphon_renderer: GlyphonRenderer,
    egui_renderer: EguiRenderer,
    default_pipeline: DefaultPipeline,
    render_context: RenderContext,
    world: World,
}

impl EngineContext {
    pub async fn new(window: &Arc<Window>) -> Self {
        let size = window.inner_size();

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
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let depth_texture = Texture::depth_texture(&device, &config);
        let world = World::default();

        let input_storage = InputStorage::default();
        let asset_server = AssetServer::default();
        let render_storage = RenderStorage::default();
        let screen_server = ScreenServer::default();

        let glyphon_renderer = GlyphonRenderer::new(&device, &queue);
        let egui_renderer = EguiRenderer::new(&device, window);
        let default_pipeline = DefaultPipeline::new(&device, &config);
        let render_context = RenderContext {
            window: window.clone(),
            config,
            size,
            device,
            queue,
            surface,
            depth_texture,
        };

        let update_dt = 1.0/20.0;
        let delta_time = Instant::now();
        let time_accumulator = f32::default();

        Self {
            delta_time,
            time_accumulator,
            input_storage,
            asset_server,
            render_storage,
            screen_server,
            update_dt,
            world,
            glyphon_renderer,
            egui_renderer,
            default_pipeline,
            render_context,
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let ctx = &mut self.render_context;

            ctx.size = new_size;
            ctx.config.width = new_size.width;
            ctx.config.height = new_size.height;
            ctx.depth_texture = Texture::depth_texture(&ctx.device, &ctx.config);
            ctx.surface.configure(&ctx.device, &ctx.config);
        }
    }

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
        self.screen_server.update();
    }

    fn draw(&mut self) {
        let render_ctx = &self.render_context;
        let mut frame_ctx = FrameContext::new(render_ctx, None);

        self.screen_server.draw();

        let state = self.screen_server.state_mut();
        self.egui_renderer.draw(render_ctx, &mut frame_ctx, state);

        self.glyphon_renderer.draw(render_ctx, &mut frame_ctx);

        let buffers = frame_ctx
            .encoders
            .into_iter()
            .map(|encoder| {
                encoder.finish()
            })
            .collect::<Vec<_>>();

        render_ctx.queue.submit(buffers);
        frame_ctx.output.present();
    }
}

pub struct Engine {
    window: Option<Arc<Window>>,
    engine_context: Option<EngineContext>,
    app: Box<dyn App>,
}

impl Engine {
    fn new(app: impl App + 'static) -> Self{
        let window = Option::default();
        let app = Box::new(app);
        let engine_context = Option::default();

        Self {
            window,
            app,
            engine_context,
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
                let engine_ctx = self.engine_context.as_mut().unwrap();
                engine_ctx.resize(physical_size);
            },
            WindowEvent::RedrawRequested => {
                let engine_ctx = self.engine_context.as_mut().unwrap();
                engine_ctx.redraw_requested();
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: key_state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => {
                let engine_ctx = self.engine_context.as_mut().unwrap();
                self.app.input(engine_ctx, &keycode, &key_state);
            },
            _ => {}
        }

        let window = self.window.as_mut().unwrap();
        let engine_ctx = self.engine_context.as_mut().unwrap();
        engine_ctx.egui_renderer
            .window_event(&window, &event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            let resources = self.engine_context.as_mut().unwrap();
            self.app.mouse_moved(resources, delta);
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(WindowAttributes::default()).unwrap();
        let window = Arc::new(window);
        let mut engine_ctx = pollster::block_on(EngineContext::new(&window));

        self.app.start(&mut engine_ctx);

        self.window = Some(window);
        self.engine_context = Some(engine_ctx);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.window.as_ref().unwrap();
        window.request_redraw();
    }
}

pub fn run(app: impl App + 'static) {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut engine = Engine::new(app);
    let _ = event_loop.run_app(&mut engine);
}
