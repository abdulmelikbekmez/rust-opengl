use glam::Vec3;
use glutin::event::{MouseButton, MouseScrollDelta};

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
    target: Option<Entity>,
    cursor: Entity,
    cursor_dist: f32,
    swarm: Swarm,
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::new();
        let mut swarm = Swarm::new(4);
        let entity_list = vec![Entity::new_scale(
            Vec3::new(0., -20., 0.),
            Vec3::new(500., 0.2, 500.),
            true,
            MeshType::CUBE,
        )];
        swarm.start();

        Self {
            camera,
            entity_list,
            cursor: Entity::new(Vec3::default(), false, MeshType::CUBE),
            cursor_dist: 10.,
            target: None,
            swarm,
        }
    }

    pub fn draw(&mut self, _: &Window, renderer: &mut Renderer) {
        for e in self.entity_list.iter_mut() {
            e.draw(renderer);
        }
        self.cursor.draw(renderer);
        self.swarm.draw(renderer);
        for i in self.target.iter_mut() {
            i.draw(renderer);
        }
    }

    pub fn update(&mut self, key_state: &KeyboardState, _: &Window) {
        self.camera.handle_input(key_state);
        self.cursor
            .get_mut_transform()
            .set_pos(self.camera.position + self.camera.direction * self.cursor_dist);
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

    pub fn on_mouse_click(&mut self, button: &MouseButton) {
        match button {
            MouseButton::Left => {
                let pos = self.cursor.get_transform().get_pos();
                self.target
                    .get_or_insert_with(|| Entity::new(pos, false, MeshType::CUBE))
                    .get_mut_transform()
                    .set_pos(pos);
                self.swarm.set_target(pos);
            }
            MouseButton::Right => {
                self.swarm.clear_target();
                self.target = None;
            }
            MouseButton::Middle => todo!(),
            MouseButton::Other(_) => todo!(),
        }
    }

    pub fn on_mouse_wheel(&mut self, delta: &MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, y) => self.cursor_dist += y,
            _ => return,
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
}
