use gl::types::GLuint;

pub struct VertexBuffer {
    id: GLuint,
    size: i32,
    length: i32,
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
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
        Self {
            id,
            size: N as i32,
            length: N as i32 * std::mem::size_of::<f32>() as i32,
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
                0 as *const _,
                gl::DYNAMIC_DRAW,
            );

            Self {
                id,
                size: element_count as i32,
                length: element_count as i32 * std::mem::size_of::<T>() as i32,
            }
        }
    }

    #[inline]
    pub fn get_size(&self) -> i32 {
        return self.size;
    }

    #[inline]
    pub fn get_byte_length(&self) -> i32 {
        return self.length;
    }

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
