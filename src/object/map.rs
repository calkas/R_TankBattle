use super::Object;
use crate::engine::settings;
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

    pub fn is_out_of_boundaries(&self, new_x: f64, new_y: f64, object: &Object) -> bool {
        let object_dim = object
            .sprite
            .unwrap()
            .surface
            .get_info()
            .kind
            .get_dimensions();

        let out_of_x = new_x + object_dim.0 as f64 > settings::RESOLUTION[0] / 2.0
            || new_x < -settings::RESOLUTION[0] / 2.0 + object_dim.0 as f64;
        let out_of_y = new_y + object_dim.1 as f64 > settings::RESOLUTION[1] / 2.0
            || new_y < -settings::RESOLUTION[1] / 2.0 + object_dim.1 as f64;

        return out_of_x || out_of_y;
    }
}
