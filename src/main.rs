extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate piston_window;
use piston_window::*;
mod engine;
mod object;
use engine::game::{settings, Game};

fn main() {
    println!("..::R_TankBattle::..");

    let mut game = Game::new();

    let mut windows_settings =
        WindowSettings::new(settings::TITLE.to_string(), settings::RESOLUTION);
    windows_settings.set_exit_on_esc(true);

    let mut window: PistonWindow = windows_settings.build().unwrap();

    game.load_sprites(&window);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        e.press(|key| {
            game.input(key, settings::KeyStatus::Pressed);
        });

        e.release(|key| {
            game.input(key, settings::KeyStatus::Released);
        });

        e.update(|args| {
            game.update(args.dt);
        });

        window.draw_2d(&e, |c, g, _d| {
            game.render(&c, g);
        });
    }

    println!("\n...::END::..")
}
