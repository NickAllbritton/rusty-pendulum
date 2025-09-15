use fltk::enums::Color;

#[derive(PartialEq, Clone)]
pub enum ApproximationMethods {
    None,
    SmallAngle,
    Euler,
    EulerCromer,
    RungeKutta
}

#[derive(Clone)]
pub struct Pendulum {
    theta: f32,
    theta_dot: f32,
    length: f32,
    method: ApproximationMethods,
    pub color: Color
}

impl Pendulum {
    pub fn new(length: f32, color: Color, approx_method: ApproximationMethods) -> Self {
        Self {
            theta: 0f32, // Keep this
            theta_dot: 0f32, // Keep this
            length: length,
            method: approx_method, 
            color: color 
        }
    }

    pub fn method(&self) -> String {
        match self.method {
            ApproximationMethods::None => { "None".to_string() },
            ApproximationMethods::SmallAngle => { "Small angle approximation".to_string() },
            ApproximationMethods::Euler => { "Euler method".to_string() },
            ApproximationMethods::EulerCromer => { "Euler-Cromer method".to_string() },
            ApproximationMethods::RungeKutta => { "Runge-Kutta method".to_string() }
        }
    }


    pub fn update(&mut self, phase: (f32, f32)) {
        self.theta = phase.0;
        self.theta_dot = phase.1;
    }

    // cartesian_coords function
    pub fn cartesian_pos(&self) -> (i32, i32) {
        ((self.length*self.theta.sin()).floor() as i32,
        (self.length - self.length*self.theta.cos()).floor() as i32)
    }
}
