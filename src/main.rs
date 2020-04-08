extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use crate::graphics::Transformed;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod editor;
mod font;
mod frame;
use editor::Editor;

struct App {
    gl: opengl_graphics::GlGraphics,
    main_editor: editor::Editor,
}

impl App {
    fn new(gl: GlGraphics) -> Self {
        return Self {
            gl: gl,
            main_editor: Editor::new(),
        };
    }

    fn render(&mut self, arg: &RenderArgs) {
        let background: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        let main_editor = &mut self.main_editor;
        self.gl.draw(arg.viewport(), |ctx, gl| {
            graphics::clear(background, gl);
            let width = ctx.get_view_size()[0] as u32;
            let height = ctx.get_view_size()[1] as u32;
            main_editor.render(&ctx.scale(2.0, 2.0), gl, width / 2, height / 2);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Środowisko", [1920, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .decorated(false)
        .fullscreen(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        } else if let Some(t) = e.text_args() {
            app.main_editor.insert(t);
        } else if let Some(t) = e.button_args() {
            if t.state == ButtonState::Press && t.button == Button::Keyboard(Key::Backspace) {
                app.main_editor.backspace();
            }
            if t.state == ButtonState::Press && t.button == Button::Keyboard(Key::Return) {
                app.main_editor.enter();
            }
        }
    }
}
