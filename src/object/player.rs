use crate::piston_window::Transformed;
use piston_window::rectangle;
use piston_window::types::Color;
use piston_window::{Context, G2d};
const TANK_COLOR: Color = [0.90, 0.49, 0.13, 1.0];

pub trait Drawable {
    fn draw(&self, c: &Context, g: &mut G2d);
}
pub struct Tank {
    start_pos_x: f64,
    start_pos_y: f64,
    pos_x: f64,
    pos_y: f64,
    height: f64,
    width: f64,
}

impl Drawable for Tank {
    fn draw(&self, c: &Context, g: &mut G2d) {
        //let square = rectangle::square(0.0, 0.0, 100.0);
        rectangle(
            TANK_COLOR,
            [self.start_pos_x, self.start_pos_y, self.height, self.width],
            c.transform.trans(self.pos_x, self.pos_y),
            g,
        );
    }
}

impl Tank {
    pub fn new(x: f64, y: f64) -> Self {
        Tank {
            start_pos_x: x,
            start_pos_y: y,
            pos_x: 0.0,
            pos_y: 0.0,
            height: 20.0,
            width: 20.0,
        }
    }
    pub fn mov(&mut self, x: f64, y: f64) {
        self.pos_x += x;
        self.pos_y += y;
    }
}
