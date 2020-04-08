use graphics::types::{Color, SourceRectangle};
use graphics::{DrawState, Image, ImageSize, Transformed};
use opengl_graphics::{Filter, Texture, TextureSettings};

use std::path::Path;

impl Font {
    const LETTER_COUNT: usize = 96;
    const CHARACTER_SIZE: (u32, u32) = (8, 12);
    const FIRST_CHARACTER: char = ' ';
}
pub struct Font {
    images: [Image; Font::LETTER_COUNT],
    texture: Texture,
}
impl Font {
    pub fn from_file(path: &Path) -> Self {
        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);
        let texture = Texture::from_path(path, &texture_settings).unwrap();
        let (width, _height) = texture.get_size();
        let width = width / Self::CHARACTER_SIZE.0;
        // let height = height / Self::CHARACTER_SIZE.1;
        let images: [Image; Font::LETTER_COUNT] = array_init::array_init(|i: usize| {
            let x = (i as u32 % width) as f64;
            let y = (i as u32 / width) as f64;
            let src_rect: SourceRectangle = [x * 8.0, y * 12.0, 8.0, 12.0];
            let image = Image::new().src_rect(src_rect).color([0.0, 1.0, 0.0, 1.0]);
            return image;
        });
        return Self { images, texture };
    }
    pub fn draw_letter(
        &self,
        ctx: &graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        letter: char,
        color: Color,
        (x, y): (u32, u32),
    ) {
        let x = x as f64 * 8.0;
        let y = y as f64 * 12.0;
        let image = self.get_letter_image(letter, color);
        image.draw(
            &self.texture,
            &DrawState::new_alpha(),
            ctx.transform.trans(x, y),
            gl,
        );
    }
    fn get_letter_image(&self, letter: char, color: Color) -> Image {
        let mut image = if (letter as usize) < (Font::FIRST_CHARACTER as usize)
            || (letter as usize) >= (Font::FIRST_CHARACTER as usize + Font::LETTER_COUNT)
        {
            self.images[Font::LETTER_COUNT - 1].clone()
        } else {
            self.images[letter as usize - Font::FIRST_CHARACTER as usize].clone()
        };
        image.color = Some(color);
        return image;
    }
}
