use crate::piston_window::Transformed;
use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, rectangle, types::Color};
use std::f64::consts::PI;

use super::Renderable;

pub struct Tank<'a> {
    pub pos_x: f64,
    pub pos_y: f64,
    pub tank_angle_rotation: f64,
    pub turret_radian_rotation: f64,
    tank_sprite: &'a Texture<Resources>,
    tank_turret_sprite: &'a Texture<Resources>,
}

impl Renderable for Tank<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        //This is half of the turret sprite size

        let turret_dimension = self.tank_turret_sprite.surface.get_info().kind.get_dimensions();

        let center_for_rotate = ((-0.5) * turret_dimension.0 as f64, (-0.5) * turret_dimension.1 as f64);

        image(
            self.tank_sprite,
            view.trans(self.pos_x, self.pos_y)
                .scale(2.0, 2.0)
                .rot_deg(self.tank_angle_rotation)
                .trans(center_for_rotate.0, center_for_rotate.1),
            g,
        );

        image(
            self.tank_turret_sprite,
            view.trans(0.0, -2.0)
                .trans(self.pos_x, self.pos_y)
                .scale(2.0, 2.0)
                .rot_rad(-PI / 2.0)
                .flip_v()
                .rot_rad(self.turret_radian_rotation)
                .trans(center_for_rotate.0, center_for_rotate.1),
            g,
        );

        //let square = rectangle::square(0.0, 0.0, 64.0);
        //let block_color: Color = [0.90, 0.49, 0.13, 1.0];
        //rectangle(block_color, square, view.trans(self.pos_x, self.pos_y).trans(center_for_rotate.0, center_for_rotate.1).trans(-16.0 ,-16.0), g);
    }
}

impl<'a> Tank<'a> {
    pub fn new(
        tank_texture: &'a Texture<Resources>,
        turret_texture: &'a Texture<Resources>,
    ) -> Self {
        Tank {
            pos_x: 0.0,
            pos_y: 0.0,
            tank_angle_rotation: 0.0,
            turret_radian_rotation: PI / 2.0,
            tank_sprite: tank_texture,
            tank_turret_sprite: turret_texture,
        }
    }

    pub fn mov(&mut self, x: f64, y: f64) {
        self.set_tank_direction(x, y);
        self.pos_x += x;
        self.pos_y += y;
    }

    pub fn rotate_turret_left(&mut self, dt: f64) {
        let rot_speed = 1.0;
        self.turret_radian_rotation += rot_speed * dt;

        if self.turret_radian_rotation >= 2.0 * PI {
            self.turret_radian_rotation = 0.0;
        }
    }

    pub fn rotate_turret_right(&mut self, dt: f64) {
        let rot_speed = 1.5;
        self.turret_radian_rotation -= rot_speed * dt;

        if self.turret_radian_rotation <= 0.0 {
            self.turret_radian_rotation = 2.0 * PI;
        }
    }

    fn set_tank_direction(&mut self, x: f64, y: f64) {
        if x == 0.0 && y.is_sign_negative() {
            self.tank_angle_rotation = 0.0;
        }

        if y == 0.0 && x.is_sign_positive() {
            self.tank_angle_rotation = 90.0;
        }

        if x == 0.0 && y.is_sign_positive() {
            self.tank_angle_rotation = 180.0;
        }

        if y == 0.0 && x.is_sign_negative() {
            self.tank_angle_rotation = 270.0;
        }
    }
}