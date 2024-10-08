use std::collections::HashMap;

use egui::{Context, Ui};
use egui_wgpu::ScreenDescriptor;
use egui_winit::winit::event::WindowEvent;
use wgpu::CommandEncoderDescriptor;
use winit::window::Window;

use crate::modules::frame_context::FrameContext;

use super::screen_server::{GameState, ScreenServer};

pub trait EguiWidget {
    fn show(&mut self, unique_name: &str, ui: &mut Ui);
}

pub trait EguiWindow {
    fn ui(&mut self, ctx: &Context);
}

pub struct EguiRenderer {
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
    window_map: HashMap<GameState, Box<dyn EguiWindow>>,
}

impl EguiRenderer {
    pub fn new(device: &wgpu::Device, window: &Window) -> Self {
        let context = egui::Context::default();
        let viewport_id = context.viewport_id();

        let state = egui_winit::State::new(context,
            viewport_id,
            window,
            None,
            None,
            None
        );

        let renderer = egui_wgpu::Renderer::new(device,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            None,
            1,
            false
        );

        let window_map = HashMap::new();

        Self {
            state,
            renderer,
            window_map,
        }
    }

    pub fn window_event(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    // TODO return a en EguiWindowId to let user manage visibility of window
    pub fn register_window(&mut self,
        window: impl EguiWindow + 'static,
        required_state: GameState,
    ) {
        let func = Box::new(window);
        self.window_map.insert(required_state, func);
    }

    pub fn draw(&mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        window: &Window,
        screen_server: &mut ScreenServer,
        frame_ctx: &mut FrameContext,
    ) {
        let view = &frame_ctx.view;
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Egui Encoder"),
        });

        let input = self.state.take_egui_input(window);
        let context = self.state.egui_ctx();

        let game_state = screen_server.state();

        context.begin_pass(input);
        self.window_map
            .iter_mut()
            .for_each(|(required_state, window)| {
                if game_state != *required_state {
                    return;
                } 

                window.ui(context);
            });
        let output = context.end_pass();

        self.state.handle_platform_output(window, output.platform_output);

        let tris = self.state
            .egui_ctx()
            .tessellate(output.shapes,
                output.pixels_per_point
            );

        output.textures_delta.set
            .into_iter()
            .for_each(|(id, image_delta)| {
                self.renderer
                    .update_texture(device, queue, id, &image_delta);
            });

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [config.width, config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        self.renderer
            .update_buffers(device, queue, &mut encoder, &tris, &screen_descriptor);

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            label: Some("Egui Pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.renderer.render(&mut render_pass.forget_lifetime(),
            &tris,
            &screen_descriptor
        );

        output.textures_delta.free
            .iter()
            .for_each(|id| {
                self.renderer
                    .free_texture(id);
            });

        frame_ctx.add_encoder(encoder);
    }
}
