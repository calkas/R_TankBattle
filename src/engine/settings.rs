pub const RESOLUTION: [f64; 2] = [800.0, 600.0];
pub const TITLE: &str = "R_TankBattle";

#[derive(PartialEq, Eq)]
pub enum KeyStatus {
    Pressed,
    Released,
}