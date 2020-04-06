extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use graphics::types::{Color, SourceRectangle};
use graphics::{DrawState, Image, ImageSize, Transformed};
use opengl_graphics::{Filter, GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::path::Path;

impl Font {
    const LETTER_COUNT: usize = 96;
    const CHARACTER_SIZE: (u32, u32) = (8, 12);
    const FIRST_CHARACTER: char = ' ';
}

struct Font {
    images: [Image; Font::LETTER_COUNT],
    texture: Texture,
}

impl Font {
    fn from_file(path: &Path) -> Self {
        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);
        let texture = Texture::from_path(path, &texture_settings).unwrap();

        let (width, height) = texture.get_size();

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

    fn draw_letter(
        &self,
        ctx: &graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        letter: char,
        color: Color,
        (x, y): (u32, u32),
        scale: f64,
    ) {
        let x = x as f64 * 8.0 * scale;
        let y = y as f64 * 12.0 * scale;

        let image = self.get_letter_image(letter, color);
        image.draw(
            &self.texture,
            &DrawState::new_alpha(),
            ctx.transform.trans(x, y).scale(scale, scale),
            gl,
        );
    }
}

struct Editor {
    gl: GlGraphics,
    content: String,
    font: Font,
}

impl Editor {
    fn new(gl: GlGraphics) -> Self {
        return Self {
            gl: gl,
            content: String::new(),
            font: Font::from_file(Path::new("font.png")),
        };
    }

    fn render(&mut self, arg: &RenderArgs) {
        let background: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

        let content = &self.content;
        let font = &mut self.font;

        self.gl.draw(arg.viewport(), |ctx, gl| {
            graphics::clear(background, gl);
            let mut x = 0;
            for letter in content.chars() {
                font.draw_letter(&ctx, gl, letter, [0.0, 1.0, 0.0, 1.0], (x, 0), 2.0);
                x += 1;
            }
        });
    }

    fn insert(&mut self, text: String) {
        if text.chars().count() > 0 {
            // assert_eq!(text.chars().count(), 1);
            //   assert!(text.chars().next().unwrap().is_ascii_graphic());
        }

        self.content.push_str(text.as_str());
    }

    fn backspace(&mut self) {
        self.content.pop();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Środowisko", [1000, 50])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .decorated(false)
        .build()
        .unwrap();

    let mut editor = Editor::new(GlGraphics::new(opengl));
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            editor.render(&r);
        } else if let Some(t) = e.text_args() {
            editor.insert(t);
            println!("{:?}", editor.content);
        } else if let Some(t) = e.button_args() {
            if t.state == ButtonState::Press && t.button == Button::Keyboard(Key::Backspace) {
                editor.backspace();
            }
        }
    }
}
