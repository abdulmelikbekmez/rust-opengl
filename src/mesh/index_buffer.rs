use gl::types::GLuint;

pub struct IndexBuffer {
    id: GLuint,
    pub count: i32,
}

static mut BINDED_ID: GLuint = 0;

impl IndexBuffer {
    pub fn new(data: &[GLuint]) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            BINDED_ID = id;
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(data) as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
        Self {
            id,
            count: data.len() as i32,
        }
    }

    #[allow(dead_code)]
    pub fn bind(&self) {
        unsafe {
            if self.id != BINDED_ID {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
                BINDED_ID = self.id;
            }
        }
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            BINDED_ID = 0;
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
