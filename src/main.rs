use application::Application;
use application::*;
use camera::Camera;
use glam::Mat4;
use key::KeyboardState;
use mesh::Mesh;
use shader::*;
use transform::Transform;

mod application;
mod camera;
mod key;
mod mesh;
mod shader;
mod transform;

struct MyApp {
    mesh: Mesh,
    shader: ShaderProgram,
    camera: Camera,
    transform: Transform,
}
impl Application for MyApp {
    fn new() -> Self {
        let camera = Camera::new();
        let shader = ShaderProgram::new(
            include_str!("../resources/vertex.glsl"),
            include_str!("../resources/fragment.glsl"),
        );
        let mesh = Mesh::cube();
        mesh.bind();
        shader.activate();
        let transform = Transform::new();
        Self {
            mesh,
            shader,
            transform,
            camera,
        }
    }

    fn update(&mut self, key_state: &KeyboardState, window: &Window) {
        self.camera.handle_input(key_state);

        self.camera.handle_input(&key_state);

        let model = self.transform.get_matrix();
        self.shader.set_mat4("model", &model);

        let view = self.camera.get_matrix();
        self.shader.set_mat4("view", &view);

        let projection = Mat4::perspective_rh(
            (45 as f32).to_radians() as f32,
            window.width / window.height,
            0.1,
            100.,
        );
        self.shader.set_mat4("projection", &projection);
    }

    fn draw(&self) {
        self.mesh.draw();
    }

    fn event(&mut self) {}

    fn on_mouse_move(&mut self, delta: &(f64, f64)) {
        self.camera.update(delta);
    }
}

fn main() {
    let w = Window {
        width: 800.,
        height: 600.,
    };
    AppBuilder::build::<MyApp>(w);
}
