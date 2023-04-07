pub mod intersection {
    pub mod rectangle {
        use crate::object::Object;
        pub fn collision(object_a: &Object, object_b: &Object) -> bool {
            let object_a_dim = object_a
                .sprite
                .unwrap()
                .surface
                .get_info()
                .kind
                .get_dimensions();
            let object_b_dim = object_b
                .sprite
                .unwrap()
                .surface
                .get_info()
                .kind
                .get_dimensions();

            let object_a_width = object_a_dim.0 as f64 * object_a.scale;
            let object_a_height = object_a_dim.1 as f64 * object_a.scale;

            let object_b_width = object_b_dim.0 as f64 * object_b.scale;
            let object_b_height = object_b_dim.1 as f64 * object_b.scale;

            let collision_x1 = object_a.x < object_b.x + object_b_width;
            let collision_x2 = object_a.x + object_a_width > object_b.x;

            let collision_y1 = object_a.y < object_b.y + object_b_height;
            let collision_y2 = object_a.y + object_a_height > object_b.y;

            return collision_x1 && collision_x2 && collision_y1 && collision_y2;
        }
    }

    pub mod circle {
        use crate::object::Object;
        pub fn collision(object_a: &Object, object_b: &Object) -> bool {
            let object_a_dim = object_a
                .sprite
                .unwrap()
                .surface
                .get_info()
                .kind
                .get_dimensions();
            let object_b_dim = object_b
                .sprite
                .unwrap()
                .surface
                .get_info()
                .kind
                .get_dimensions();

            //We have circle in the square texture
            //----
            //|()|
            //----

            let object_a_radius = (object_a_dim.0 as f64 / 2.0) * object_a.scale;
            let object_b_radius = (object_b_dim.0 as f64 / 2.0) * object_b.scale;

            let object_a_circle_center_position =
                (object_a.x + object_a_radius, object_a.y + object_a_radius);
            let object_b_circle_center_position =
                (object_b.x + object_b_radius, object_b.y + object_b_radius);

            let dx = object_a_circle_center_position.0 - object_b_circle_center_position.0;
            let dy = object_a_circle_center_position.1 - object_b_circle_center_position.1;

            let d = dx * dx + dy * dy;
            let distance = d.sqrt();

            return distance < object_a_radius + object_b_radius;
        }
    }
}
