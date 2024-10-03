pub mod render;
pub mod util;
pub mod components;
pub mod resources;
pub mod screens;
pub mod pass_ext;
pub mod device_ext;

pub use bevy_ecs;
pub use egui;
pub use egui_wgpu;
pub use egui_winit;
use resources::asset_server;
use resources::input_server;
use resources::input_server::InputServer;
use resources::screen_server;
use resources::screen_server::GameState;
use screens::screen::Screen;
pub use wgpu;

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
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::window::WindowAttributes;
use winit::window::WindowId;
use winit::{
    event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};

pub struct DrawContext {
    pub window: Arc<Window>,
    pub depth_texture: Arc<Texture>,
    pub device: wgpu::Device,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub queue: wgpu::Queue,
    pub window_size: winit::dpi::PhysicalSize<u32>,
    pub glyphon_renderer: GlyphonRenderer,
    pub egui_renderer: EguiRenderer,
    pub render_storage: RenderStorage,
    pub default_pipeline: DefaultPipeline,
}

impl DrawContext {
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

        DrawContext {
            glyphon_renderer,
            egui_renderer,
            default_pipeline,
            render_storage,
            window,
            config,
            window_size,
            device,
            queue,
            surface,
            depth_texture,
        }
    }
}

pub struct ScreenContext {
    pub state: GameState,
    pub draw_ctx: DrawContext,
    pub asset_server: AssetServer,
    pub input_server: InputServer,
}

impl ScreenContext {
    pub fn new(draw_ctx: DrawContext) -> Self {
        let state = GameState::default();
        let asset_server = AssetServer::default();
        let input_server = InputServer::default();

        Self {
            draw_ctx,
            state,
            asset_server,
            input_server,
        }
    }
}

pub struct EngineInternal {
    delta_time: Instant,
    time_accumulator: f32,
    update_dt: f32,

    screen_server: ScreenServer,
    world: World,
}

impl EngineInternal {
    pub fn new() -> Self {
        let world = World::default();

        let update_dt = 1.0/20.0;
        let delta_time = Instant::now();
        let time_accumulator = f32::default();
        let screen_server = ScreenServer::default();

        Self {
            delta_time,
            time_accumulator,
            update_dt,
            world,
            screen_server,
        }
    }

    fn resize(&mut self, draw_ctx: &mut DrawContext, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            draw_ctx.window_size = new_size;
            draw_ctx.config.width = new_size.width;
            draw_ctx.config.height = new_size.height;

            let device = &draw_ctx.device;
            let config = &draw_ctx.config;
            draw_ctx.depth_texture = Texture::depth_texture(device, config);
            draw_ctx.surface.configure(device, config);
        }
    }

    fn redraw_requested(&mut self, screen_ctx: &mut ScreenContext) {
        self.time_accumulator += self.delta_time
            .elapsed()
            .as_secs_f32();
        self.delta_time = Instant::now();

        while self.time_accumulator >= self.update_dt {
            self.update(screen_ctx);
            self.time_accumulator -= self.update_dt;
        }

        self.draw();
    }

    fn update(&mut self, screen_ctx: &mut ScreenContext) {
        self.screen_server.update(screen_ctx);
    }

    fn draw(&mut self, screen_ctx: &mut ScreenContext) {
        let mut frame_ctx = FrameContext::new(&screen_ctx.draw_ctx, None);
        self.screen_server.draw(screen_ctx);

        screen_ctx.renderer_ctx.egui_renderer.draw(screen_ctx, &mut frame_ctx);
        screen_ctx.renderer_ctx.glyphon_renderer.draw(&screen_ctx.draw_ctx, &mut frame_ctx);

        let buffers = frame_ctx
            .encoders
            .into_iter()
            .map(|encoder| {
                encoder.finish()
            })
            .collect::<Vec<_>>();

        screen_ctx.draw_ctx.queue.submit(buffers);
        frame_ctx.output.present();
    }
}


pub struct Engine {
    screen_ctx: Option<ScreenContext>,
    engine_internal: Option<EngineInternal>,
    screen: Box<dyn Screen>,
}

impl Engine {
    fn new(screen: impl Screen + 'static) -> Self{
        let screen = Box::new(screen);
        let engine_internal = Option::default();
        let screen_ctx = Option::default();

        Self {
            screen,
            engine_internal,
            screen_ctx,
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
                let engine_internal = self.engine_state.as_mut().unwrap();
                let draw_ctx = self.draw_ctx.as_mut().unwrap();
                engine_internal.resize(draw_ctx, physical_size);
            },
            WindowEvent::RedrawRequested => {
                let engine_internal = self.engine_state.as_mut().unwrap();
                let draw_ctx = self.draw_ctx.as_ref().unwrap();
                let renderer_ctx = self.renderer_ctx.as_mut().unwrap();
                let server_ctx = self.server_ctx.as_mut().unwrap();
                engine_internal.redraw_requested(server_ctx, renderer_ctx, draw_ctx);
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => {
                let server_ctx = self.server_ctx.as_mut().unwrap();
                server_ctx.input_server
                    .keyboard_input(keycode, state);
            },
            _ => {}
        }

        let draw_ctx = self.draw_ctx.as_ref().unwrap();
        let renderer_ctx = self.renderer_ctx.as_mut().unwrap();
        renderer_ctx.egui_renderer
            .window_event(&draw_ctx.window, &event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            let server_ctx = self.server_ctx.as_mut().unwrap();
            server_ctx.input_server
                .mouse_delta(delta);
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let engine_internal = EngineInternal::new();
        let draw_ctx = pollster::block_on(DrawContext::new(event_loop));
        let screen_ctx = ScreenContext::new(draw_ctx);

        self.screen_ctx = Some(screen_ctx);
        self.engine_internal = Some(engine_internal);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let draw_ctx = self.draw_ctx.as_ref().unwrap();
        draw_ctx.window.request_redraw();
    }
}

pub fn run(screen: impl Screen + 'static) {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut engine = Engine::new(screen);
    let _ = event_loop.run_app(&mut engine);
}
