use log::debug;

use crate::Texture;

pub type UniformId = usize;
pub type LayoutId = usize;

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
pub struct Pipeline {
    free_vertex_layout_id: LayoutId,
    free_uniform_id: UniformId,
    shader: wgpu::ShaderModule,
    uniforms: Vec<ShaderUniform>,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
    render_pipeline: Option<wgpu::RenderPipeline>,
}

impl Pipeline {
    pub fn new(shader: wgpu::ShaderModule) -> Self {
        let uniforms = Vec::default();
        let vertex_buffer_layouts = Vec::default();
        let render_pipeline = None;
        let free_vertex_layout_id = LayoutId::default();
        let free_uniform_id = UniformId::default();

        Self {
            free_uniform_id,
            free_vertex_layout_id,
            vertex_buffer_layouts,
            uniforms,
            shader,
            render_pipeline,
        }
    }

    pub fn add_uniform(&mut self, uniform: ShaderUniform,) -> UniformId {
        let uniform_id = self.free_uniform_id;

        self.uniforms.push(uniform);
        self.free_uniform_id += 1;

        uniform_id
    }

    pub fn add_vertex_buffer_layout(&mut self,
        layout: wgpu::VertexBufferLayout<'static>
    ) -> LayoutId {
        let layout_id = self.free_vertex_layout_id;

        self.vertex_buffer_layouts.push(layout);
        self.free_vertex_layout_id += 1;

        layout_id
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

    // todo too many render pipeline calls! change their names
    pub fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        self.render_pipeline.as_ref().unwrap()
    }

    pub fn buffer(&self, idx: UniformId) -> &wgpu::Buffer {
        self.uniforms.get(idx)
            .expect("Could not find uniform with specified index")
            .buffer
            .as_ref()
            .expect("Found a shader uniform, but it doesnt have a buffer set!")
    }
}
