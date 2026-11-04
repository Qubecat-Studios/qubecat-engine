use glam::Mat4;

use crate::{mesh::MeshHandle, transform::Transform};

#[derive(Clone, Debug)]
pub struct Object {
    pub name: String,
    pub transform: Transform,
    pub mesh: Option<MeshHandle>,
    pub parent: Option<u32>,
    pub children: Vec<u32>,
    pub visible: bool,
}

impl Object {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            transform: Transform::default(),
            mesh: None,
            parent: None,
            children: Vec::new(),
            visible: true,
        }
    }

    pub fn with_mesh(mut self, handle: MeshHandle) -> Self {
        self.mesh = Some(handle);
        self
    }

    pub fn world_matrix(&self, parents: Option<&Object>) -> Mat4 {
        match parents {
            Some(p) => p.transform.matrix() * self.transform.matrix(),
            None => self.transform.matrix(),
        }
    }
}
