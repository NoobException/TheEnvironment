use crate::font::Font;

pub struct TextLine<'a, 'b> {
    text: &'a str,
    font: &'b Font,
    width: u32,
    height: u32,
    max_width: u32,
}

impl<'a, 'b> TextLine<'a, 'b> {
    pub fn new(text: &'a str, font: &'b Font, max_width: u32) -> Self {
        let width = if (text.len() as u32) < max_width {
            text.len() as u32
        } else {
            max_width
        };
        let height = if text.len() > 0 {
            1 + ((text.len() as u32) - 1) / max_width
        } else {
            1
        };
        TextLine {
            text,
            font,
            width,
            height,
            max_width,
        }
    }
    pub fn get_width(&self) -> u32 {
        self.width * 8
    }
    pub fn get_height(&self) -> u32 {
        self.height * 12
    }
    pub fn get_line_count(&self) -> u32 {
        self.height
    }
    pub fn draw(
        &self,
        ctx: &graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        line_position: u32,
    ) {
        let mut x: u32 = 0;
        let mut y: u32 = line_position;
        for letter in self.text.chars() {
            let color = [0.9, 0.9, 0.9, 1.0];
            self.font.draw_letter(ctx, gl, letter, color, (x, y));
            x += 1;
            if x >= self.max_width {
                x = 0;
                y += 1;
            }
        }
    }
}
