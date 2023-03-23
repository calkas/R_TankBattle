use super::Renderable;
use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, Transformed};

pub struct Bullet<'a> {
    pub pos_x: f64,
    pub pos_y: f64,
    pub rotation: f64,
    pub to_destroy: bool,
    sprite: &'a Texture<Resources>,
}

impl Renderable for Bullet<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        image(
            self.sprite,
            view.trans(self.pos_x, self.pos_y).trans(-8.0, -5.0),
            g,
        );
    }
}

impl<'a> Bullet<'a> {
    pub fn new(x: f64, y: f64, rot: f64, texture: &'a Texture<Resources>) -> Self {
        Bullet {
            pos_x: x,
            pos_y: y,
            to_destroy: false,
            rotation: rot,
            sprite: texture,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let velocity = 170.0;
        self.pos_x += velocity * self.rotation.cos() * dt;
        self.pos_y += velocity * -self.rotation.sin() * dt;
    }
}