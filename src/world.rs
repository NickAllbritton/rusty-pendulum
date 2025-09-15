use fltk::prelude::*;
use fltk::window;
use fltk::enums::{Color, Font};
use fltk::draw;

use crate::time;
use crate::pendulum::*;
use crate::physics;

// the pendulum systems
//pub static mut systems: Vec<Pendulum> = Vec::new();

#[derive(Clone)]
pub struct World {
    pub window: window::Window,
    data: physics::PhysicsVariables,
    pub systems: Vec<Pendulum>,
    size: (i32, i32),
    pos: (i32, i32),
    time: time::FrameTimer,
    pub play: bool,
    pub started: bool,
    bg_color: Color,
    text_color: Color,
    pub len: i32, // L - length of the pendulum in pixels
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
            data: physics::PhysicsVariables::empty(),
            systems: Vec::new(),
            size: size,
            pos: (0, 0), // Temporary -> call set_height
            time: time::FrameTimer::new(),
            play: false,
            started: false,
            bg_color: bg_color,
            text_color: text_color,
            len: len.floor() as i32,  // 2.5 L's fit in the frame
            orig: (mid_x.floor() as i32, (len*2.2).floor() as i32)
        }
    }
    pub fn setup(&mut self, parent_wnd: &window::Window, y_pos: i32) {
        // Create a pendulum
        //self.systems.push( Pendulum::new(self.len as f32, Color::Red, ApproximationMethods::None) );
        self.window.set_color(self.bg_color);
        let x_pos: i32 = parent_wnd.width()/2 - self.size.0/2 - 4;
        self.pos = (x_pos, y_pos);
        self.window.clone().with_pos(x_pos, y_pos);
        self.window.end();
    }
    
    // For now it just creates a new pendulum without checking whether one already exists
    // TODO: toggle creating and destroying pendulum
    pub fn add_remove_system(len: f32, method: ApproximationMethods, pendulum_systems: &mut Vec<Pendulum>) {
        let mut already_added = false;
        for pendulum in &pendulum_systems.clone() {
            if pendulum.method == method { already_added = true; }
        }
        if !already_added {
            pendulum_systems.push( Pendulum::new(len, method) ); 
        }
        else {
            pendulum_systems.retain( |each_pendulum| each_pendulum.method != method );
        }
    }

    pub fn screen_pos(orig: (i32, i32), world_pos: (i32, i32)) -> (i32, i32) {
        (world_pos.0 + orig.0, -world_pos.1 + orig.1)
    }

    pub fn update(&mut self) {
        let millisperframe: u128 = 17; // roughly 60 fps
        let dt: f32 = 0.035f32;
        // If this many milliseconds have passed since last frame
        // calculate new pendulum positions
        if self.time.mark(millisperframe) && self.play {
            if self.started { 
                self.data.update(dt); 
                //println!("{}", self.data.clone().now());
            }
            for system in &mut self.systems {
                if system.method == ApproximationMethods::SmallAngle {
                    let new_phase = physics::small_angle(self.data.clone().now(), self.data.clone().initial());
                    system.theta = new_phase.0;
                    system.theta_dot = new_phase.1;
                }
                else
                {
                    system.update(dt);
                }
            }
        }
    }

    pub fn initialize(&mut self) {
        self.data = physics::PhysicsVariables::initialize(self.systems.first()
            .expect("This is only ever called from within a callback which checks if self.systems is empty first.")
            .clone().theta);
    }

    pub fn draw(&mut self) {
        // Parameters for the axes
        let text_color = self.text_color;
        let orig_pos = self.orig;
        let len = self.len;
        let radius: i32 = (self.len as f32 / 18f32).floor() as i32;
        let systems = self.systems.clone();
        let orig = self.orig;


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
            for pendulum in &systems {

                let draw_pos_center: (i32, i32) = Self::screen_pos(orig, pendulum.cartesian_pos());
                // Calculate the top right corner
                let draw_pos: (i32, i32) = (draw_pos_center.0 - radius, draw_pos_center.1 - radius);

                // Draw string
                draw::set_draw_color(text_color);
                let attach_pos: (i32, i32) = Self::screen_pos(orig, (0i32, len));
                draw::draw_line(attach_pos.0, attach_pos.1, 
                    draw_pos_center.0, draw_pos_center.1);

                // Draw bob
                draw::set_draw_color(pendulum.method.color());
                draw::draw_pie(draw_pos.0, draw_pos.1, radius*2, radius*2, 0.0, 360.0);
            }
        });
    }
}
