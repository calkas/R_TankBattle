use super::{Object, Renderable};
use gfx_device_gl::Resources;
use piston_window::{math, G2d, Texture};

pub struct ShootingTarget<'a> {
    pub target: Object<'a>,
}

impl Renderable for ShootingTarget<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        self.target.render(view, g);
    }
}

impl<'a> ShootingTarget<'a> {
    pub fn new(x: f64, y: f64, texture: &'a Texture<Resources>) -> Self {
        Self {
            target: Object {
                x,
                y,
                scale: 0.5,
                rotation: 0.0,
                velocity: 1.0,
                sprite: Some(texture),
            },
        }
    }
}