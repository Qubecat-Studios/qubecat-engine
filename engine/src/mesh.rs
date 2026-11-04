use std::cmp::{max, min};

use glam::{Mat4, Quat, Vec2, Vec3};

use crate::{aabb::Aabb, transform::Transform};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub u32);

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}

impl Mesh {
    
}

pub struct Object {
    pub name: String,
    pub transform: Transform,
    pub mesh: Option<MeshHandle>,
    pub parent: Option<u32>,
    pub children: Vec<u32>,
    pub visible: bool,
//    pub material: Option<MaterialHandle>,
}