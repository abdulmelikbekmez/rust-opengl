use camera::Camera;
use gl::types::*;
use glam::Mat4;
use key::KeyboardState;
use mesh::{Mesh, Vertex};
use transform::Transform;

mod camera;
mod key;
mod mesh;
mod shader;
mod transform;

use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;

use crate::shader::ShaderProgram;

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

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let window = glutin::window::WindowBuilder::new().with_title("hello opengl with rust");
    // It is essential to make the context current before calling `gl::load_with`.
    let gl_context = unsafe {
        glutin::ContextBuilder::new()
            .build_windowed(window, &event_loop)
            .expect("Cannot create windowed context")
            .make_current()
            .expect("Failed to make context current")
    };

    gl_context.window().set_cursor_visible(false);
    gl_context
        .window()
        .set_cursor_grab(glutin::window::CursorGrabMode::Confined)
        .unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|ptr| gl_context.get_proc_address(ptr));

    // Create GLSL shaders and use shader program
    let shader = ShaderProgram::new("resources/vertex.glsl", "resources/fragment.glsl");
    shader.activate();

    let mesh = Mesh::new(&ELEMENT_DATA, &VERTICES);
    mesh.bind();

    unsafe {
        gl::ClearColor(0., 0., 0., 1.);
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut key_state = KeyboardState::new();

    let transform = Transform::new();
    let mut camera = Camera::new();

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
                glutin::event::DeviceEvent::MouseMotion { delta } => camera.update(&delta),
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
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => unsafe {
                    gl_context.resize(size);
                    gl::Viewport(0, 0, size.width as i32, size.height as i32);
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    // Clear the screen to black
                    gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);

                    mesh.draw();
                }
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
