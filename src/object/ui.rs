use super::{Object, Renderable};
use crate::engine::settings;
use gfx_device_gl::Resources;
use piston_window::Texture;
use piston_window::{Context, G2d, Glyphs, Text, Transformed};
#[allow(dead_code)]
pub mod color {
    pub const ORANGE: [f32; 4] = [255.0, 215.0, 0.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    pub const GREEN: [f32; 4] = [0.0, 255.0, 0.0, 1.0];
    pub const RED: [f32; 4] = [255.0, 0.0, 0.0, 1.0];
    pub const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
}
pub struct UI<'a> {
    score_board: Object<'a>,
    game_score: u32,
    game_time: f64,
    gameover_board: Object<'a>,
}

impl<'a> UI<'a> {
    pub fn new(score_board_texture: &'a Texture<Resources>, gameover_texture: &'a Texture<Resources>) -> Self {
        Self {
            score_board: Object {
                x: settings::RESOLUTION[0] / 8.0,
                y: settings::RESOLUTION[1] / 12.0,
                scale: 1.0,
                rotation: 0.0,
                velocity: 1.0,
                sprite: Some(score_board_texture),
            },
            game_score: 0,
            game_time: 0.0,

            gameover_board: Object {
                x: settings::RESOLUTION[0] / 2.0,
                y: settings::RESOLUTION[1] / 2.0,
                scale: 1.0,
                rotation: 0.0,
                velocity: 1.0,
                sprite: Some(gameover_texture),
            },
        }
    }

    pub fn score_board_update(&mut self, score: u32, time: f64) {
        self.game_score = score;
        self.game_time = time;
    }

    pub fn render(&self, c: &Context, g: &mut G2d, glyph: &mut Glyphs) {
        self.score_board.render(c.transform, g);
        let score_str = format!("Score: {}         Time: {:.1}", self.game_score, self.game_time);
        Self::draw_text(
            score_str.as_str(),
            [30.0, 60.0],
            20,
            color::ORANGE,
            glyph,
            c,
            g,
        );
    }

    pub fn game_over(&self, c: &Context, g: &mut G2d, glyph: &mut Glyphs) {
        self.gameover_board.render(c.transform, g);

        let score_str = format!("X {} ", self.game_score);
        Self::draw_text(
            score_str.as_str(),
            [(settings::RESOLUTION[0] / 2.0) - 20.0, (settings::RESOLUTION[1] / 2.0) + 100.0],
            48,
            color::WHITE,
            glyph,
            c,
            g,
        );

        Self::draw_text(
            "Press Enter to restart the game",
            [(settings::RESOLUTION[0] / 3.0) + 50.0, (settings::RESOLUTION[1] / 2.0) + 190.0],
            20,
            color::GREEN,
            glyph,
            c,
            g,
        );
    }

    pub fn draw_text(
        text: &str,
        pos: [f64; 2],
        size: u32,
        color: [f32; 4],
        glyph: &mut Glyphs,
        c: &Context,
        g: &mut G2d,
    ) {
        Text::new_color(color, size)
            .draw(
                text,
                glyph,
                &c.draw_state,
                c.transform.trans(pos[0], pos[1]),
                g,
            )
            .unwrap();
    }
}
