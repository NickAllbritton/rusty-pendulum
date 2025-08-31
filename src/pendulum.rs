use fltk::prelude::*;
use fltk::enums::Color;

#[derive(PartialEq)]
pub enum ApproximationMethods {
    None,
    SmallAngle,
    Euler,
    EulerCromer,
    RungeKutta
}

pub struct Pendulum {
    theta: f32,
    theta_dot: f32,
    length: f32,
    method: ApproximationMethods,
    pub color: Color
}

impl Pendulum {
    pub fn new(length: f32) -> Self {
        Self {
            theta: 0f32, // Keep this
            theta_dot: 0f32, // Keep this
            length: length,
            method: ApproximationMethods::None, // Probably keep this?
            color: Color::Red // temporary - replace this with constructor argument
        }
    }

    // cartesian_coords function
    pub fn cartesian_pos(&self) -> (i32, i32) {
        ((self.length*self.theta.sin()).floor() as i32,
        (self.length - self.length*self.theta.cos()).floor() as i32)
    }
}
