use piston_window::{Context,G2d};


pub trait Drawable {
    fn draw(&self, c: &Context,  g: &mut G2d);
}