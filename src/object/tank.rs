use crate::piston_window::Transformed;
use gfx_device_gl::Resources;
use piston_window::types::Color;
use piston_window::{image, math, rectangle, G2d, Texture};

use super::Entity;

pub struct Tank {
    pos_x: f64,
    pos_y: f64,
    tank_sprite: Option<Texture<Resources>>,
    tank_turret_sprite: Option<Texture<Resources>>,
}

impl Entity for Tank {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        if self.tank_sprite.is_some() {
            image(
                self.tank_sprite.as_ref().unwrap(),
                view.trans(self.pos_x, self.pos_y).scale(2.0, 2.0),
                g,
            );

            if self.tank_turret_sprite.is_some() {
                image(
                    self.tank_turret_sprite.as_ref().unwrap(),
                    view.trans(0.0, -2.0)
                        .trans(self.pos_x, self.pos_y)
                        .scale(2.0, 2.0),
                    g,
                );
            }
        } else {
            let square = rectangle::square(0.0, 0.0, 50.0);
            const BLOCK_COLOR: Color = [0.90, 0.49, 0.13, 1.0];
            rectangle(BLOCK_COLOR, square, view.trans(self.pos_x, self.pos_y), g);
        }
    }
}

impl Tank {
    pub fn new() -> Self {
        Tank {
            pos_x: 0.0,
            pos_y: 0.0,
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
