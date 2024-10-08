use cgmath::{Matrix4, SquareMatrix};
use wgpu::util::DeviceExt;

use crate::{render::{camera::CameraUniform, pipeline_system::{AsVertexBufferLayout, Pipeline, ShaderUniform}, vertex::Vertex}, InstanceRaw};

#[derive(Debug)]
pub struct DefaultPipeline {
    internal_pipeline: Pipeline,
}

impl DefaultPipeline {
    pub fn new(device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shader.wgsl"));

        let camera_uniform = Self::create_camera_uniform(device);
        let texture_uniform = Self::create_texture_uniform(device);

        let mut internal_pipeline = Pipeline::new(shader);
        internal_pipeline.add_uniform(texture_uniform);
        internal_pipeline.add_uniform(camera_uniform);
        internal_pipeline.add_vertex_buffer_layout(Vertex::desc());
        internal_pipeline.add_vertex_buffer_layout(InstanceRaw::desc());
        internal_pipeline.build_pipeline(device, config);

        Self {
            internal_pipeline,
        }
    }

    pub fn update(&mut self,
        queue: &wgpu::Queue,
        camera_uniform: &CameraUniform
    ) {
        queue.write_buffer(self.internal_pipeline.buffer(1),
            0, bytemuck::cast_slice(camera_uniform));
    }

    fn create_camera_uniform(device: &wgpu::Device) -> ShaderUniform {
        let camera_uniform: CameraUniform = Matrix4::identity()
            .into();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&camera_uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
            ],
        });

        ShaderUniform {
            bind_group: Some(camera_bind_group),
            bind_group_layout: camera_bind_group_layout,
            buffer: Some(camera_buffer),
        }
    }

    fn create_texture_uniform(device: &wgpu::Device) -> ShaderUniform {
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ]
        });

        ShaderUniform {
            bind_group_layout: texture_bind_group_layout,
            bind_group: None,
            buffer: None,
        }
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

        let render_pipeline = &self.internal_pipeline;
        render_pass.set_pipeline(render_pipeline.render_pipeline());
        render_pass
    }

}
