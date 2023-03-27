use glam::Mat4;

use crate::{application::Window, glcall, scene::Scene};

use self::{mesh::Mesh, shader::ShaderProgram, vertex_array::VertexArray};

mod index_buffer;
mod mesh;
pub mod shader;
mod vertex_array;
pub mod vertex_buffer;

pub struct Renderer {
    count: i32,
    last_static_index: isize,
    shader: ShaderProgram,
    vao: VertexArray,
    _mesh: Mesh,
    projection: Mat4,
}

impl Renderer {
    const MAX_COUNT: isize = (100 as isize).pow(3);

    pub fn cube(window: &Window) -> Self {
        let mesh = Mesh::cube();
        let vao = VertexArray::new(&mesh, Self::MAX_COUNT);
        let shader = ShaderProgram::new(
            include_str!("../resources/vertex.glsl"),
            include_str!("../resources/fragment.glsl"),
        );

        let projection = Mat4::perspective_rh(
            (45 as f32).to_radians(),
            window.get_aspect_ratio(),
            0.1,
            1000.,
        );

        Self {
            count: 0,
            last_static_index: 0,
            shader,
            vao,
            _mesh: mesh,
            projection,
        }
    }

    pub fn on_resize(&mut self, window: &Window) {
        self.projection = Mat4::perspective_rh(
            (45 as f32).to_radians(),
            window.get_aspect_ratio(),
            0.1,
            1000.,
        );
    }

    pub fn draw(&mut self, scene: &mut Scene) {
        self.vao.bind();

        let data: Vec<_> = scene
            .get_mut_static_entities()
            .iter()
            .map(|e| e.get_transform().get_matrix())
            .collect();

        self.count += scene.get_mut_static_entities().len() as i32;

        self.last_static_index += self
            .vao
            .instanced_buffer
            .set_data(&data, self.last_static_index);
        scene.get_mut_static_entities().clear();

        let mut count = self.count;
        let data: Vec<_> = scene
            .get_dynamic_entities()
            .iter()
            .map(|e| e.get_transform().get_matrix())
            .collect();
        count += scene.get_dynamic_entities().len() as i32;

        self.vao
            .instanced_buffer
            .set_data(&data, self.last_static_index);

        self.shader.activate();
        let view_matrix = scene.get_mut_camera().get_matrix();
        self.shader.set_mat4("view", &view_matrix);
        self.shader.set_mat4("projection", &self.projection);
        unsafe {
            glcall!(gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.vao.get_index_size(),
                gl::UNSIGNED_INT,
                std::ptr::null(),
                count,
            ));
        }
    }
}
