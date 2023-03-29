extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate piston_window;
extern crate rand;
use piston_window::*;
mod engine;
mod object;
use engine::game::Game;
use engine::resource;
use engine::settings;

fn main() {
    println!("..::R_TankBattle::..");

    let mut windows_settings =
        WindowSettings::new(settings::TITLE.to_string(), settings::RESOLUTION);
    windows_settings.set_exit_on_esc(true);

    let mut window: PistonWindow = windows_settings.build().unwrap();

    let mut resource_manager = resource::Manager::new();
    resource_manager.load_textures(&window);

    let mut glyph = window.load_font("assets/fonts/Roboto-Bold.ttf").unwrap();

    let mut game = Game::new(&resource_manager);

    let mut events = Events::new(EventSettings::new());

    println!("Game starts...");

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

        window.draw_2d(&e, |c, g, d| {
            game.render(&c, g, &mut glyph);
            // Update glyphs before rendering.
            glyph.factory.encoder.flush(d);
        });
    }

    println!("\n...::END::..")
}
