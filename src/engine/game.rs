use crate::object::player::{self, Drawable};
use piston_window::{Button, Context, G2d, Key};

pub mod settings {
    pub const RESOLUTION: [f64; 2] = [600.0, 400.0];
    pub const TITLE: &str = "R_TankBattle";
    #[derive(PartialEq, Eq)]
    pub enum KeyStatus {
        Pressed,
        Released,
    }
}

use settings::KeyStatus;
struct Direction {
    up: KeyStatus,
    down: KeyStatus,
    left: KeyStatus,
    right: KeyStatus,
}
impl Direction {
    pub fn new() -> Self {
        Direction {
            up: KeyStatus::Released,
            down: KeyStatus::Released,
            left: KeyStatus::Released,
            right: KeyStatus::Released,
        }
    }
}
pub struct Game {
    tank: player::Tank,
    direction: Direction,
}

impl Game {
    pub fn new() -> Self {
        Game {
            tank: player::Tank::new(settings::RESOLUTION[0] / 2.0, settings::RESOLUTION[1] / 2.0),
            direction: Direction::new(),
        }
    }

    pub fn render(&self, c: &Context, g: &mut G2d) {
        self.tank.draw(&c, g);
    }

    pub fn input(&mut self, input: Button, keystatus: KeyStatus) {
        match input {
            Button::Keyboard(Key::Up) => self.direction.up = keystatus,
            Button::Keyboard(Key::Down) => self.direction.down = keystatus,
            Button::Keyboard(Key::Left) => self.direction.left = keystatus,
            Button::Keyboard(Key::Right) => self.direction.right = keystatus,
            _ => {}
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        if self.direction.up == KeyStatus::Pressed {
            self.tank.mov(0.0, -150.0 * delta_time);
        }

        if self.direction.down == KeyStatus::Pressed {
            self.tank.mov(0.0, 150.0 * delta_time);
        }

        if self.direction.left == KeyStatus::Pressed {
            self.tank.mov(-150.0 * delta_time, 0.0);
        }

        if self.direction.right == KeyStatus::Pressed {
            self.tank.mov(150.0 * delta_time, 0.0);
        }
    }
}
