use crate::piston_window::Transformed;
use gfx_device_gl::Resources;
use piston_window::types::Color;
use piston_window::{image, math, rectangle, G2d, Texture};
use std::f64::consts::PI;

use super::Entity;

pub struct Tank {
    pos_x: f64,
    pos_y: f64,
    turret_rottation: f64,
    tank_sprite: Option<Texture<Resources>>,
    tank_turret_sprite: Option<Texture<Resources>>,
}

impl Entity for Tank {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        if self.tank_sprite.is_some() && self.tank_turret_sprite.is_some() {
            //This is half of the turret sprite size
            let center_for_rotate = (-16.0, -16.0);

            image(
                self.tank_sprite.as_ref().unwrap(),
                view.trans(self.pos_x, self.pos_y)
                    .scale(2.0, 2.0)
                    .trans(center_for_rotate.0, center_for_rotate.1),
                g,
            );

            image(
                self.tank_turret_sprite.as_ref().unwrap(),
                view.trans(0.0, -2.0)
                    .trans(self.pos_x, self.pos_y)
                    .scale(2.0, 2.0)
                    .rot_rad(self.turret_rottation)
                    .trans(center_for_rotate.0, center_for_rotate.1),
                g,
            );
        } else {
            let square = rectangle::square(0.0, 0.0, 50.0);
            let block_color: Color = [0.90, 0.49, 0.13, 1.0];
            rectangle(block_color, square, view.trans(self.pos_x, self.pos_y), g);
        }
    }
}

impl Tank {
    pub fn new() -> Self {
        Tank {
            pos_x: 0.0,
            pos_y: 0.0,
            turret_rottation: 0.0,
            tank_sprite: None,
            tank_turret_sprite: None,
        }
    }
    pub fn mov(&mut self, x: f64, y: f64) {
        self.pos_x += x;
        self.pos_y += y;
    }

    pub fn set_tank_sprite(&mut self, sprite: Texture<Resources>) {
        self.tank_sprite = Some(sprite);
    }

    pub fn set_turret_sprite(&mut self, sprite: Texture<Resources>) {
        self.tank_turret_sprite = Some(sprite);
    }

    pub fn rottate_turret_left(&mut self, dt: f64) {
        let rot_speed = 1.0;
        self.turret_rottation -= rot_speed * dt;
        if self.turret_rottation <= 0.0 {
            self.turret_rottation = 2.0 * PI;
        }
    }

    pub fn rottate_turret_right(&mut self, dt: f64) {
        let rot_speed = 1.0;
        self.turret_rottation += rot_speed * dt;

        if self.turret_rottation >= 2.0 * PI {
            self.turret_rottation = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_move_to() {
        let mut tank = Tank::new();
        tank.mov(10.0, 15.0);
        assert_eq!(tank.pos_x, 10.0);
        assert_eq!(tank.pos_y, 15.0);
    }
}
