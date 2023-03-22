use super::Renderable;
use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, Transformed};

pub struct GameMap<'a> {
    pub pos_x: f64,
    pub pos_y: f64,
    pub sprite: &'a Texture<Resources>,
}

impl Renderable for GameMap<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        image(
            self.sprite,
            view.trans(self.pos_x, self.pos_y).scale(1.0, 1.0),
            g,
        );
    }
}

impl<'a> GameMap<'a> {
    pub fn new(texture: &'a Texture<Resources>) -> Self {
        GameMap {
            pos_x: 0.0,
            pos_y: 0.0,
            sprite: &texture,
        }
    }
}
