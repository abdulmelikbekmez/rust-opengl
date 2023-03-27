use gl::types::GLuint;
use glam::Mat4;

use super::{
    mesh::Mesh,
    vertex_buffer::{Buffer, Dynamic},
};

static mut BINDED_ID: GLuint = 0;

pub struct VertexArray {
    id: GLuint,
    index_size: i32,
    pub instanced_buffer: Buffer<Dynamic>,
}

impl VertexArray {
    pub fn new(mesh: &Mesh, instance_count: isize) -> Self {
        unsafe {
            let mut id = 0;
            gl::CreateVertexArrays(1, &mut id);
            gl::VertexArrayElementBuffer(id, mesh.index_buffer.get_id());

            let mut binding_index: u32 = 0;
            let mut index: u32 = 0;

            for buffer in mesh.vb_list.iter() {
                // bind buffer to vertex arrays specific index
                gl::VertexArrayVertexBuffer(
                    id,
                    binding_index,
                    buffer.get_id(),
                    0,
                    buffer.get_stride(),
                );

                // enable vertex array attrib
                gl::EnableVertexArrayAttrib(id, index);

                // bind vertex attrib to buffers binded index
                gl::VertexArrayAttribBinding(id, index, binding_index);
                gl::VertexArrayAttribFormat(
                    id,
                    index,
                    buffer.get_element_count(),
                    gl::FLOAT,
                    gl::FALSE,
                    0,
                );
                binding_index += 1;
                index += 1;
            }
            let instanced_buffer = Buffer::<Dynamic>::instanced::<Mat4>(instance_count);

            gl::VertexArrayVertexBuffer(
                id,
                binding_index,
                instanced_buffer.get_id(),
                0,
                instanced_buffer.get_stride(),
            );

            for i in 0..4 {
                gl::EnableVertexArrayAttrib(id, index);
                gl::VertexArrayAttribBinding(id, index, binding_index);
                gl::VertexArrayAttribFormat(id, index, 4, gl::FLOAT, gl::FALSE, i * 16);
                index += 1;
            }
            gl::VertexArrayBindingDivisor(id, binding_index, 1);

            Self {
                id,
                index_size: mesh.index_buffer.count,
                instanced_buffer,
            }
        }
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
