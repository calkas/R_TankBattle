use crate::object::tank::Tank;
use crate::object::Entity;
use piston_window::{
    clear, Button, Context, Flip, G2d, Key, PistonWindow, Texture, TextureContext, TextureSettings,
    Transformed,
};

use std::path::Path;
pub mod settings {
    pub const RESOLUTION: [f64; 2] = [800.0, 600.0];
    pub const TITLE: &str = "R_TankBattle";
    #[derive(PartialEq, Eq)]
    pub enum KeyStatus {
        Pressed,
        Released,
    }
}

use settings::KeyStatus;
struct Control {
    up: KeyStatus,
    down: KeyStatus,
    left: KeyStatus,
    right: KeyStatus,
}
impl Control {
    pub fn new() -> Self {
        Control {
            up: KeyStatus::Released,
            down: KeyStatus::Released,
            left: KeyStatus::Released,
            right: KeyStatus::Released,
        }
    }
}
pub struct Game {
    player: Tank,
    direction: Control,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Tank::new(),
            direction: Control::new(),
        }
    }

    pub fn load_sprites(&mut self, window: &PistonWindow) {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.clone().create_command_buffer().into(),
        };

        let texture_settings = TextureSettings::new();

        let tank_sprite = Texture::from_path(
            &mut texture_context,
            Path::new("assets/tankBase.png"),
            Flip::None,
            &texture_settings,
        );

        if tank_sprite.is_ok() {
            self.player.set_tank_sprite(tank_sprite.unwrap());
        }

        let tank_turret = Texture::from_path(
            &mut texture_context,
            Path::new("assets/tankTurret.png"),
            Flip::None,
            &texture_settings,
        );

        if tank_turret.is_ok() {
            self.player.set_turret_sprite(tank_turret.unwrap());
        }
    }

    pub fn render(&self, c: &Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        let center = c
            .transform
            .trans(settings::RESOLUTION[0] / 2.0, settings::RESOLUTION[1] / 2.0);

        let game_object: &dyn Entity = &self.player;
        game_object.render(center, g);
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
            self.player.mov(0.0, -150.0 * delta_time);
        }

        if self.direction.down == KeyStatus::Pressed {
            self.player.mov(0.0, 150.0 * delta_time);
        }

        if self.direction.left == KeyStatus::Pressed {
            self.player.mov(-150.0 * delta_time, 0.0);
        }

        if self.direction.right == KeyStatus::Pressed {
            self.player.mov(150.0 * delta_time, 0.0);
        }
    }
}
