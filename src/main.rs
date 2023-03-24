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

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! glcall {
    ($a: expr) => {
        loop {
            let err = gl::GetError();
            if err == gl::NO_ERROR {
                break;
            }
            eprintln!("GL ERROR: {} on calling this: {}", err, stringify!($a));
        }
        $a
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! glcall {
    ($a: expr) => {
        $a
    };
}

struct MyApp {
    renderer: Renderer,
    scene: Scene,
}

impl Application for MyApp {
    fn new(window: &Window) -> Self {
        let renderer = Renderer::cube(window);
        let mut scene = Scene::new();
        scene.add_entities(100, true);
        Self { renderer, scene }
    }

    #[inline]
    fn update(&mut self, key_state: &KeyboardState) {
        self.scene.get_mut_camera().handle_input(key_state);
        self.scene.update();
    }

    #[inline]
    fn draw(&mut self) {
        self.renderer.draw(&mut self.scene);
    }

    #[inline]
    fn event(&mut self) {}

    #[inline]
    fn on_mouse_move(&mut self, delta: &(f64, f64)) {
        self.scene.get_mut_camera().update(delta)
    }
    #[inline]
    fn on_resize(&mut self, window: &Window) {
        self.renderer.on_resize(window);
    }
}

fn main() {
    AppBuilder::build::<MyApp>(Window {
        width: 1400.,
        height: 900.,
    });
}
