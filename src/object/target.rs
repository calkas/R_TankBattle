use super::Object;
use gfx_device_gl::Resources;
use piston_window::Texture;

pub struct ShootingTarget<'a> {
    pub target: Object<'a>,
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