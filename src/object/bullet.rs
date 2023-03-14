use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, Transformed};
use super::Entity;


pub struct Bullet {
    pos_x: f64,
    pos_y: f64,
    sprite: Option<Texture<Resources>>
}

impl Entity for Bullet {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        image(
            self.sprite.as_ref().unwrap(),
            view.trans(self.pos_x, self.pos_y),
            g,
        );
    }
}

impl Bullet {
    pub fn new() -> Self {
            Bullet { pos_x: 0.0, pos_y: 0.0, sprite: None}
        }

    pub fn update(&mut self, dt: f64) {
        self.pos_x += 50.0 * dt;
    }

    pub fn set_sprite(&mut self, texture: Texture<Resources>) {
        self.sprite = Some(texture);
    }
}


