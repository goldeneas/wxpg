use cgmath::{Matrix4, SquareMatrix};
use wgpu::util::DeviceExt;

use crate::{render::{camera::CameraUniform, pipeline_system::{AsVertexBufferLayout, PipelineSystem, ShaderUniform}, vertex::Vertex}, InstanceRaw};

#[derive(Debug)]
pub struct DefaultPipeline {
    pipeline_point: PipelineSystem,
}

impl DefaultPipeline {
    pub fn new(device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shader.wgsl"));

        let camera_uniform = Self::create_camera_uniform(device);
        let texture_uniform = Self::create_texture_uniform(device);

        let mut pipeline_point = PipelineSystem::new(shader);
        pipeline_point.add_uniform(texture_uniform);
        pipeline_point.add_uniform(camera_uniform);
        pipeline_point.add_vertex_buffer_layout(Vertex::desc());
        pipeline_point.add_vertex_buffer_layout(InstanceRaw::desc());
        pipeline_point.build_pipeline(device, config);

        Self {
            pipeline_point,
        }
    }

    pub fn update(&mut self,
        queue: &wgpu::Queue,
        camera_uniform: &CameraUniform
    ) {
        queue.write_buffer(self.pipeline_point.buffer(1),
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
}
