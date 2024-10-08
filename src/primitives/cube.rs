use crate::{modules::render_storage::MaterialId, render::{mesh::AsMesh, vertex::{Index, Vertex}}, InstanceData};

#[derive(Default)]
pub struct Cube {
    instances: Vec<InstanceData>,
}

const VERTICES: [Vertex ; 24] = [
    // UP
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    // DOWN
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    // RIGHT
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.0, -1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, -1.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    // LEFT
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.0, 1.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    // FRONT
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    // BACK
    Vertex {
        position: [1.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
];

const INDICES: [Index ; 36] = [
    // UP
    0, 1, 2, 0, 2, 3,
    // DOWN
    1, 0, 3, 1, 3, 2,
    // RIGHT
    3, 2, 1, 3, 1, 0,
    // LEFT
    0, 1, 2, 0, 2, 3,
    // FRONT
    0, 3, 1, 1, 3, 2,
    // BACK
    0, 1, 2, 0, 2, 3,
];

impl AsMesh for Cube {
    fn vertices(&self) -> &[Vertex] {
        &VERTICES
    }

    fn indices(&self) -> &[Index] {
        &INDICES
    }

    fn instances(&self) -> &[InstanceData] {
        self.instances.as_slice()
    }

    fn material_id(&self) -> MaterialId {
        0
    }
}

impl Cube {
    pub fn add_instance(&mut self, instance: InstanceData) {
        self.instances.push(instance);
    }
}
