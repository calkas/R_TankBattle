use super::Object;
use gfx_device_gl::Resources;
use piston_window::{math, G2d, Texture};
use std::f64::consts::PI;

use super::Renderable;

pub struct Tank<'a> {
    pub hull: Object<'a>,
    pub turret: Object<'a>,
}

impl<'a> Tank<'a> {
    pub fn new(
        hull_texture: &'a Texture<Resources>,
        turret_texture: &'a Texture<Resources>,
    ) -> Self {
        Tank {
            hull: Object {
                x: 0.0,
                y: 0.0,
                scale: 2.0,
                rotation: 0.0,
                velocity: 150.0,
                sprite: Some(hull_texture),
            },
            turret: Object {
                x: 0.0,
                y: 0.0,
                scale: 2.0,
                rotation: 0.0,
                velocity: 1.0,
                sprite: Some(turret_texture),
            },
        }
    }

    pub fn mov(&mut self, x: f64, y: f64) {
        self.hull.mov(x, y);
        self.turret.mov(x, y);
        self.set_hull_direction(x, y);
    }

    pub fn rotate_turret_left(&mut self, dt: f64) {
        self.turret.rotate(-self.turret.velocity * dt);

        if self.turret.rotation <= 0.0 {
            self.turret.rotate_to(2.0 * PI);
        }
    }

    pub fn rotate_turret_right(&mut self, dt: f64) {
        self.turret.rotate(self.turret.velocity * dt);

        if self.turret.rotation >= 2.0 * PI {
            self.turret.rotate_to(0.0);
        }
    }

    fn set_hull_direction(&mut self, x: f64, y: f64) {
        if x == 0.0 && y.is_sign_negative() {
            self.hull.rotate_to(0.0);
        }

        if y == 0.0 && x.is_sign_positive() {
            self.hull.rotate_to(PI / 2.0);
        }

        if x == 0.0 && y.is_sign_positive() {
            self.hull.rotate_to(PI);
        }

        if y == 0.0 && x.is_sign_negative() {
            self.hull.rotate_to(PI + PI / 2.0);
        }
    }
}

impl Renderable for Tank<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        self.hull.render(view, g);
        self.turret.render(view, g);
    }
}
