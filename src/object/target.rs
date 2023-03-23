use super::Renderable;
use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, Transformed};

pub struct Target<'a> {
    pub pos_x: f64,
    pub pos_y: f64,
    pub sprite: &'a Texture<Resources>,
}

impl Renderable for Target<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        image(
            self.sprite,
            view.trans(self.pos_x, self.pos_y).scale(0.5, 0.5),
            g,
        );
    }
}

impl<'a> Target<'a> {
    pub fn new(x: f64, y: f64, texture: &'a Texture<Resources>) -> Self {
        Target {
            pos_x: x,
            pos_y: y,
            sprite: &texture,
        }
    }

    pub fn is_collision(
        &self,
        object_pos_x: f64,
        object_pos_y: f64,
        object_w: f64,
        object_h: f64,
    ) -> bool {
        let target_w = 40.0;
        let target_h = 40.0;

        let collision_x =
            self.pos_x + target_w >= object_pos_x && object_pos_x + object_w >= self.pos_x;

        let collision_y =
            self.pos_y + target_h >= object_pos_y && object_pos_y + object_h >= self.pos_y;
        return collision_x && collision_y;
    }
}
