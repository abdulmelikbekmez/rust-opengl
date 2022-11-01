use gl::types::GLuint;
type Pos = [f32; 3];
type Color = [f32; 3];

pub struct Vertex(pub Pos, pub Color);

pub struct VertexBuffer {
    id: GLuint,
}

static mut BINDED_ID: GLuint = 0;

impl VertexBuffer {
    pub fn new(data: &[Vertex]) -> Self {
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
        Self { id }
    }

    #[allow(dead_code)]
    pub fn bind(&self) {
        unsafe {
            if self.id != BINDED_ID {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
                BINDED_ID = self.id;
            }
        }
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            BINDED_ID = 0;
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
