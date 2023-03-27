use std::marker::PhantomData;

use gl::types::GLuint;

pub trait Vertex {
    fn size(&self) -> u32;
}

pub trait Buf {}

pub struct Dynamic;
impl Buf for Dynamic {}
pub struct Static;
impl Buf for Static {}

pub struct Buffer<T: Buf> {
    id: GLuint,
    element_count: i32,
    stride: i32,
    t_buf_type: PhantomData<T>,
}

impl<T: Buf> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            println!("buffer deleted");
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl<T: Buf> Buffer<T> {
    #[inline(always)]
    pub fn get_id(&self) -> GLuint {
        self.id
    }

    #[inline(always)]
    pub fn get_element_count(&self) -> i32 {
        self.element_count
    }

    #[inline(always)]
    pub fn get_stride(&self) -> i32 {
        self.stride
    }
}

impl Buffer<Static> {
    pub fn new<T, const N: usize>(data: &[[T; N]]) -> Self {
        let mut id = 0;
        let stride = std::mem::size_of::<T>() as i32 * N as i32;
        let element_count = N as i32;
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
            element_count,
            stride,
            t_buf_type: PhantomData,
        }
    }
}

impl Buffer<Dynamic> {
    pub fn set_data<T>(&self, data: &[T], offset: isize) -> isize {
        let byte_length = (std::mem::size_of::<T>() * data.len()) as isize;
        unsafe { gl::NamedBufferSubData(self.get_id(), offset, byte_length, data.as_ptr().cast()) }
        byte_length
    }

    pub fn instanced<T>(instance_count: isize) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(
                id,
                instance_count * std::mem::size_of::<T>() as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            let stride = std::mem::size_of::<T>() as i32;
            Self {
                id,
                element_count: stride / std::mem::size_of::<f32>() as i32,
                stride,
                t_buf_type: PhantomData,
            }
        }
    }
}
