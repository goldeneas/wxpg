use crate::DrawContext;

pub struct FrameContext {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoders: Vec<wgpu::CommandEncoder>,
}

impl FrameContext {
    pub fn new(draw_ctx: &DrawContext) -> Self {
        let output = draw_ctx.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let encoders = Vec::new();

        Self {
            output,
            view,
            encoders,
        }
    }

    pub fn add_encoder(&mut self, encoder: wgpu::CommandEncoder) {
        self.encoders.push(encoder);
    }
}
