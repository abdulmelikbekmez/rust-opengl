use std::ffi::CString;

use glutin::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use crate::key::KeyboardState;

pub struct AppBuilder;

pub struct Window {
    pub width: f32,
    pub height: f32,
}

impl Window {
    #[inline]
    pub fn get_aspect_ratio(&self) -> f32 {
        self.width / self.height
    }
}

impl AppBuilder {
    pub fn build<T: Application + 'static>(mut w: Window) {
        let event_loop = glutin::event_loop::EventLoop::new();

        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("hello opengl with rust")
            .with_inner_size(LogicalSize::new(w.width, w.height));
        // It is essential to make the context current before calling `gl::load_with`.
        let gl_context = unsafe {
            glutin::ContextBuilder::new()
                .build_windowed(window_builder, &event_loop)
                .expect("Cannot create windowed context")
                .make_current()
                .expect("Failed to make context current")
        };

        gl_context.window().set_cursor_visible(false);
        // gl_context
        //     .window()
        //     .set_cursor_grab(CursorGrabMode::None)
        //     .unwrap();

        gl::load_with(|ptr| gl_context.get_proc_address(ptr));

        let version = unsafe {
            let tmp = gl::GetString(gl::VERSION) as *mut i8;
            CString::from_raw(tmp)
        };
        println!("{:?}", version.to_str().unwrap());

        let mut app = T::new(&w);
        let mut key_state = KeyboardState::new();

        unsafe {
            gl::ClearColor(0., 0., 0., 1.);
            gl::Enable(gl::DEPTH_TEST);
        }

        event_loop.run(move |event, _, control_flow| {
            // *control_flow = ControlFlow::Wait;
            if key_state.is_pressed(&VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
            }

            app.update(&key_state);

            gl_context.window().request_redraw();

            match event {
                Event::LoopDestroyed => return,
                Event::DeviceEvent { event, .. } => match event {
                    glutin::event::DeviceEvent::MouseMotion { delta } => app.on_mouse_move(&delta),
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
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
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
                        app.draw();
                    }
                    gl_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}

pub trait Application {
    fn new(window: &Window) -> Self;
    fn update(&mut self, key_state: &KeyboardState);
    fn draw(&mut self);
    fn event(&mut self);
    fn on_mouse_move(&mut self, delta: &(f64, f64));
    fn on_resize(&mut self, window: &Window);
}
