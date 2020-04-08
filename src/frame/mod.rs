use graphics::math::Vec2d;
use graphics::types::SourceRectangle;
use graphics::{DrawState, Image, ImageSize, Transformed};
use opengl_graphics::{Filter, Texture, TextureSettings, Wrap};

use std::path::Path;

pub struct Frame {
    texture: Texture,
    corner_size: Vec2d,
}
impl Frame {
    pub fn from_file(path: &Path) -> Self {
        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);
        texture_settings.set_wrap_u(Wrap::ClampToEdge);
        texture_settings.set_wrap_v(Wrap::ClampToEdge);
        let texture = Texture::from_path(path, &texture_settings).unwrap();
        let (_width, height) = texture.get_size();
        let corner_size = [height as f64 / 2.0; 2];
        return Self {
            texture,
            corner_size,
        };
    }
    pub fn draw(
        &self,
        ctx: &graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        (width, height): (u32, u32),
        split_at: u32,
    ) {
        let (screen_width, screen_height) = (width as f64, height as f64);
        let [corner_width, corner_height] = self.corner_size;
        let src_rect: SourceRectangle = [
            corner_width + 2.0,
            corner_height,
            screen_width - corner_width,
            screen_height - corner_height,
        ];
        let image = Image::new().src_rect(src_rect);
        image.draw(&self.texture, &DrawState::new_alpha(), ctx.transform, gl);
        let src_rect: SourceRectangle = [
            corner_width + 2.0,
            0.0,
            screen_width - corner_width,
            corner_height,
        ];
        let image = Image::new().src_rect(src_rect);
        image.draw(
            &self.texture,
            &DrawState::new_alpha(),
            ctx.transform.trans(0.0, screen_height - corner_height),
            gl,
        );
        let src_rect: SourceRectangle = [
            0.0,
            corner_height,
            corner_width,
            screen_height - corner_height,
        ];
        let image = Image::new().src_rect(src_rect);
        image.draw(
            &self.texture,
            &DrawState::new_alpha(),
            ctx.transform.trans(screen_width - corner_width, 0.0),
            gl,
        );
        let src_rect: SourceRectangle = [0.0, 0.0, corner_width, corner_height];
        let image = Image::new().src_rect(src_rect);
        image.draw(
            &self.texture,
            &DrawState::new_alpha(),
            ctx.transform
                .trans(screen_width - corner_width, screen_height - corner_height),
            gl,
        );
        let src_rect: SourceRectangle = [
            corner_width,
            corner_height,
            2.0,
            screen_height - corner_height,
        ];
        let image = Image::new().src_rect(src_rect);
        image.draw(
            &self.texture,
            &DrawState::new_alpha(),
            ctx.transform.trans(split_at as f64, 0.0),
            gl,
        );
        let src_rect: SourceRectangle = [corner_width, 0.0, 2.0, corner_height];
        let image = Image::new().src_rect(src_rect);
        image.draw(
            &self.texture,
            &DrawState::new_alpha(),
            ctx.transform
                .trans(split_at as f64, screen_height - corner_height),
            gl,
        );
    }
}
