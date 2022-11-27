use application::Application;
use application::*;
use key::KeyboardState;
use renderer::Renderer;
use scene::Scene;

mod application;
mod camera;
mod entity;
mod key;
mod renderer;
mod scene;
mod transform;

struct MyApp {
    renderer: Renderer,
    scene: Scene,
}
impl Application for MyApp {
    fn new() -> Self {
        let renderer = Renderer::cube();
        let mut scene = Scene::new();
        // scene.add_static_entities(100);
        scene.add_dynamic_entities(50);
        Self { renderer, scene }
    }

    fn update(&mut self, key_state: &KeyboardState, _: &Window) {
        self.scene.get_camera().handle_input(key_state);
        self.scene.update();
    }

    fn draw(&mut self, window: &Window) {
        self.renderer.draw(&mut self.scene, window);
    }

    fn event(&mut self) {}

    fn on_mouse_move(&mut self, delta: &(f64, f64)) {
        self.scene.get_camera().update(delta);
    }
}

fn main() {
    let w = Window {
        width: 800.,
        height: 600.,
    };
    AppBuilder::build::<MyApp>(w);
}
