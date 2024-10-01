use crate::{resources::{asset_server::{Asset, AssetServer}, render_server::{MaterialId, RenderServer}}, InstanceData, Texture};

use super::{model_mesh::ModelMesh, vertex::Vertex};

#[derive(Debug, Default)]
pub struct Model {
    pub name: String,
    pub meshes: Vec<ModelMesh>,
    pub textures: Vec<Texture>,
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

        let material_ids: Vec<MaterialId> = match materials_opt {
            Ok(tobj_materials) => {
                tobj_materials
                    .into_iter()
                    .map(|m| {
                        let diffuse_texture_name = &m.diffuse_texture.unwrap();
                        let diffuse_texture = asset_server
                            .get_or_load(diffuse_texture_name, device, queue)
                            .unwrap();

                        render_server.push_material(diffuse_texture, device)
                    }).collect::<Vec<_>>()
            },
            Err(_) => {
                let diffuse_texture = Texture::debug(asset_server, device, queue);
                let material_id = render_server
                    .push_material(diffuse_texture, device);

                vec![material_id]
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

                // we need to convert the meshes' material idxs to our own
                // material ids
                let material_id = material_ids[m.mesh.material_id.unwrap_or(0)];

                ModelMesh {
                    vertices,
                    indices,
                    instances,
                    material_id
                }
            }).collect::<Vec<_>>();

        Self {
            name: file_name.to_string(),
            meshes,
        }
    }
}
