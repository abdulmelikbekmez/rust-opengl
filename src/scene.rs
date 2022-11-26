use glam::Vec3;

use crate::{camera::Camera, entity::Entity};

pub struct Scene {
    camera: Camera,
    entity_list: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::new();
        Self {
            camera,

            entity_list: vec![],
        }
    }

    pub fn add_static_entities(&mut self, count: i32) {
        let padding = 5;
        for i in -(count / 2)..(count / 2) {
            for j in -(count / 2)..(count / 2) {
                for k in -(count / 2)..(count / 2) {
                    let pos = Vec3::new(
                        (i * padding) as f32,
                        (j * padding) as f32,
                        (k * padding) as f32,
                    );
                    self.entity_list.push(Entity::new(pos, true));
                }
            }
        }
    }

    #[inline]
    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    #[inline]
    pub fn get_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.entity_list
    }
}
