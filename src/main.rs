use application::Application;
use application::*;
use camera::Camera;
use glutin::event::{MouseButton, MouseScrollDelta};
use key::KeyboardState;
use renderer::Renderer;
use scene::Scene;

mod application;
mod camera;
mod entity;
mod key;
mod renderer;
mod scene;
mod swarm;
mod transform;

struct MyApp {
    scene: Scene,
}
impl Application for MyApp {
    fn new() -> Self {
        Self {
            scene: Scene::new(),
        }
    }

    fn update(&mut self, key_state: &KeyboardState, window: &Window) {
        self.scene.update(key_state, window);
    }

    fn draw(&mut self, window: &Window, renderer: &mut Renderer) {
        self.scene.draw(window, renderer)
    }

    fn event(&mut self) {}

    fn on_mouse_move(&mut self, delta: &(f64, f64)) {
        self.scene.get_mut_camera().update(delta);
    }
    fn on_mouse_wheel(&mut self, delta: &MouseScrollDelta) {
        self.scene.on_mouse_wheel(delta);
    }

    fn on_mouse_click(&mut self, button: &MouseButton) {
        self.scene.on_mouse_click(button);
    }

    fn get_camera(&self) -> &Camera {
        self.scene.get_camera()
    }
}

// #[tokio::main]
fn main() {
    let w = Window {
        width: 1400.,
        height: 900.,
    };
    AppBuilder::build::<MyApp>(w);
}
