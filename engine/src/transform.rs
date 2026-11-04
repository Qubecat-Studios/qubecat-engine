use glam::{Mat4, Quat, Vec3};

/// Позиция / поворот (кватернион) / масштаб узла сцены.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn compose(parent: &Transform, local: &Transform) -> Mat4 {
        parent.matrix() * local.matrix()
    }
}
