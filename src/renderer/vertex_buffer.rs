use std::time::Instant;

use gl::types::GLuint;

pub struct VertexBuffer {
    id: GLuint,
    element_count: i32,
    byte_length: i32,
}

impl VertexBuffer {
    pub fn new<const N: usize>(data: &[[f32; N]]) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(
                id,
                std::mem::size_of_val(data) as isize,
                data.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
        Self {
            id,
            element_count: N as i32,
            byte_length: N as i32 * std::mem::size_of::<f32>() as i32,
        }
    }

    pub fn instanced<T>(element_count: isize) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(
                id,
                element_count * std::mem::size_of::<T>() as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            let byte_length = std::mem::size_of::<T>() as i32;
            Self {
                id,
                element_count: byte_length / std::mem::size_of::<f32>() as i32,
                byte_length,
            }
        }
    }

    #[inline]
    pub fn get_id(&self) -> GLuint {
        return self.id;
    }

    #[inline]
    pub fn get_element_count(&self) -> i32 {
        return self.element_count;
    }

    #[inline]
    pub fn get_byte_length(&self) -> i32 {
        return self.byte_length;
    }

    pub fn set_data<T>(&self, data: &[T], offset: isize) -> isize {
        let byte_length = (std::mem::size_of::<T>() * data.len()) as isize;
        let t = Instant::now();
        unsafe { gl::NamedBufferSubData(self.id, offset, byte_length, data.as_ptr().cast()) }
        println!(
            "set data with byte {} takes {} micro",
            byte_length,
            t.elapsed().as_micros()
        );
        byte_length
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            println!("vertex buffer deleted");
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
