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

#[allow(dead_code)]
pub struct Mesh {
    vao: GLuint,
    vb: VertexBuffer,
    ib: IndexBuffer,
}

static mut BINDED_ID: GLuint = 0;

fn bind(id: GLuint) {
    unsafe {
        if id != BINDED_ID {
            gl::BindVertexArray(id);
            BINDED_ID = id;
        }
    }
}

fn unbind() {
    unsafe {
        gl::BindVertexArray(0);
        BINDED_ID = 0;
    }
}

impl Mesh {
    pub fn cube() -> Self {
        Mesh::new(&ELEMENT_DATA, &VERTICES)
    }

    fn new(index_arr: &[GLuint], vertex_arr: &[Vertex]) -> Self {
        let vb = VertexBuffer::new(vertex_arr);
        unsafe {
            let mut vao = 0;
            // Generate Vertex Array and bind
            gl::GenVertexArrays(1, &mut vao);
            bind(vao);
            gl::BindVertexArray(vao);

            let ib = IndexBuffer::new(index_arr);

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
            unbind();
            ib.unbind();

            Mesh { vao, vb, ib }
        }
    }

    pub fn bind(&self) {
        bind(self.vao)
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        unbind();
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
