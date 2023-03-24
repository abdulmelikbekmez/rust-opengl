use gl::types::GLuint;

pub struct IndexBuffer {
    id: GLuint,
    pub count: i32,
}

impl IndexBuffer {
    pub fn new(data: &[GLuint]) -> Self {
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
            count: data.len() as i32,
        }
    }

    #[inline]
    pub fn get_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            println!("index buffer deleted");
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
