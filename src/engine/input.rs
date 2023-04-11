use super::settings::KeyStatus;
use piston_window::{Button, Key};

pub struct Controller {
    up: KeyStatus,
    down: KeyStatus,
    left: KeyStatus,
    right: KeyStatus,
    turret_left: KeyStatus,
    turret_right: KeyStatus,
    fire: KeyStatus,
}

impl Controller {
    pub fn default() -> Self {
        Self {
            up: KeyStatus::Released,
            down: KeyStatus::Released,
            left: KeyStatus::Released,
            right: KeyStatus::Released,
            turret_left: KeyStatus::Released,
            turret_right: KeyStatus::Released,
            fire: KeyStatus::Released,
        }
    }

    pub fn on_input(&mut self, button: Button, status: KeyStatus) {
        match button {
            Button::Keyboard(Key::Up) => self.up = status,
            Button::Keyboard(Key::Down) => self.down = status,
            Button::Keyboard(Key::Left) => self.left = status,
            Button::Keyboard(Key::Right) => self.right = status,
            Button::Keyboard(Key::S) => self.turret_left = status,
            Button::Keyboard(Key::D) => self.turret_right = status,
            Button::Keyboard(Key::Space) => self.fire = status,
            _ => {}
        }
    }

    pub fn is_up(&self) -> bool {
        self.up == KeyStatus::Pressed
    }

    pub fn is_down(&self) -> bool {
        self.down == KeyStatus::Pressed
    }

    pub fn is_left(&self) -> bool {
        self.left == KeyStatus::Pressed
    }
    pub fn is_right(&self) -> bool {
        self.right == KeyStatus::Pressed
    }

    pub fn is_turret_left(&self) -> bool {
        self.turret_left == KeyStatus::Pressed
    }

    pub fn is_turret_right(&self) -> bool {
        self.turret_right == KeyStatus::Pressed
    }

    pub fn is_fire(&self) -> bool {
        self.fire == KeyStatus::Pressed
    }
}

#[cfg(test)]

mod unit_tests {
    use super::*;

    #[test]
    fn button_press_test() {
        let mut input = Controller::default();

        assert_eq!(input.is_up(), false);
        input.on_input(Button::Keyboard(Key::Up), KeyStatus::Pressed);
        assert_eq!(input.is_up(), true);

        assert_eq!(input.is_fire(), false);
        input.on_input(Button::Keyboard(Key::Space), KeyStatus::Pressed);
        assert_eq!(input.is_fire(), true);

        input.on_input(Button::Keyboard(Key::Space), KeyStatus::Released);
        assert_eq!(input.is_fire(), false);
    }
}
