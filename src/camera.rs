use glam::{Mat4, Vec3};
use glutin::event::VirtualKeyCode;

use crate::key::KeyboardState;

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,

    pitch: f64,
    yaw: f64,
    // roll: f64,
}

impl Camera {
    const SENSIVITY: f64 = 0.1;
    const SPEED: f32 = 0.01;
    pub fn new() -> Self {
        Self {
            position: Vec3 {
                x: 0.,
                y: 0.,
                z: 5.,
            },
            direction: Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            },
            up: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            pitch: 0.,
            yaw: -90.,
            // roll: 0.,
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.direction, self.up)
    }

    pub fn update(&mut self, delta: &(f64, f64)) {
        let (x, y) = delta;
        self.yaw += x * Camera::SENSIVITY;
        self.pitch -= y * Camera::SENSIVITY;

        if self.pitch > 89. {
            self.pitch = 89.;
        }
        if self.pitch < -89. {
            self.pitch = -89.;
        }
        self.direction.x = (self.yaw.to_radians().cos() * self.pitch.to_radians().cos()) as f32;
        self.direction.y = self.pitch.to_radians().sin() as f32;
        self.direction.z = (self.yaw.to_radians().sin() * self.pitch.to_radians().cos()) as f32;
    }

    pub fn handle_input(&mut self, key_state: &KeyboardState) {
        if key_state.is_pressed(&VirtualKeyCode::W) {
            self.position += self.direction * Camera::SPEED;
        }
        if key_state.is_pressed(&VirtualKeyCode::A) {
            self.position -= self.direction.cross(self.up).normalize() * Camera::SPEED;
        }
        if key_state.is_pressed(&VirtualKeyCode::S) {
            self.position -= self.direction * Camera::SPEED;
        }
        if key_state.is_pressed(&VirtualKeyCode::D) {
            self.position += self.direction.cross(self.up).normalize() * Camera::SPEED;
        }
    }
}
