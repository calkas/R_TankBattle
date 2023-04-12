use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, Transformed};
use std::default::Default;
pub mod bullet;
pub mod map;
pub mod tank;
pub mod target;
pub mod ui;

pub trait Renderable {
    fn render(&self, view: math::Matrix2d, g: &mut G2d);
}
pub struct Object<'a> {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub rotation: f64,
    pub velocity: f64,
    pub sprite: Option<&'a Texture<Resources>>,
}
impl Renderable for Object<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        if self.sprite.is_some() {
            let sprite_dimension = self
                .sprite
                .unwrap()
                .surface
                .get_info()
                .kind
                .get_dimensions();

            let center_axis_for_rotate = (
                (-0.5) * sprite_dimension.0 as f64,
                (-0.5) * sprite_dimension.1 as f64,
            );

            image(
                self.sprite.unwrap(),
                view.trans(self.x, self.y)
                    .scale(self.scale, self.scale)
                    .rot_rad(self.rotation)
                    .trans(center_axis_for_rotate.0, center_axis_for_rotate.1),
                g,
            );
        }
    }
}

impl Default for Object<'_> {
    #[allow(dead_code)]
    fn default() -> Self {
        Object {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            rotation: 0.0,
            velocity: 1.0,
            sprite: None,
        }
    }
}

impl<'a> Object<'a> {
    #[allow(dead_code)]
    pub fn new(
        x: f64,
        y: f64,
        scale: f64,
        rotation: f64,
        velocity: f64,
        texture: &'a Texture<Resources>,
    ) -> Self {
        Self {
            x,
            y,
            scale,
            rotation,
            velocity,
            sprite: Some(texture),
        }
    }
    #[allow(dead_code)]
    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    #[allow(dead_code)]
    pub fn mov_to(&mut self, pos_x: f64, pos_y: f64) {
        self.x = pos_x;
        self.y = pos_y;
    }

    #[allow(dead_code)]
    pub fn vector_rotation(&mut self, radian_angle: f64) {
        //x2=cosβx1−sinβy1
        //y2=sinβx1+cosβy1

        let new_x = radian_angle.cos() * self.x - radian_angle.sin() * self.y;
        let new_y = radian_angle.sin() * self.x + radian_angle.cos() * self.y;

        self.x = new_x;
        self.y = new_y;
    }

    #[allow(dead_code)]
    pub fn rotate(&mut self, radian_angle: f64) {
        self.rotation += radian_angle;
    }

    #[allow(dead_code)]
    pub fn rotate_to(&mut self, radian_angle: f64) {
        self.rotation = radian_angle;
    }

    #[allow(dead_code)]
    pub fn forward(&mut self, dt: f64) {
        self.x += self.velocity * self.rotation.cos() * dt;
        //y-axis is down
        self.y += self.velocity * -self.rotation.sin() * dt;
    }
}

#[cfg(test)]
mod unit_tests {

    use std::f64::consts::PI;

    use super::Object;

    #[test]
    fn should_move_to_position() {
        let mut game_object = Object::default();
        game_object.mov(100.0, 100.0);
        game_object.mov(25.0, 25.0);

        assert_eq!(125.0, game_object.x);
        assert_eq!(125.0, game_object.y);
    }

    #[test]
    fn should_switch_to_position() {
        let mut game_object = Object::default();
        game_object.mov(100.0, 100.0);
        game_object.mov_to(25.0, 25.0);

        assert_eq!(25.0, game_object.x);
        assert_eq!(25.0, game_object.y);
    }

    #[test]
    fn should_rotate_to_position() {
        let mut game_object = Object::default();
        game_object.rotate(1.5);
        game_object.rotate(1.5);

        assert_eq!(3.0, game_object.rotation);
    }

    #[test]
    fn should_switch_to_angle() {
        let mut game_object = Object::default();
        game_object.rotate(1.5);
        game_object.rotate_to(0.5);

        assert_eq!(0.5, game_object.rotation);
    }

    #[test]
    fn should_move_forward_with_angle() {
        let mut game_object = Object::default();

        game_object.velocity = 100.0;
        game_object.rotate(PI);
        game_object.forward(1.0);

        assert_eq!(game_object.x as i32, -100);
        assert_eq!(game_object.y as i32, 0)
    }
}
