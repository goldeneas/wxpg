use crate::DrawContext;

pub struct FrameContext {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoders: Vec<wgpu::CommandEncoder>,
}

impl FrameContext {
    pub fn new(draw_ctx: &DrawContext,
        vec_capacity: Option<usize>
    ) -> Self {
        let output = draw_ctx.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let capacity = vec_capacity.unwrap_or(3);
        let encoders = Vec::with_capacity(capacity);

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
