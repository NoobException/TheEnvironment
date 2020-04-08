use graphics::Transformed;
use std::path::Path;

use crate::font::Font;
use crate::frame::Frame;

mod text_line;
use text_line::TextLine;

pub struct Editor {
    content: Vec<String>,
    current_line: usize,
    font: Font,
    frame: Frame,
}

impl Editor {
    pub fn new() -> Self {
        return Self {
            content: vec![String::new()],
            current_line: 0,
            font: Font::from_file(Path::new("res/fonts/basic8bit.png")),
            frame: Frame::from_file(Path::new("res/frames/frame.png")),
        };
    }
    pub fn render(
        &mut self,
        ctx: &graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        width: u32,
        height: u32,
    ) {
        let left_margin_size = TextLine::new(&(self.content.len()).to_string(), &self.font, 15)
            .get_width() as f64
            + 10.0;
        self.frame
            .draw(ctx, gl, (width, height), left_margin_size as u32);
        let mut line_position = 0;
        let mut line_number = 1;
        for text_line in &self.content {
            let text_line = TextLine::new(text_line, &self.font, 80);
            text_line.draw(&ctx.trans(10.0 + left_margin_size, 10.0), gl, line_position);
            let number_text = line_number.to_string();
            let number_text = TextLine::new(&number_text, &self.font, 80);
            number_text.draw(
                &ctx.trans(left_margin_size - number_text.get_width() as f64, 10.0),
                gl,
                line_position,
            );
            line_number += 1;
            line_position += text_line.get_line_count();
        }
    }
    pub fn insert(&mut self, text: String) {
        self.content[self.current_line].push_str(text.as_str());
    }
    pub fn backspace(&mut self) {
        if self.content[self.current_line].is_empty() {
            if self.current_line > 0 {
                self.content.remove(self.current_line);
                self.current_line -= 1;
            }
        } else {
            self.content[self.current_line].pop();
        }
    }
    pub fn enter(&mut self) {
        self.content.push(String::new());
        self.current_line += 1;
    }
}
