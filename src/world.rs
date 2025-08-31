use fltk::prelude::*;
use fltk::window;
use fltk::enums::{Color, Font};
use fltk::draw;

use crate::pendulum::Pendulum;

pub struct World {
    window: window::Window,
    systems: Vec<Pendulum>,
    size: (i32, i32),
    pos: (i32, i32),
    bg_color: Color,
    text_color: Color,
    len: i32, // L - length of the pendulum in pixels
    orig: (i32, i32) // origin in screen coordinates
}

impl World {
    pub fn new(size: (i32, i32), bg_color: Color, text_color: Color) -> Self {
        let world_window = window::Window::default()
            .with_size(size.0, size.1);
        let len: f32 = world_window.clone().height() as f32 / 2.5;
        let mid_x: f32 = world_window.clone().width() as f32 /2.0;
        Self {
            window: world_window,
            systems: Vec::new(),
            size: size,
            pos: (0, 0), // Temporary -> call set_height
            bg_color: bg_color,
            text_color: text_color,
            len: len.floor() as i32,  // 2.5 L's fit in the frame
            orig: (mid_x.floor() as i32, (len*2.2).floor() as i32)
        }
    }

    pub fn setup(&mut self, parent_wnd: &window::Window, y_pos: i32) {
        self.window.set_color(self.bg_color);
        let x_pos: i32 = parent_wnd.width()/2 - self.size.0/2 - 4;
        self.pos = (x_pos, y_pos);
        self.window.clone().with_pos(x_pos, y_pos);
        self.window.end();
    }

    pub fn screen_pos(&self, world_pos: (i32, i32)) -> (i32, i32) {
        (world_pos.0 + self.orig.0, -world_pos.1 + self.orig.1)
    }

    pub fn draw(&mut self) {
        // Parameters for the axes
        let text_color = self.text_color;
        let orig_pos = self.orig;
        let len = self.len;
        let radius: i32 = (self.len as f32 / 18f32).floor() as i32;
        // Positions and colors of pendulums
        let mut pendulums: Vec<( (i32, i32), Color)> = Vec::new();
        for pendulum in &self.systems {
            // Parameters for the pendulums
            let draw_pos_center: (i32, i32) = self.screen_pos(pendulum.cartesian_pos());
            pendulums.push( ( draw_pos_center, pendulum.color) );
        }


        self.window.draw(move |wnd| {
            /*************** Cartesian coordinate axis **************/
            draw::set_draw_color(text_color);
            // Draw the cartesian axes
            draw::draw_line(0, orig_pos.1, wnd.w(), orig_pos.1); // x-axis
            draw::draw_line(orig_pos.0, 0, orig_pos.0, wnd.h()); // y-axis

            let tick_separation: i32 = (len as f32 / 4.0).floor() as i32;
            // Draw the x-axis ticks
            // Right tick marks
            let mut x_pos = orig_pos.0 + tick_separation;
            let mut y_pos = orig_pos.1;
            while x_pos < wnd.width() {
                draw::draw_line(x_pos, y_pos, x_pos, y_pos - 10);
                x_pos += tick_separation;
            }
            // Left tick marks
            x_pos = orig_pos.0 - tick_separation;
            while x_pos > 0 {
                draw::draw_line(x_pos, y_pos, x_pos, y_pos - 10);
                x_pos -= tick_separation;
            }
            // Draw the y-axis ticks
            // Bottom tick marks
            x_pos = orig_pos.0;
            y_pos = orig_pos.1 - tick_separation;
            while y_pos < wnd.height() {
                draw::draw_line(x_pos, y_pos, x_pos + 10, y_pos);
                y_pos += tick_separation;
            }
            // Top tick marks
            x_pos = orig_pos.0;
            y_pos = orig_pos.1 + tick_separation;
            while y_pos > 0 {
                draw::draw_line(x_pos, y_pos, x_pos + 10, y_pos);
                y_pos -= tick_separation;
            }
            // Draw the axis labels
            draw::set_font(Font::Screen, 20);
            // x-axis
            draw::draw_text("L", orig_pos.0 + len - 8, orig_pos.1 - 20);
            draw::draw_text("-L", orig_pos.0 - len - 8, orig_pos.1 - 20);
            // y-axis
            draw::draw_text("L", orig_pos.0 + 20, orig_pos.1 - len + 9);
            draw::draw_text("2L", orig_pos.0 + 20, orig_pos.1 - 2*len + 9);


            /***************     Draw the pendulums    **************/
            for pendulum in &pendulums {

                // Calculate the top right corner
                let draw_pos: (i32, i32) = (pendulum.0.0 - radius, pendulum.0.1 - radius);

                // TODO: Update for actual pendulums
                
                // Draw string
                draw::set_draw_color(text_color);
                // TODO: Draw string

                // Draw bob
                draw::set_draw_color(pendulum.1);
                draw::draw_pie(draw_pos.0, draw_pos.1, radius*2, radius*2, 0.0, 360.0);
            }
       });
    }
}
