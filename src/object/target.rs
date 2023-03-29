use super::Renderable;
use gfx_device_gl::Resources;
use piston_window::{image, math, G2d, Texture, Transformed};

pub struct ShootingTarget<'a> {
    pub pos_x: f64,
    pub pos_y: f64,
    pub sprite: &'a Texture<Resources>,
}

impl Renderable for ShootingTarget<'_> {
    fn render(&self, view: math::Matrix2d, g: &mut G2d) {
        image(
            self.sprite,
            view.trans(self.pos_x, self.pos_y).scale(0.5, 0.5),
            g,
        );
    }
}

impl<'a> ShootingTarget<'a> {
    pub fn new(x: f64, y: f64, texture: &'a Texture<Resources>) -> Self {
        ShootingTarget {
            pos_x: x,
            pos_y: y,
            sprite: &texture,
        }
    }

    pub fn collide_circle_with(
        &self,
        object_pos_x: f64,
        object_pos_y: f64,
        object_w: f64,
        _object_h: f64,
    ) -> bool {
        let dimension = self.sprite.surface.get_info().kind.get_dimensions();

        //This is square

        let target_radius = (dimension.0 as f64 / 2.0) * 0.5; //scale;

        let target_center: (f64, f64) = (self.pos_x + target_radius, self.pos_y + target_radius);

        let object_radius = object_w / 2.0;
        let object_center: (f64, f64) = (object_pos_x + object_radius, object_pos_y + object_radius);

        let dx = target_center.0 - object_center.0;
        let dy = target_center.1 - object_center.1;

        let d = dx * dx + dy * dy;

        let distance = d.sqrt();

        let is_collision = distance < target_radius + object_radius;
       
        return is_collision;

    }

    pub fn collide_with(
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

    #[allow(dead_code)]
    pub fn set_sprite(&mut self, texture: &'a Texture<Resources>) {
        self.sprite = texture;
    }
}
