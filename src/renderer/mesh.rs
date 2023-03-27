use gl::types::*;

use super::{
    index_buffer::IndexBuffer,
    vertex_buffer::{Buffer, Static},
};

type Pos = [f32; 3];
type Color = [f32; 3];

const POS_DATA: [Pos; 8] = [
    [-0.5, -0.5, 0.5],
    [0.5, -0.5, 0.5],
    [0.5, 0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [-0.5, -0.5, -0.5],
    [0.5, -0.5, -0.5],
    [0.5, 0.5, -0.5],
    [-0.5, 0.5, -0.5],
];

const COLOR_DATA: [Color; 8] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
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
    pub vb_list: Vec<Buffer<Static>>,
    pub index_buffer: IndexBuffer,
}

impl Mesh {
    fn new<const N: usize, R>(
        pos_data: &[[R; N]],
        color_data: &[[R; N]],
        index_data: &[GLuint],
    ) -> Self {
        Self {
            vb_list: vec![
                Buffer::<Static>::new(pos_data),
                Buffer::<Static>::new(color_data),
            ],
            index_buffer: IndexBuffer::new(index_data),
        }
    }

    pub fn cube() -> Self {
        Self::new(&POS_DATA, &COLOR_DATA, &ELEMENT_DATA)
    }
}
