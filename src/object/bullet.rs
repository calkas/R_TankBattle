use super::Object;
use super::Renderable;
use gfx_device_gl::Resources;
use piston_window::{math, G2d, Texture, Transformed};

pub struct Bullet<'a> {
    pub object: Object<'a>,
    pub to_destroy: bool,
}

impl Renderable for Bullet<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        view.trans(-8.0, -5.0);
        self.object.render(view, g);
    }
}

impl<'a> Bullet<'a> {
    pub fn new(x: f64, y: f64, rotation: f64, texture: &'a Texture<Resources>) -> Self {
        Self {
            object: Object {
                x,
                y,
                scale: 1.0,
                rotation,
                velocity: 170.0,
                sprite: Some(texture),
            },
            to_destroy: false,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.object.forward(dt);
    }
}
