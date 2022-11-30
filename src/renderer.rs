use std::os::raw::c_void;

use glam::Mat4;

use crate::{application::Window, camera::Camera, entity::RenderType};

use self::{mesh::Mesh, shader::ShaderProgram, vertex_array::VertexArray};

mod index_buffer;
pub mod mesh;
mod shader;
mod vertex_array;
mod vertex_buffer;

pub trait Drawable {
    fn get_model_matrix(&self) -> [f32; 16];
    fn get_render_type(&mut self) -> &mut RenderType;
}

pub struct Renderer {
    static_count: i32,
    dynamic_count: i32,
    last_static_index: isize,
    shader: ShaderProgram,
    vao: VertexArray,
    static_buffer: Vec<f32>,
    dynamic_buffer: Vec<f32>,
    _mesh: Mesh,
}

impl Renderer {
    const MAX_COUNT: isize = (100 as isize).pow(3);

    pub fn cube() -> Self {
        let mesh = Mesh::cube();
        let vao = VertexArray::new(&mesh, Self::MAX_COUNT);
        let shader = ShaderProgram::new(
            include_str!("../resources/vertex.glsl"),
            include_str!("../resources/fragment.glsl"),
        );

        Self {
            static_count: 0,
            dynamic_count: 0,
            last_static_index: 0,
            shader,
            vao,
            static_buffer: vec![],
            dynamic_buffer: vec![],
            _mesh: mesh,
        }
    }

    fn set_data(&mut self, byte_length: isize, pointer: *const c_void, is_static: bool) {
        self.vao.get_instanced_buffer().bind();
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                self.last_static_index,
                byte_length,
                pointer,
            );
        }
        if is_static {
            self.last_static_index += byte_length
        }
    }

    fn update_buffers(&mut self) {
        let byte_length = (self.static_buffer.len() * 4) as isize;
        if byte_length > 0 {
            let pointer = self.static_buffer.as_ptr();
            self.set_data(byte_length, pointer.cast(), true);
            self.static_buffer.clear();
        }

        let byte_length = (self.dynamic_buffer.len() * 4) as isize;
        if byte_length > 0 {
            let pointer = self.dynamic_buffer.as_ptr();
            self.set_data(byte_length, pointer.cast(), false);
            self.dynamic_buffer.clear();
        }
    }

    pub fn draw(&mut self, window: &Window, camera: &Camera) {
        self.vao.bind();
        self.shader.activate();

        self.update_buffers();

        let view_matrix = camera.get_matrix();
        self.shader.set_mat4("view", &view_matrix);

        let projection = Mat4::perspective_rh(
            (45 as f32).to_radians(),
            window.get_aspect_ratio(),
            0.1,
            1000.,
        );
        self.shader.set_mat4("projection", &projection);
        unsafe {
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.vao.get_index_size(),
                gl::UNSIGNED_INT,
                std::ptr::null(),
                self.static_count + self.dynamic_count,
            )
        }
        self.dynamic_count = 0;
    }

    pub fn subscribe(&mut self, entity: &mut dyn Drawable) {
        match entity.get_render_type() {
            RenderType::STATIC { subscribed: true } => return,
            RenderType::STATIC { ref mut subscribed } => {
                *subscribed = true;
                let data = entity.get_model_matrix();
                self.static_count += 1;
                self.static_buffer.extend(data.iter());
            }
            RenderType::DYNAMIC => {
                let data = entity.get_model_matrix();
                self.dynamic_count += 1;
                self.dynamic_buffer.extend(data.iter());
            }
        }
    }
}
