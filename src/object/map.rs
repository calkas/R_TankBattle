
use super::Entity;
use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, Transformed};

pub struct GameMap {
    pos_x: f64,
    pos_y: f64,
    sprite: Option<Texture<Resources>>,

}


impl Entity for GameMap {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        image(
            self.sprite.as_ref().unwrap(),
            view.trans(self.pos_x, self.pos_y).scale(1.0, 1.0),
            g,
        );
    }
}


impl GameMap {
    pub fn new() -> Self {
        GameMap{pos_x: 0.0, pos_y: 0.0, sprite : None}
    }

    pub fn set_sprite(&mut self, texture: Texture<Resources>) {
        self.sprite = Some(texture);
    }
}