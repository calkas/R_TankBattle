use super::Object;
use gfx_device_gl::Resources;
use piston_window::Texture;

pub struct GameMap<'a> {
    pub background: Object<'a>,
}

impl<'a> GameMap<'a> {
    pub fn new(texture: &'a Texture<Resources>) -> Self {
        Self {
            background: Object {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
                velocity: 1.0,
                sprite: Some(texture),
            },
        }
    }
}
