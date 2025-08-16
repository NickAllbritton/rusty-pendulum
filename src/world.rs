use fltk::prelude::*;
use fltk::window;
use fltk::enums::Color;

pub struct World {
    window: window::Window,
    size: (i32, i32),
    pos: (i32, i32),
    bg_color: Color,
    text_color: Color
}

impl World {
    pub fn new(size: (i32, i32), bg_color: Color, text_color: Color) -> Self {
        let world_window = window::Window::default()
            .with_size(size.0, size.1);
        Self {
            window: world_window,
            size: size,
            pos: (0, 0), // Temporary -> call set_height
            bg_color: bg_color,
            text_color: text_color
        }
    }

    pub fn setup(&mut self, parent_wnd: &window::Window, y_pos: i32) {
        self.window.set_color(self.bg_color);
        let x_pos: i32 = parent_wnd.width()/2 - self.size.0/2 - 4;
        self.pos = (x_pos, y_pos);
        self.window.clone().with_pos(x_pos, y_pos);
        self.window.end();
    }
}
