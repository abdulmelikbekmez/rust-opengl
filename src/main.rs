use camera::Camera;
use gl::types::*;
use glam::Mat4;
use key::KeyboardState;
use std::ffi::c_void;
use std::mem;
use transform::Transform;

mod camera;
mod key;
mod shader;
mod transform;

use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;

use crate::shader::ShaderProgram;

type Pos = [f32; 3];
type Color = [f32; 3];

struct Vertex(Pos, Color);

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

type ElementType = [GLuint; 36];
const ELEMENT_DATA: ElementType = [
    // front
    0, 1, 2, 2, 3, 0, // right
    1, 5, 6, 6, 2, 1, // back
    7, 6, 5, 5, 4, 7, // left
    4, 0, 3, 3, 7, 4, // bottom
    4, 5, 1, 1, 0, 4, // top
    3, 2, 6, 6, 7, 3,
];

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let window = glutin::window::WindowBuilder::new().with_title("hello opengl with rust");
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_context = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_context.get_proc_address(symbol));

    // Create GLSL shaders
    let shader = ShaderProgram::new("resources/vertex.glsl", "resources/fragment.glsl");

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create Element Buffer Object
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            mem::size_of::<ElementType>() as isize,
            ELEMENT_DATA.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (_, b, _) = VERTICES.align_to::<u8>();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            b.len() as isize,
            b.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Use shader program
        shader.activate();

        // Specify the layout of the vertex data
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<GLfloat>() as i32 * 6,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<GLfloat>() as i32 * 6,
            (3 * mem::size_of::<GLfloat>()) as *const _,
        );

        gl::EnableVertexAttribArray(1);
    }

    unsafe {
        gl::ClearColor(1., 1., 1., 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut key_state = KeyboardState::new();

    let transform = Transform::new();
    let mut camera = Camera::new();
    gl_context.window().set_cursor_visible(false);
    gl_context
        .window()
        .set_cursor_grab(glutin::window::CursorGrabMode::Confined)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        // *control_flow = ControlFlow::Wait;

        if key_state.is_pressed(&VirtualKeyCode::Escape) {
            *control_flow = ControlFlow::Exit;
            return;
        }

        camera.handle_input(&key_state);

        let model = transform.get_matrix();
        shader.set_mat4("model", &model);

        let view = camera.get_matrix();
        shader.set_mat4("view", &view);

        let projection =
            Mat4::perspective_rh((45 as f32).to_radians() as f32, 800. / 600., 0.1, 100.);
        shader.set_mat4("projection", &projection);
        gl_context.window().request_redraw();

        match event {
            Event::LoopDestroyed => return,
            Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::MouseMotion { delta } => camera.update(delta),
                _ => (),
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => key_state.process_event(state, virtual_code),
                WindowEvent::CloseRequested => {
                    // Cleanup
                    unsafe {
                        gl::DeleteBuffers(1, &vbo);
                        gl::DeleteVertexArrays(1, &vao);
                    }
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::Resized(size) => unsafe {
                    gl::Viewport(0, 0, size.width as i32, size.height as i32);
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    // Clear the screen to black
                    gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);

                    // Draw a triangle from the 3 vertices
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        ELEMENT_DATA.len() as i32,
                        gl::UNSIGNED_INT,
                        0 as *const c_void,
                    );
                }
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
