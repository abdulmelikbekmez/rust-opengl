use glam::Mat4;

use crate::{application::Window, scene::Scene};

use self::{mesh::Mesh, shader::ShaderProgram, vertex_array::VertexArray};

mod index_buffer;
mod mesh;
mod shader;
mod vertex_array;
mod vertex_buffer;

pub struct Renderer {
    count: usize,
    last_static_index: isize,
    shader: ShaderProgram,
    vao: VertexArray,
    _mesh: Mesh,
}

impl Renderer {
    const MAX_COUNT: isize = (30 as isize).pow(3);

    pub fn cube() -> Self {
        let mesh = Mesh::cube();
        let vao = VertexArray::new(&mesh, Self::MAX_COUNT);
        let shader = ShaderProgram::new(
            include_str!("../resources/vertex.glsl"),
            include_str!("../resources/fragment.glsl"),
        );

        Self {
            count: 0,
            last_static_index: 0,
            shader,
            vao,
            _mesh: mesh,
        }
    }

    pub fn draw(&mut self, scene: &mut Scene, window: &Window) {
        self.vao.bind();
        self.shader.activate();
        let buffer = self.vao.get_instanced_buffer();
        buffer.bind();

        let mut i = 0;
        while i < scene.get_entities().len() {
            if scene.get_entities()[i].is_static() {
                let e = scene.get_entities().remove(i);
                let data = e.get_transform().get_matrix().to_cols_array();
                let byte_length = std::mem::size_of_val(&data) as isize;
                unsafe {
                    gl::BufferSubData(
                        gl::ARRAY_BUFFER,
                        self.last_static_index,
                        byte_length,
                        data.as_ptr() as *const _,
                    );
                }
                self.count += 1;
                self.last_static_index += byte_length;
            } else {
                i += 1;
            }
        }

        let mut count = self.count as i32;
        let mut index = self.last_static_index;
        for e in scene.get_entities() {
            let data = e.get_transform().get_matrix().to_cols_array();
            let byte_length = std::mem::size_of_val(&data) as isize;
            unsafe {
                gl::BufferSubData(gl::ARRAY_BUFFER, index, byte_length, data.as_ptr().cast());
            }
            count += 1;
            index += byte_length;
        }

        let view_matrix = scene.get_camera().get_matrix();
        self.shader.set_mat4("view", &view_matrix);

        let aspect_ratio = window.width / window.height;
        let projection = Mat4::perspective_rh((45 as f32).to_radians(), aspect_ratio, 0.1, 1000.);
        self.shader.set_mat4("projection", &projection);
        let index_size = self.vao.get_index_size();
        unsafe {
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                index_size,
                gl::UNSIGNED_INT,
                0 as *const _,
                count,
            )
        }
    }
}
