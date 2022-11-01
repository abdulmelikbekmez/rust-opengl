use gl::types::{GLfloat, GLuint};

mod index_buffer;
mod vertex_buffer;

use vertex_buffer::Vertex;

use self::{index_buffer::IndexBuffer, vertex_buffer::VertexBuffer};

#[rustfmt::skip]
const VERTICES: [Vertex; 8] = [
    Vertex([-0.5, -0.5,  0.5], [1.0, 0.0, 0.0]),
    Vertex([0.5,  -0.5,  0.5], [0.0, 1.0, 0.0]),
    Vertex([0.5,   0.5,  0.5], [0.0, 0.0, 1.0]),
    Vertex([-0.5,  0.5,  0.5], [0.0, 1.0, 0.0]),

    Vertex([-0.5, -0.5, -0.5], [1.0, 0.0, 0.0]),
    Vertex([0.5,  -0.5, -0.5], [0.0, 1.0, 0.0]),
    Vertex([0.5,   0.5, -0.5], [0.0, 0.0, 1.0]),
    Vertex([-0.5,  0.5, -0.5], [0.0, 1.0, 0.0])
];

const ELEMENT_DATA: [GLuint; 36] = [
    // front
    0, 1, 2, 2, 3, 0, // right
    1, 5, 6, 6, 2, 1, // back
    7, 6, 5, 5, 4, 7, // left
    4, 0, 3, 3, 7, 4, // bottom
    4, 5, 1, 1, 0, 4, // top
    3, 2, 6, 6, 7, 3,
];

pub struct Mesh {
    vao: GLuint,
    #[allow(dead_code)]
    vb: VertexBuffer,
    ib: IndexBuffer,
}

impl Mesh {
    pub fn cube() -> Self {
        Mesh::new(&ELEMENT_DATA, &VERTICES)
    }

    fn new(index_arr: &[GLuint], vertex_arr: &[Vertex]) -> Self {
        unsafe {
            let mut vao = 0;
            // Generate Vertex Array and bind
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let ib = IndexBuffer::new(index_arr);
            let vb = VertexBuffer::new(vertex_arr);

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                0 as *const _,
            );
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                (3 * std::mem::size_of::<GLfloat>()) as *const _,
            );
            vb.unbind();
            gl::BindVertexArray(0);
            ib.unbind();

            Mesh { vao, vb, ib }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        self.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.ib.count,
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
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
