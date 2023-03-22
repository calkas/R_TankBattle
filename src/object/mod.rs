use piston_window::{math, G2d};
pub mod bullet;
pub mod map;
pub mod tank;

pub trait Renderable {
    fn render(&self, view: math::Matrix2d, g: &mut G2d);
}
