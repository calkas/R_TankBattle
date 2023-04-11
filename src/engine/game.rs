use crate::object::bullet::Bullet;
use crate::object::map::GameMap;
use crate::object::tank::Tank;
use crate::object::target::ShootingTarget;
use crate::object::Renderable;
use piston_window::{clear, Button, Context, G2d, Glyphs, Text, Transformed};
use rand::Rng;

use super::collider::intersection;
use super::input::Controller;
use super::settings::KeyStatus;
use super::{resource, settings};

pub struct Game<'a> {
    player: Tank<'a>,
    bullets: Vec<Bullet<'a>>,
    targets: Vec<ShootingTarget<'a>>,
    map: GameMap<'a>,
    ready_for_fire: bool,
    is_player_moving: bool,
    controller: Controller,
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
            controller: Controller::default(),
            resource_manager: res,
            score: 0,
            time: 0.0,
        };
    }

    pub fn render(&self, c: &Context, g: &mut G2d, glyph: &mut Glyphs) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        let center = c
            .transform
            .trans(settings::RESOLUTION[0] / 2.0, settings::RESOLUTION[1] / 2.0);

        self.map.background.render(center, g);

        for bullet in self.bullets.iter() {
            bullet.object.render(center, g);
        }

        for target in self.targets.iter() {
            target.target.render(center, g);
        }

        self.player.render(center, g);

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
    }

    pub fn input(&mut self, input: Button, keystatus: KeyStatus) {
        self.controller.on_input(input, keystatus);
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
        if self.controller.is_up() {
            self.try_move_player(0.0, -self.player.hull.velocity * delta_time);
        } else if self.controller.is_down() {
            self.try_move_player(0.0, self.player.hull.velocity * delta_time);
        } else if self.controller.is_left() {
            self.try_move_player(-self.player.hull.velocity * delta_time, 0.0);
        } else if self.controller.is_right() {
            self.try_move_player(self.player.hull.velocity * delta_time, 0.0);
        } else {
            self.is_player_moving = false;
        }
    }

    fn try_move_player(&mut self, x: f64, y: f64) {
        if self.map.is_out_of_boundaries(
            self.player.hull.x + x,
            self.player.hull.y + y,
            &self.player.hull,
        ) {
            return;
        }
        self.player.mov(x, y);
        self.is_player_moving = true;
    }

    fn turret_control_handling(&mut self, delta_time: f64) {
        if self.controller.is_turret_left() {
            self.player.rotate_turret_left(delta_time);
        } else if self.controller.is_turret_right() {
            self.player.rotate_turret_right(delta_time);
        }
    }

    fn bullet_control_handling(&mut self, delta_time: f64) {
        if self.controller.is_fire() {
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
            if self
                .map
                .is_out_of_boundaries(bullet.object.x, bullet.object.y, &bullet.object)
            {
                return false;
            }
            return true;
        });
    }
}
