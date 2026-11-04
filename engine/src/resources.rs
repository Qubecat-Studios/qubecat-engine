use crate::{camera::Camera, input::Input, time::Time};

/// Ресурсы: всё игровое состояние вне мира сущностей.
pub struct Resources {
    pub time: Time,
    pub input: Input,
    pub camera: Camera
}
