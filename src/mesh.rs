use gl::types::{GLfloat, GLuint};

type Pos = [f32; 3];
type Color = [f32; 3];

pub struct Vertex(pub Pos, pub Color);

pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    element_size: usize,
}

impl Mesh {
    pub fn new(index_arr: &[GLuint], vertex_arr: &[Vertex]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            // Generate Vertex Array and bind
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(index_arr) as isize,
                index_arr.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(vertex_arr) as isize,
                vertex_arr.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                0 as *const _,
            );

            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                (3 * std::mem::size_of::<GLfloat>()) as *const _,
            );

            gl::EnableVertexAttribArray(1);
        }
        Mesh {
            vao,
            vbo,
            ebo,
            element_size: index_arr.len(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.element_size as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        println!("Mesh dropped!");
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
