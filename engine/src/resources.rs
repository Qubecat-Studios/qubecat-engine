use std::collections::HashMap;

use crate::{camera::Camera, input::Input, mesh::{Mesh, MeshHandle}, time::Time};

/// Ресурсы: всё игровое состояние вне мира сущностей.
pub struct Resources {
    pub time: Time,
    pub input: Input,
    pub camera: Camera,
    pub assets: HashMap<MeshHandle, Mesh>
}
