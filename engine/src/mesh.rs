use std::collections::HashMap;

use glam::{Mat4, Quat, Vec2, Vec3};

use crate::transform::Transform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    /// Треугольник по трём вершинам (добавляет и вершины, и индексы).
    pub fn triangle(&mut self, a: Vertex, b: Vertex, c: Vertex) {
        let base = self.vertices.len() as u32;
        self.vertices.extend_from_slice(&[a, b, c]);
        self.indices.extend_from_slice(&[base, base + 1, base + 2]);
    }
}

/// Дескриптор меша в хранилище ресурсов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub u32);

/// Хранилище мешей. Загрузка возвращает MeshHandle, дубли не хранятся.
#[derive(Default)]
pub struct Assets {
    meshes: HashMap<MeshHandle, Mesh>,
    next_id: u32,
}

impl Assets {
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавляет меш и возвращает его дескриптор.
    pub fn load(&mut self, mesh: Mesh) -> MeshHandle {
        let handle = MeshHandle(self.next_id);
        self.next_id += 1;
        self.meshes.insert(handle, mesh);
        handle
    }

    pub fn get(&self, handle: MeshHandle) -> Option<&Mesh> {
        self.meshes.get(&handle)
    }

    pub fn get_mut(&mut self, handle: MeshHandle) -> Option<&mut Mesh> {
        self.meshes.get_mut(&handle)
    }

    pub fn remove(&mut self, handle: MeshHandle) -> Option<Mesh> {
        self.meshes.remove(&handle)
    }
}
