pub const RESOLUTION: [f64; 2] = [1280.0, 720.0];
pub const TITLE: &str = "R_TankBattle";
pub const GAME_TIME: f64 = 60.0;

#[derive(PartialEq, Eq)]
pub enum KeyStatus {
    Pressed,
    Released,
}