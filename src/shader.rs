use gl::types::*;
use glam::Mat4;
use std::ffi::CString;
use std::ptr;
use std::str;

struct Shader {
    id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        println!("Shader deleted!");
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub fn new(src: &str, shader_type: GLenum) -> Self {
        let id;
        unsafe {
            id = gl::CreateShader(shader_type);
            // Attempt to compile the shader
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(id, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(id);

            check_compile_status(&id);
        }
        Self { id }
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        println!("ShaderProgram deleted");
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        // let a = std::fs::read_to_string(vertex_path).expect("Unable to read vertex shader!!");
        // let b = std::fs::read_to_string(fragment_path).expect("Unable to read fragment shader");

        let vs = Shader::new(&vertex_src, gl::VERTEX_SHADER);
        let fs = Shader::new(&fragment_src, gl::FRAGMENT_SHADER);

        let id = link_program(&vs, &fs);

        return Self { id };
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    #[allow(dead_code)]
    fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(self.id, c_str.as_ptr());
            gl::Uniform1i(location, value as GLint);
        }
    }

    #[allow(dead_code)]
    pub fn set_int(&self, name: &str, value: i8) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(self.id, c_str.as_ptr());
            gl::Uniform1i(location, value as GLint);
        }
    }

    #[allow(dead_code)]
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(self.id, c_str.as_ptr());
            gl::Uniform1f(location, value as GLfloat);
        }
    }

    pub fn set_mat4(&self, name: &str, mat4: &Mat4) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(self.id, c_str.as_ptr());
            gl::UniformMatrix4fv(location, 1, 0, mat4.to_cols_array().as_ptr());
        }
    }
}

unsafe fn check_compile_status(id_shader: &GLuint) {
    // Get the compile status
    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(*id_shader, gl::COMPILE_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        gl::GetShaderiv(*id_shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderInfoLog(*id_shader, len, &mut len, buf.as_mut_ptr() as *mut _);
        panic!(
            "{}",
            str::from_utf8(&buf)
                .ok()
                .expect("ShaderInfoLog not valid utf8")
        );
    }
}

fn link_program(vs: &Shader, fs: &Shader) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs.id);
        gl::AttachShader(program, fs.id);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}
