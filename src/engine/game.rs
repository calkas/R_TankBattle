use crate::object::bullet::Bullet;
use crate::object::map::GameMap;
use crate::object::tank::Tank;
use crate::object::target::ShootingTarget;
use crate::object::ui::UI;
use crate::object::Renderable;
use piston_window::{Button, Context, G2d, Glyphs, Transformed};
use rand::Rng;

use super::collider::intersection;
use super::input::Controller;
use super::settings::KeyStatus;
use super::stats::GameStatistics;
use super::{resource, settings};

pub struct Game<'a> {
    player: Tank<'a>,
    bullets: Vec<Bullet<'a>>,
    targets: Vec<ShootingTarget<'a>>,
    map: GameMap<'a>,
    controller: Controller,
    resource_manager: &'a resource::Manager,
    game_stats: GameStatistics,
    ui: UI<'a>,
}

impl<'a> Game<'a> {
    pub fn new(res: &'a resource::Manager) -> Self {
        let player = Tank::new(
            res.get_texture("tank").unwrap(),
            res.get_texture("turret").unwrap(),
        );
        let map = GameMap::new(res.get_texture("map1").unwrap());
        let ui = UI::new(
            res.get_texture("ui_score_board").unwrap(),
            res.get_texture("ui_gameover").unwrap(),
        );

        return Game {
            player,
            bullets: Vec::new(),
            targets: Vec::new(),
            map,
            controller: Controller::default(),
            resource_manager: res,
            game_stats: GameStatistics::new(),
            ui,
        };
    }

    pub fn render(&self, c: &Context, g: &mut G2d, glyph: &mut Glyphs) {
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
        self.ui.render(c, g, glyph);

        if self.game_stats.is_gameover {
            self.ui.game_over(c, g, glyph);
        }
    }

    pub fn input(&mut self, input: Button, keystatus: KeyStatus) {
        self.controller.on_input(input, keystatus);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.game_stats.time += delta_time;

        if self.game_stats.time > settings::GAME_TIME {
            self.game_stats.is_gameover = true;
        }

        if self.game_stats.is_gameover {
            if !self.controller.is_reset() {
                return;
            }
            self.bullets.clear();
            self.targets.clear();
            self.player.reset();
            self.game_stats.reset_game();
        }

        self.tank_control_handling(delta_time);

        self.turret_control_handling(delta_time);

        self.shooting_target_handling();

        self.bullet_control_handling(delta_time);

        self.ui
            .score_board_update(self.game_stats.score, self.game_stats.time);
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
                    self.game_stats.bonus_score_update();
                    return false;
                }
            }
            if intersection::rectangle::collision(&target.target, &self.player.hull) {
                self.game_stats.normal_score_update();
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
            self.game_stats.is_player_moving = false;
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
        self.game_stats.is_player_moving = true;
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
            if self.game_stats.ready_for_fire == true && self.game_stats.is_player_moving == false {
                let mut bullet = Bullet::new(
                    self.player.hull.x,
                    self.player.hull.y,
                    self.resource_manager.get_texture("bullet").unwrap(),
                );
                bullet.calculate_rotation(self.player.turret.rotation);

                self.bullets.push(bullet);
                self.game_stats.ready_for_fire = false;
            }
        } else {
            if self.game_stats.ready_for_fire == false {
                self.game_stats.ready_for_fire = true;
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
