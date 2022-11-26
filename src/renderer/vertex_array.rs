use gl::types::GLuint;
use glam::Mat4;

use super::{mesh::Mesh, vertex_buffer::VertexBuffer};

pub struct VertexArray {
    index: u32,
    id: GLuint,
    index_size: i32,
    instanced_buffer: Option<VertexBuffer>,
    mesh: Mesh,
}

static mut BINDED_ID: GLuint = 0;

impl VertexArray {
    fn new(mesh: Mesh) -> Self {
        unsafe {
            let mut id = 0;
            let mut index: u32 = 0;
            gl::CreateVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
            BINDED_ID = id;
            mesh.index_buffer.bind();
            for buffer in mesh.vb_list.iter() {
                buffer.bind();
                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(
                    index,
                    buffer.get_size(),
                    gl::FLOAT,
                    gl::FALSE,
                    buffer.get_byte_length(),
                    0 as *const _,
                );
                index += 1;
            }
            Self {
                index,
                id,
                index_size: mesh.index_buffer.count,
                instanced_buffer: None,
                mesh,
            }
        }
    }

    pub fn cube() -> Self {
        Self::new(Mesh::cube())
    }

    pub fn make_instanced(&mut self, instance_count: isize) {
        self.bind();
        let buffer = VertexBuffer::instanced::<Mat4>(instance_count);
        buffer.bind();
        for i in 0..4 {
            unsafe {
                gl::EnableVertexAttribArray(self.index);
                gl::VertexAttribPointer(
                    self.index,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    buffer.get_byte_length(),
                    (i * buffer.get_size()) as *const _,
                );
                gl::VertexAttribDivisor(self.index, 1);
                self.index += 1;
            }
        }
        self.instanced_buffer = Some(buffer);
    }

    #[inline]
    pub fn get_instanced_buffer(&mut self) -> &mut Option<VertexBuffer> {
        &mut self.instanced_buffer
    }

    #[inline]
    pub fn get_index_size(&self) -> i32 {
        return self.index_size;
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}
