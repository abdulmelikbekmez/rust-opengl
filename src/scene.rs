use glam::Vec3;

use crate::{camera::Camera, entity::Entity};

pub struct Scene {
    camera: Camera,
    static_entity_list: Vec<Entity>,
    dynamic_entity_list: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::new();
        Self {
            camera,
            static_entity_list: vec![],
            dynamic_entity_list: vec![],
        }
    }

    pub fn update(&mut self) {
        for e in self.dynamic_entity_list.iter_mut() {
            e.get_mut_transform().update_pos(Vec3 {
                x: 0.01,
                y: 0.,
                z: 0.005,
            });
        }
    }

    pub fn add_entities(&mut self, count: i32, is_static: bool) {
        let padding = 5;
        for i in -(count / 2)..(count / 2) {
            for j in -(count / 2)..(count / 2) {
                for k in -(count / 2)..(count / 2) {
                    let pos = Vec3::new(
                        (i * padding) as f32,
                        (j * padding) as f32,
                        (k * padding) as f32,
                    );
                    if is_static {
                        self.static_entity_list.push(Entity::new(pos));
                    } else {
                        self.dynamic_entity_list.push(Entity::new(pos));
                    }
                }
            }
        }
    }

    #[inline]
    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    #[inline]
    pub fn get_static_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.static_entity_list
    }

    #[inline]
    pub fn get_dynamic_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.dynamic_entity_list
    }
}
