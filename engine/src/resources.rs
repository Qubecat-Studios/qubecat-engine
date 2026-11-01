use crate::{camera::Camera, input::Input, time::Time, voxels::voxel_world::VoxelWorld};

/// Ресурсы: всё игровое состояние вне мира сущностей.
pub struct Resources {
    pub time: Time,
    pub input: Input,
    pub camera: Camera,
    pub voxels: VoxelWorld,
}
