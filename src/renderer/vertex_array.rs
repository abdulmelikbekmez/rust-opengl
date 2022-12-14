use gl::types::GLuint;
use glam::Mat4;

use super::{mesh::Mesh, vertex_buffer::VertexBuffer};

static mut BINDED_ID: GLuint = 0;

pub struct VertexArray {
    id: GLuint,
    index_size: i32,
    instanced_buffer: VertexBuffer,
}

impl VertexArray {
    pub fn new(mesh: &Mesh, instance_count: isize) -> Self {
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
                    buffer.get_element_count(),
                    gl::FLOAT,
                    gl::FALSE,
                    buffer.get_byte_length(),
                    std::ptr::null(),
                );
                index += 1;
            }
            let instanced_buffer = VertexBuffer::instanced::<Mat4>(instance_count);
            instanced_buffer.bind();
            for i in 0..4 {
                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(
                    index,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    instanced_buffer.get_byte_length(),
                    (i * instanced_buffer.get_element_count()) as *const _,
                );
                gl::VertexAttribDivisor(index, 1);
                index += 1;
            }

            Self {
                id,
                index_size: mesh.index_buffer.count,
                instanced_buffer,
            }
        }
    }

    // pub fn make_instanced(&mut self, instance_count: isize) {
    //     let buffer = VertexBuffer::instanced::<Mat4>(instance_count);
    //     buffer.bind();
    //     for i in 0..4 {
    //         unsafe {
    //             gl::EnableVertexAttribArray(self.index);
    //             gl::VertexAttribPointer(
    //                 self.index,
    //                 4,
    //                 gl::FLOAT,
    //                 gl::FALSE,
    //                 buffer.get_byte_length(),
    //                 (i * buffer.get_size()) as *const _,
    //             );
    //             gl::VertexAttribDivisor(self.index, 1);
    //             self.index += 1;
    //         }
    //     }
    //     self.instanced_buffer = Some(buffer);
    // }

    #[inline]
    pub fn get_instanced_buffer(&mut self) -> &mut VertexBuffer {
        &mut self.instanced_buffer
    }

    #[inline]
    pub fn get_index_size(&self) -> i32 {
        return self.index_size;
    }

    #[inline]
    pub fn bind(&self) {
        unsafe {
            if BINDED_ID != self.id {
                gl::BindVertexArray(self.id);
                BINDED_ID = self.id;
            }
        }
    }
}
