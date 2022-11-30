use glam::Vec3;

use crate::{
    renderer::{mesh::MeshType, Drawable, Renderer},
    transform::Transform,
};

pub enum RenderType {
    STATIC { subscribed: bool },
    DYNAMIC,
}

impl RenderType {
    fn new(is_static: bool) -> Self {
        if is_static {
            Self::STATIC { subscribed: false }
        } else {
            Self::DYNAMIC
        }
    }
}

pub struct Entity {
    transform: Transform,
    mesh_type: MeshType,
    pub render_type: RenderType,
}

impl Entity {
    pub fn new(pos: Vec3, is_static: bool, mesh_type: MeshType) -> Self {
        Self {
            transform: Transform::with_pos(pos),
            mesh_type,
            render_type: RenderType::new(is_static),
        }
    }

    pub fn new_scale(pos: Vec3, scale: Vec3, is_static: bool, mesh_type: MeshType) -> Self {
        Self {
            transform: Transform::with_pos_scale(pos, scale),
            mesh_type,
            render_type: RenderType::new(is_static),
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

    pub fn draw(&mut self, renderer: &mut Renderer) {
        renderer.subscribe(self)
    }
}

impl Drawable for Entity {
    fn get_model_matrix(&self) -> [f32; 16] {
        self.transform.get_matrix().to_cols_array()
    }

    fn get_render_type(&mut self) -> &mut RenderType {
        &mut self.render_type
    }
}
