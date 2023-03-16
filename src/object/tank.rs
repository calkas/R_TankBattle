use crate::piston_window::Transformed;
use gfx_device_gl::Resources;
use piston_window::types::Color;
use piston_window::{image, math, rectangle, G2d, Texture};
use std::f64::consts::PI;

use super::Entity;

pub struct Tank {
    pub tank_angle_rotation: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub turret_radian_rotation: f64,
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
                    .rot_deg(self.tank_angle_rotation)
                    .trans(center_for_rotate.0, center_for_rotate.1),
                g,
            );

            image(
                self.tank_turret_sprite.as_ref().unwrap(),
                view.trans(0.0, -2.0)
                    .trans(self.pos_x, self.pos_y)
                    .scale(2.0, 2.0)
                    .rot_deg(self.tank_angle_rotation)
                    .rot_rad(self.turret_radian_rotation)
                    .trans(center_for_rotate.0, center_for_rotate.1),
                g,
            );
            let square = rectangle::square(0.0, 0.0, 5.0);
            let block_color: Color = [0.90, 0.49, 0.13, 1.0];
            rectangle(block_color, square, view.trans(self.pos_x, self.pos_y).trans(0.0, -20.0), g);
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
            tank_angle_rotation: 0.0,
            pos_x: 0.0,
            pos_y: 0.0,
            turret_radian_rotation: 0.0,
            tank_sprite: None,
            tank_turret_sprite: None,
        }
    }
    pub fn mov(&mut self, x: f64, y: f64) {
        self.set_tank_direction(x,y);
        self.pos_x += x;
        self.pos_y += y;
    }

    pub fn set_tank_sprite(&mut self, sprite: Texture<Resources>) {
        self.tank_sprite = Some(sprite);
    }

    pub fn set_turret_sprite(&mut self, sprite: Texture<Resources>) {
        self.tank_turret_sprite = Some(sprite);
    }

    pub fn rotate_turret_left(&mut self, dt: f64) {
        let rot_speed = 1.0;
        self.turret_radian_rotation -= rot_speed * dt;
        if self.turret_radian_rotation <= 0.0 {
            self.turret_radian_rotation = 2.0 * PI;
        }
    }

    pub fn rotate_turret_right(&mut self, dt: f64) {
        let rot_speed = 1.0;
        self.turret_radian_rotation += rot_speed * dt;

        if self.turret_radian_rotation >= 2.0 * PI {
            self.turret_radian_rotation = 0.0;
        }
    }

    fn set_tank_direction(&mut self, x:f64, y:f64) {
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

    pub fn get_turret_angle(&self) -> f64 {
        self.turret_radian_rotation * 180.0 / PI
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
