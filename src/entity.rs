use glam::Vec3;

use crate::transform::Transform;

pub struct Entity {
    transform: Transform,
}

impl Entity {
    pub fn new(pos: Vec3) -> Self {
        Self {
            transform: Transform::with_pos(pos),
        }
    }

    #[inline]
    pub fn get_mut_transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    #[inline]
    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }
}
