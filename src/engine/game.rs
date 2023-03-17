use crate::object::bullet::Bullet;
use crate::object::tank::Tank;
use crate::object::Entity;
use piston_window::{clear, Button, Context, Flip, G2d, Key, PistonWindow, Transformed};

use super::{resource, settings};
use super::settings::KeyStatus;
struct Control {
    up: KeyStatus,
    down: KeyStatus,
    left: KeyStatus,
    right: KeyStatus,
    turret_left: KeyStatus,
    turret_right: KeyStatus,
    fire: KeyStatus,
}
impl Control {
    pub fn new() -> Self {
        Control {
            up: KeyStatus::Released,
            down: KeyStatus::Released,
            left: KeyStatus::Released,
            right: KeyStatus::Released,
            turret_left: KeyStatus::Released,
            turret_right: KeyStatus::Released,
            fire: KeyStatus::Released,
        }
    }
}
pub struct Game {
    player: Tank,
    bullets: Vec<Bullet>,
    ready_for_fire: bool,
    is_player_moving: bool,
    controller: Control,
    resource_manager: resource::Manager,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Tank::new(),
            bullets: Vec::new(),
            ready_for_fire: false,
            is_player_moving: false,
            controller: Control::new(),
            resource_manager: resource::Manager::new(),
        }
    }

    pub fn load_resources(&mut self, window: &PistonWindow) {
        self.resource_manager
            .load_texture(window, "tank", "assets/tankBase.png", Flip::None);
        self.resource_manager
            .load_texture(window, "turret", "assets/tankTurret.png", Flip::None);
        self.resource_manager
            .load_texture(window, "bullet", "assets/bullet.png", Flip::None);

        self.player
            .set_tank_sprite(self.resource_manager.get_texture("tank").unwrap().clone());
        self.player
            .set_turret_sprite(self.resource_manager.get_texture("turret").unwrap().clone());
    }

    pub fn render(&self, c: &Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        let center = c
            .transform
            .trans(settings::RESOLUTION[0] / 2.0, settings::RESOLUTION[1] / 2.0);

        for bullet in self.bullets.iter() {
            bullet.render(center, g);
        }

        self.player.render(center, g);
    }

    pub fn input(&mut self, input: Button, keystatus: KeyStatus) {
        match input {
            Button::Keyboard(Key::Up) => self.controller.up = keystatus,
            Button::Keyboard(Key::Down) => self.controller.down = keystatus,
            Button::Keyboard(Key::Left) => self.controller.left = keystatus,
            Button::Keyboard(Key::Right) => self.controller.right = keystatus,
            Button::Keyboard(Key::S) => self.controller.turret_left = keystatus,
            Button::Keyboard(Key::D) => self.controller.turret_right = keystatus,
            Button::Keyboard(Key::Space) => self.controller.fire = keystatus,
            _ => {}
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.tank_control_handling(delta_time);

        self.turret_control_handling(delta_time);

        self.bullet_control_handling(delta_time);
    }

    fn tank_control_handling(&mut self, delta_time: f64) {
        if self.controller.up == KeyStatus::Pressed {
            self.player.mov(0.0, -150.0 * delta_time);
            self.is_player_moving = true;

        } else if self.controller.down == KeyStatus::Pressed {
            self.player.mov(0.0, 150.0 * delta_time);
            self.is_player_moving = true;

        } else if self.controller.left == KeyStatus::Pressed {
            self.player.mov(-150.0 * delta_time, 0.0);
            self.is_player_moving = true;

        } else if self.controller.right == KeyStatus::Pressed {
            self.player.mov(150.0 * delta_time, 0.0);
            self.is_player_moving = true;

        } else {
            self.is_player_moving = false;
        }
    }

    fn turret_control_handling(&mut self, delta_time: f64) {
        if self.controller.turret_left == KeyStatus::Pressed {
            self.player.rotate_turret_left(delta_time);
        } else if self.controller.turret_right == KeyStatus::Pressed {
            self.player.rotate_turret_right(delta_time);
        }
    }

    fn bullet_control_handling(&mut self, delta_time: f64) {
        if self.controller.fire == KeyStatus::Pressed {
            if self.ready_for_fire == true && self.is_player_moving == false {
                let mut bullet = Bullet::new(
                    self.player.pos_x,
                    self.player.pos_y,
                    self.player.get_turret_radians(),
                );
                bullet.set_sprite(self.resource_manager.get_texture("bullet").unwrap().clone());
                self.bullets.push(bullet);
                self.ready_for_fire = false;
            }
        } else {
            if self.ready_for_fire == false {
                self.ready_for_fire = true;
            }
        }

        for bullet in self.bullets.iter_mut() {
            bullet.update(delta_time);
        }

        //Remove bullets out of map
        self.bullets.retain(|bullet| {
            bullet.pos_x < settings::RESOLUTION[0] / 2.0
                && bullet.pos_x > -settings::RESOLUTION[0] / 2.0
                && bullet.pos_y < settings::RESOLUTION[1] / 2.0
                && bullet.pos_y > -settings::RESOLUTION[1] / 2.0
        });
    }
}
