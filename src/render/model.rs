use std::sync::Arc;

use crate::{modules::asset_server::{Asset, AssetServer}, InstanceData, Texture};

use super::{model_mesh::ModelMesh, vertex::Vertex};

#[derive(Debug, Default)]
pub struct Model {
    pub name: String,
    pub meshes: Vec<ModelMesh>,
    pub textures: Vec<Arc<Texture>>,
}

impl Asset for Model {
    fn file_name(&self) -> &str {
        &self.name
    }
}

impl Model {
    pub fn load(file_name: &str,
        asset_server: &mut AssetServer,
        device: &wgpu::Device,
        queue: &wgpu::Queue
    ) -> Model {
        let (models, materials_opt) = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS)
            .expect("Could not load file OBJ file");

        let textures: Vec<Arc<Texture>> = match materials_opt {
            Ok(tobj_materials) => {
                tobj_materials
                    .into_iter()
                    .map(|m| {
                        let diffuse_texture_name = &m.diffuse_texture.unwrap();
                        asset_server
                            .get_or_load(diffuse_texture_name, device, queue)
                            .unwrap()
                    }).collect::<Vec<_>>()
            },
            Err(_) => {
                let diffuse_texture = Texture::debug(asset_server, device, queue);
                vec![diffuse_texture]
            }
        };

        let meshes = models.into_iter()
            .map(|m| {
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| {
                        let mut normals = [0.0, 0.0, 0.0];
                        if !m.mesh.normals.is_empty() { 
                            normals = [
                                m.mesh.normals[i * 2],
                                m.mesh.normals[i * 2 + 1],
                                m.mesh.normals[i * 2 + 2],
                            ];
                        }

                        Vertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                            normal: normals,
                        }
                    }).collect::<Vec<_>>();

                let indices = m.mesh.indices;

                let instance_data = InstanceData::from_position((0.0, 0.0, 0.0));
                let instances = vec![instance_data];

                // this material id is relative to the textures in the model.
                // it will be converted to render_server's ids when pushing
                // the model
                let material_id = m.mesh.material_id.unwrap_or(0);

                ModelMesh {
                    vertices,
                    indices,
                    instances,
                    material_id
                }
            }).collect::<Vec<_>>();

        Self {
            name: file_name.to_string(),
            textures,
            meshes,
        }
    }
}
