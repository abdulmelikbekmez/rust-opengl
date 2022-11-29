use gl::types::GLuint;

pub struct VertexBuffer {
    id: GLuint,
    element_count: i32,
    byte_length: i32,
}

static mut BINDED_ID: GLuint = 0;

impl VertexBuffer {
    pub fn new<const N: usize>(data: &[[f32; N]]) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            BINDED_ID = id;
            gl::BufferData(
                gl::ARRAY_BUFFER,
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
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            BINDED_ID = id;
            gl::BufferData(
                gl::ARRAY_BUFFER,
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
    pub fn get_element_count(&self) -> i32 {
        return self.element_count;
    }

    #[inline]
    pub fn get_byte_length(&self) -> i32 {
        return self.byte_length;
    }

    #[inline]
    pub fn bind(&self) {
        unsafe {
            if self.id != BINDED_ID {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
                BINDED_ID = self.id;
            }
        }
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
