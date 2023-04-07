use crate::object::bullet::Bullet;
use crate::object::map::GameMap;
use crate::object::tank::Tank;
use crate::object::target::ShootingTarget;
use crate::object::Renderable;
use piston_window::{clear, Button, Context, G2d, Glyphs, Key, Text, Transformed};
use rand::Rng;

use super::collider::intersection;
use super::settings::KeyStatus;
use super::{resource, settings};
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
pub struct Game<'a> {
    player: Tank<'a>,
    bullets: Vec<Bullet<'a>>,
    targets: Vec<ShootingTarget<'a>>,
    map: GameMap<'a>,
    ready_for_fire: bool,
    is_player_moving: bool,
    controller: Control,
    resource_manager: &'a resource::Manager,
    score: u32,
    time: f64,
}

impl<'a> Game<'a> {
    pub fn new(res: &'a resource::Manager) -> Self {
        let player = Tank::new(
            res.get_texture("tank").unwrap(),
            res.get_texture("turret").unwrap(),
        );
        let map = GameMap::new(res.get_texture("map1").unwrap());

        return Game {
            player,
            bullets: Vec::new(),
            targets: Vec::new(),
            map,
            ready_for_fire: false,
            is_player_moving: false,
            controller: Control::new(),
            resource_manager: res,
            score: 0,
            time: 0.0,
        };
    }

    pub fn render(&self, c: &Context, g: &mut G2d, glyph: &mut Glyphs) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        self.map.render(c.transform, g);

        let center = c
            .transform
            .trans(settings::RESOLUTION[0] / 2.0, settings::RESOLUTION[1] / 2.0);

        let score_str = format!("Score: {}             Time: {:.1}", self.score, self.time);

        Text::new_color([255.0, 0.0, 0.0, 1.0], 24)
            .draw(
                score_str.as_str(),
                glyph,
                &c.draw_state,
                c.transform.trans(20.0, 50.0),
                g,
            )
            .unwrap();

        for bullet in self.bullets.iter() {
            bullet.render(center, g);
        }

        for target in self.targets.iter() {
            target.render(center, g);
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
        self.time += delta_time;
        self.tank_control_handling(delta_time);

        self.turret_control_handling(delta_time);

        self.shooting_target_handling();

        self.bullet_control_handling(delta_time);
    }

    fn shooting_target_handling(&mut self) {
        if self.targets.len() == 0 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(
                ((-settings::RESOLUTION[0] / 2.0) + 40.0)..((settings::RESOLUTION[0] / 2.0) - 40.0),
            );
            let y = rng.gen_range(
                ((-settings::RESOLUTION[1] / 2.0) + 40.0)..((settings::RESOLUTION[1] / 2.0) - 40.0),
            );
            let target =
                ShootingTarget::new(x, y, self.resource_manager.get_texture("target").unwrap());
            self.targets.push(target);
        }

        //Remove target
        self.targets.retain(|target| {
            for bullet in self.bullets.iter_mut() {
                if intersection::circle::collision(&target.target, &bullet.object) {
                    bullet.to_destroy = true;
                    self.score += 10;
                    return false;
                }
            }
            if intersection::rectangle::collision(&target.target, &self.player.hull) {
                self.score += 1;
                return false;
            }
            return true;
        });
    }

    fn tank_control_handling(&mut self, delta_time: f64) {
        if self.controller.up == KeyStatus::Pressed {
            self.player
                .mov(0.0, -self.player.hull.velocity * delta_time);
            self.is_player_moving = true;
        } else if self.controller.down == KeyStatus::Pressed {
            self.player.mov(0.0, self.player.hull.velocity * delta_time);
            self.is_player_moving = true;
        } else if self.controller.left == KeyStatus::Pressed {
            self.player
                .mov(-self.player.hull.velocity * delta_time, 0.0);
            self.is_player_moving = true;
        } else if self.controller.right == KeyStatus::Pressed {
            self.player.mov(self.player.hull.velocity * delta_time, 0.0);
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
                    self.player.hull.x,
                    self.player.hull.y,
                    self.resource_manager.get_texture("bullet").unwrap(),
                );
                bullet.calculate_rotation(self.player.turret.rotation);

                self.bullets.push(bullet);
                self.ready_for_fire = false;
            }
        } else {
            if self.ready_for_fire == false {
                self.ready_for_fire = true;
            }
        }

        //Remove bullet if hit the target
        if self.bullets.len() != 0 {
            let bullet_id = self
                .bullets
                .iter()
                .position(|bullet| bullet.to_destroy == true);
            if bullet_id.is_some() {
                self.bullets.remove(bullet_id.unwrap());
            }
        }

        //Update the position
        for bullet in self.bullets.iter_mut() {
            bullet.update(delta_time);
        }

        //Remove bullets out of map
        self.bullets.retain(|bullet| {
            bullet.object.x < settings::RESOLUTION[0] / 2.0
                && bullet.object.x > -settings::RESOLUTION[0] / 2.0
                && bullet.object.y < settings::RESOLUTION[1] / 2.0
                && bullet.object.y > -settings::RESOLUTION[1] / 2.0
        });
    }
}
