use gfx_device_gl::Resources;
use gfx_graphics::Flip;
use piston_window::{PistonWindow, Texture, TextureContext, TextureSettings };
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
pub struct Manager {
    textures: HashMap<String, Texture<Resources>>,
}

impl Manager {

    pub fn new() -> Self {
        Manager { textures: HashMap::new()}
    }
    pub fn load_texture(
        &mut self,
        window: &PistonWindow,
        name: String,
        file_path: String,
        flip: Flip,
    ) {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.clone().create_command_buffer().into(),
        };

        let texture_settings = TextureSettings::new();

        let texture = Texture::from_path(
            &mut texture_context,
            Path::new(&file_path),
            flip,
            &texture_settings,
        ).unwrap();

        self.textures.insert(name, texture);
    }

    pub fn get_texture(&self, name: String) -> Option<&Texture<Resources>> {
        self.textures.get(&name)
    }
}
