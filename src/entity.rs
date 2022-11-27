use glam::Vec3;

use crate::transform::Transform;

pub struct Entity {
    transform: Transform,
    is_static: bool,
}

impl Entity {
    pub fn new(pos: Vec3, is_static: bool) -> Self {
        Self {
            transform: Transform::with_pos(pos),
            is_static,
        }
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    #[inline]
    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }
}
