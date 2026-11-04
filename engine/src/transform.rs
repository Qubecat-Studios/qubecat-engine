use glam::{Mat4, Quat, Vec3};

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