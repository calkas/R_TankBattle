extern crate piston_window;
use piston_window::*;


fn main() {
    println!("..::R_TankBattle::..");

    let window_res = [600.0, 400.0];
    let mut windows_settings = WindowSettings::new("RTankBattle", window_res);
    windows_settings.set_exit_on_esc(true);


    let mut window: PistonWindow = windows_settings.build().unwrap();


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(& mut window)  {
        
    }



    println!("...::END::..")
}
