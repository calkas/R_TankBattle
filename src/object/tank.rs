use crate::piston_window::Transformed;
use gfx_device_gl::Resources;
use piston_window::types::Color;
use piston_window::{image, rectangle, Context, G2d, Texture};

pub trait Drawable {
    fn draw(&self, c: &Context, g: &mut G2d);
}
pub struct Tank {
    start_pos_x: f64,
    start_pos_y: f64,
    pos_x: f64,
    pos_y: f64,
    tank_sprite: Option<Texture<Resources>>,
    tank_turret_sprite: Option<Texture<Resources>>,
}

impl Drawable for Tank {
    fn draw(&self, c: &Context, g: &mut G2d) {
        if self.tank_sprite.is_some() {
            image(
                self.tank_sprite.as_ref().unwrap(),
                c.transform
                    .trans(self.start_pos_x, self.start_pos_y)
                    .trans(self.pos_x, self.pos_y)
                    .scale(2.0, 2.0),
                g,
            );

            if self.tank_turret_sprite.is_some() {
                image(
                    self.tank_turret_sprite.as_ref().unwrap(),
                    c.transform
                        .trans(self.start_pos_x, self.start_pos_y)
                        .trans(0.0, -2.0)
                        .trans(self.pos_x, self.pos_y)
                        .scale(2.0, 2.0),
                    g,
                );
            }
        } else {
            let square = rectangle::square(self.start_pos_x, self.start_pos_y, 50.0);
            const BLOCK_COLOR: Color = [0.90, 0.49, 0.13, 1.0];
            rectangle(
                BLOCK_COLOR,
                square,
                c.transform.trans(self.pos_x, self.pos_y),
                g,
            );
        }
    }
}

impl Tank {
    pub fn new(x: f64, y: f64) -> Self {
        Tank {
            start_pos_x: x,
            start_pos_y: y,
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
        let mut tank = Tank::new(0.0, 0.0);
        tank.mov(10.0, 15.0);
        assert_eq!(tank.pos_x, 10.0);
        assert_eq!(tank.pos_y, 15.0);
    }
    #[test]
    fn should_have_proper_start_pos() {
        let mut tank = Tank::new(100.0, 200.0);
        assert_eq!(tank.start_pos_x, 100.0);
        assert_eq!(tank.start_pos_y, 200.0);
    }
}
