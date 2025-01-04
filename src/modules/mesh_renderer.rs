use super::{frame_context::FrameContext, render_storage::RenderStorage};

#[derive(Default)]
pub struct MeshRenderer {}

impl MeshRenderer {
    pub fn draw(&self,
        storage: &RenderStorage,
        frame_ctx: &FrameContext,
        device: &wgpu::Device,
    ) {
        let view = &frame_ctx.view;
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Mesh Render Encoder"),
        });
    }
}
