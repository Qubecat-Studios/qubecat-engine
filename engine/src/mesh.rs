use glam::{Mat4, Quat, Vec3};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub u32);

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat, // Кватерион
    pub scale: Vec3,
}

impl Transform {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
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