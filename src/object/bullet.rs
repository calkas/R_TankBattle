use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, Transformed};
use super::Entity;
use std::f64::consts::PI;

pub struct Bullet {
    pub pos_x: f64,
    pub pos_y: f64,
    pub rotation: f64,
    sprite: Option<Texture<Resources>>
}

impl Entity for Bullet {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        image(
            self.sprite.as_ref().unwrap(),
            view.trans(self.pos_x, self.pos_y).trans(-8.0, 0.0),
            g,
        );
    }
}

impl Bullet {
    pub fn new(x:f64, y:f64, rot: f64) -> Self {
        Bullet { pos_x: x, pos_y: y, rotation: rot, sprite: None}
    }

    pub fn update(&mut self, dt: f64) {
        let velocity = 100.0;
        self.pos_x += velocity * self.rotation.cos() * dt;
        self.pos_y += velocity * -self.rotation.sin() * dt;
    }

    pub fn set_sprite(&mut self, texture: Texture<Resources>) {
        self.sprite = Some(texture);
    }
}


