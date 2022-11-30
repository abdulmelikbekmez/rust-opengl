use glam::Vec3;

use crate::{
    application::Window,
    camera::Camera,
    entity::Entity,
    key::KeyboardState,
    renderer::{mesh::MeshType, Renderer},
    swarm::Swarm,
};

pub struct Scene {
    camera: Camera,
    entity_list: Vec<Entity>,
    tmp: Entity,
    swarm: Swarm,
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::new();
        let mut swarm = Swarm::new(10);
        let entity_list = vec![Entity::new_scale(
            Vec3::default(),
            Vec3::new(500., 0.2, 500.),
            true,
            MeshType::CUBE,
        )];
        swarm.start();

        Self {
            camera,
            entity_list,
            tmp: Entity::new_scale(Vec3::default(), Vec3::ONE * 3., false, MeshType::CUBE),
            swarm,
        }
    }

    pub fn draw(&mut self, _: &Window, renderer: &mut Renderer) {
        // for e in self.get_mut_entities() {
        //     e.draw(renderer);
        // }
        self.tmp.draw(renderer);
        self.swarm.draw(renderer);
    }

    pub fn update(&mut self, key_state: &KeyboardState, _: &Window) {
        self.camera.handle_input(key_state);
        self.tmp
            .get_mut_transform()
            .set_pos(self.camera.position + self.camera.direction * 100.);
        self.swarm.update();
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
                    self.entity_list
                        .push(Entity::new(pos, is_static, MeshType::CUBE));
                }
            }
        }
    }

    #[inline]
    pub fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    #[inline]
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    #[inline]
    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entity_list
    }

    #[inline]
    pub fn get_mut_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.entity_list
    }
}
