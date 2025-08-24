use fltk::prelude::*;
use fltk::enums::Color;

#[derive(PartialEq)]
pub enum ApproximationMethods {
    None,
    Small_Angle,
    Euler,
    Euler_Cromer,
    Runge_Kutta
}

pub struct Pendulum {
    theta: f32,
    theta_dot: f32,
    length: f32,
    method: ApproximationMethods,
    color: Color
}

impl Pendulum {
    pub fn new(length: f32) -> Self {
        Self {
            theta: 0f32, // Keep this
            theta_dot: 0f32, // Keep this
            length: length,
            method: ApproximationMethods::None, // Probably keep this?
            color: Color::Red // temporary - replace this with command line argument
        }
    }

    // cartesian_coords function
    pub fn cartesian_pos(&self) -> (i32, i32) {
        ((self.length*self.theta.sin()).floor() as i32,
        (self.length - self.length*self.theta.cos()).floor() as i32)
    }
}
