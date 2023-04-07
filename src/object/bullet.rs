use std::f64::consts::PI;

use super::Object;
use super::Renderable;
use gfx_device_gl::Resources;
use piston_window::{math, G2d, Texture};

pub struct Bullet<'a> {
    pub object: Object<'a>,
    pub to_destroy: bool,
}

impl Renderable for Bullet<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        self.object.render(view, g);
    }
}

impl<'a> Bullet<'a> {
    pub fn new(x: f64, y: f64, texture: &'a Texture<Resources>) -> Self {
        Self {
            object: Object {
                x,
                y,
                scale: 1.0,
                rotation: 0.0,
                velocity: 170.0,
                sprite: Some(texture),
            },
            to_destroy: false,
        }
    }

    pub fn calculate_rotation(&mut self, turret_rotation: f64) {
        if 0.0 <= turret_rotation && turret_rotation <= PI / 2.0 {
            self.object.rotation = PI / 2.0 - turret_rotation;
        } else if PI / 2.0 < turret_rotation && turret_rotation <= PI {
            self.object.rotation = 2.0 * PI + PI / 2.0 - turret_rotation;
        } else if PI < turret_rotation && turret_rotation <= 3.0 * PI / 2.0 {
            self.object.rotation = PI / 2.0 + 2.0 * PI - turret_rotation;
        } else {
            self.object.rotation = 2.0 * PI + PI / 2.0 - turret_rotation;
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.object.forward(dt);
    }
}
