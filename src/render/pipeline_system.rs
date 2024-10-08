use log::debug;

use crate::Texture;

pub type UniformIdx = usize;
pub type LayoutIdx = usize;

pub trait AsVertexBufferLayout {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[derive(Debug)]
pub struct ShaderUniform {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: Option<wgpu::BindGroup>,
    pub buffer: Option<wgpu::Buffer>,
}

#[derive(Debug)]
pub struct PipelineSystem {
    shader: wgpu::ShaderModule,
    uniforms: Vec<ShaderUniform>,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
    render_pipeline: Option<wgpu::RenderPipeline>,
}

impl PipelineSystem {
    pub fn new(shader: wgpu::ShaderModule) -> Self {
        let uniforms = Vec::default();
        let vertex_buffer_layouts = Vec::default();
        let render_pipeline = None;

        Self {
            vertex_buffer_layouts,
            uniforms,
            shader,
            render_pipeline,
        }
    }

    pub fn add_uniform(&mut self, uniform: ShaderUniform,) -> UniformIdx {
        let uniform_idx = self.uniforms.len();
        self.uniforms.push(uniform);

        uniform_idx
    }

    pub fn add_vertex_buffer_layout(&mut self,
        layout: wgpu::VertexBufferLayout<'static>
    ) -> LayoutIdx {
        let layout_idx = self.vertex_buffer_layouts.len();
        self.vertex_buffer_layouts.push(layout);

        layout_idx
    }

    pub fn build_pipeline(&mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration
    ) {
        if self.render_pipeline.is_some() {
            debug!("Rebuilding pipeline point...");
        }

        let bind_group_layouts = self.uniforms
            .iter()
            .map(|uniform| &uniform.bind_group_layout)
            .collect::<Vec<_>>();

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.shader,
                entry_point: "vs_main",
                buffers: self.vertex_buffer_layouts.as_slice(),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive : wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_TEXTURE_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
            cache: None,
        });

        self.render_pipeline = Some(render_pipeline)
    }

    pub fn pass<'a>(&self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
label: Some("Render Pass"),
            // this is what @location(0) in the fragment shader targets
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),

                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        let render_pipeline = &self.render_pipeline.as_ref().unwrap();
        render_pass.set_pipeline(render_pipeline);
        render_pass
    }

    pub fn buffer(&self, idx: UniformIdx) -> &wgpu::Buffer {
        self.uniforms.get(idx)
            .expect("Could not find uniform with specified index")
            .buffer
            .as_ref()
            .expect("Found a shader uniform, but it doesnt have a buffer set!")
    }
}
