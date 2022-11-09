use std::ffi::CString;

use camera::Camera;
use glam::Mat4;
use key::KeyboardState;
use mesh::Mesh;
use transform::Transform;

mod camera;
mod key;
mod mesh;
mod shader;
mod transform;

use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;

use crate::shader::ShaderProgram;

struct Window {
    width: f32,
    height: f32,
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let window_builder = glutin::window::WindowBuilder::new().with_title("hello opengl with rust");
    // It is essential to make the context current before calling `gl::load_with`.
    let gl_context = unsafe {
        glutin::ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .expect("Cannot create windowed context")
            .make_current()
            .expect("Failed to make context current")
    };
    let window = gl_context.window();
    let mut w = Window {
        width: 800.,
        height: 600.,
    };

    window.set_cursor_visible(false);
    // window
    //     .set_cursor_grab(glutin::window::CursorGrabMode::Confined)
    //     .unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|ptr| gl_context.get_proc_address(ptr));

    let version = unsafe {
        let tmp = gl::GetString(gl::VERSION) as *mut i8;
        CString::from_raw(tmp)
    };

    println!("{:?}", version.to_str().unwrap());

    // Create GLSL shaders and use shader program
    let mut shader = ShaderProgram::new(
        include_str!("../resources/vertex.glsl"),
        include_str!("../resources/fragment.glsl"),
    );
    shader.activate();

    let mesh = Mesh::cube();
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

        let projection = Mat4::perspective_rh(
            (45 as f32).to_radians() as f32,
            w.width / w.height,
            0.1,
            100.,
        );
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
                } => key_state.process_event(&state, &virtual_code),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => unsafe {
                    w.width = size.width as f32;
                    w.height = size.height as f32;
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
