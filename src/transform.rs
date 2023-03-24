use glam::{Mat4, Quat, Vec3};

pub struct Transform {
    position: Vec3,
    scale: Vec3,
    rotation: Rotation,
}

struct Rotation {
    direction: Vec3,
    angle: f32,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            direction: Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            },
            angle: 0.,
        }
    }
}

impl Rotation {
    #[inline]
    pub fn get_rotation(&self) -> Quat {
        Quat::from_axis_angle(self.direction, self.angle)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            scale: Vec3::ONE,
            rotation: Rotation::default(),
        }
    }
}

impl Transform {
    pub fn update_pos(&mut self, dif: Vec3) {
        self.position += dif;
    }

    pub fn with_pos(position: Vec3) -> Self {
        Self {
            position,
            scale: Vec3::ONE,
            rotation: Rotation::default(),
        }
    }

    #[inline]
    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation.get_rotation(),
            self.position,
        )
    }
}
