use glam::Mat4;

use crate::{application::Window, scene::Scene};

use self::{mesh::Mesh, shader::ShaderProgram, vertex_array::VertexArray};

mod index_buffer;
mod mesh;
mod shader;
mod vertex_array;
mod vertex_buffer;

pub struct Renderer {
    count: i32,
    last_static_index: isize,
    shader: ShaderProgram,
    vao: VertexArray,
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
            count: 0,
            last_static_index: 0,
            shader,
            vao,
            _mesh: mesh,
        }
    }

    /// Ensure that related vertex buffer is binded!!
    /// Other vise this function may be work unexpectedly
    fn set_data(&mut self, data: &[f32], update: bool) {
        let byte_length = (data.len() * std::mem::size_of::<f32>()) as isize;
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                self.last_static_index,
                byte_length,
                data.as_ptr().cast(),
            );
        }

        if update {
            self.last_static_index += byte_length;
        }
    }

    pub fn draw(&mut self, scene: &mut Scene, window: &Window) {
        self.vao.bind();
        self.vao.get_instanced_buffer().bind();

        let data: Vec<_> = scene
            .get_static_entities()
            .iter()
            .flat_map(|e| {
                self.count += 1;
                e.get_transform().get_matrix().to_cols_array()
            })
            .collect();

        self.set_data(&data, true);
        scene.get_static_entities().clear();

        let mut count = self.count;
        let data: Vec<_> = scene
            .get_dynamic_entities()
            .iter()
            .flat_map(|e| {
                count += 1;
                e.get_transform().get_matrix().to_cols_array()
            })
            .collect();

        self.set_data(&data, false);

        self.shader.activate();
        let view_matrix = scene.get_camera().get_matrix();
        self.shader.set_mat4("view", &view_matrix);

        let projection = Mat4::perspective_rh(
            (45 as f32).to_radians(),
            window.get_aspect_ratio(),
            0.1,
            1000.,
        );
        self.shader.set_mat4("projection", &projection);
        let index_size = self.vao.get_index_size();
        unsafe {
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                index_size,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                count,
            )
        }
    }
}
