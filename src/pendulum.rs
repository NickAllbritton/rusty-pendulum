use fltk::prelude::*;
use fltk::enums::Color;

// TODO: add an enum for the type of approximation and add a member to 
// pendulum struct 

pub struct Pendulum {
    theta: f32,
    color: Color
}

impl Pendulum {
    pub fn new() -> Self {
        Self {
            theta: 0f32, // Keep this
            color: Color::Red // temporary - replace this with command line argument
        }
    }

    // cartesian_coords function

}
