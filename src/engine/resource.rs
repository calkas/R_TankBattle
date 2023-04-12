use gfx_device_gl::Resources;
use gfx_graphics::Flip;
use piston_window::{PistonWindow, Texture, TextureContext, TextureSettings};
use std::collections::HashMap;
use std::path::Path;

pub struct Manager {
    textures: HashMap<String, Texture<Resources>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, window: &PistonWindow,name: &str, file_path: &str, flip: Flip) {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.clone().create_command_buffer().into(),
        };

        let texture_settings = TextureSettings::new();

        let texture = Texture::from_path(
            &mut texture_context,
            Path::new(file_path),
            flip,
            &texture_settings,
        )
        .unwrap();

        self.textures.insert(name.to_string(), texture);
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture<Resources>> {
        self.textures.get(&name.to_string())
    }

    pub fn load_textures(&mut self, window: &PistonWindow) {
        self.load_texture(&window, "tank", "assets/tankBase.png", Flip::None);
        self.load_texture(&window, "turret", "assets/tankTurret.png", Flip::None);
        self.load_texture(&window, "bullet", "assets/bullet.png", Flip::None);
        self.load_texture(&window, "map1", "assets/grass_template2.jpg", Flip::None);
        self.load_texture(&window, "target", "assets/target.png", Flip::None);
        self.load_texture(&window, "ui_score_board", "assets/futureui1.png", Flip::None);
    }
}
