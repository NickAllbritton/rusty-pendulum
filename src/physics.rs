const SQRT_Q_OVR_L: f32 = 1f32;

// Small-angle analytic approximation
pub fn small_angle(t: f32, init_bob: (f32, f32)) -> (f32, f32) {
    // Small-angle approximation is analytical, meaning:
    //
    // -> Provide the TOTAL elapsed time and the initial angle
    // and the equation will produce the new angle (and angular velocity)
    // BUT
    // -> Practically speaking, by providing the previous angle and previous velocity
    // and using the equation with the elapsed time since the previous calculation,
    // the result is the same as the typcally way. 
    // 
    // The benefit of this way is to have the same parameters and return values as the
    // numerical methods.
    let theta = init_bob.0*(SQRT_Q_OVR_L*t).cos();
    let theta_dot = -init_bob.0*SQRT_Q_OVR_L*(SQRT_Q_OVR_L*t).sin();
    (theta, theta_dot)
}

// Second-order Euler method
pub fn euler_method(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
    let theta = prev_bob.0 + dt*prev_bob.1; // theta_n+1 = theta_n + dt*omega_n
    // The following calculation is inefficient if sqrt_g_ovr_L remains 1 because 1*1=1
    let theta_double_dot = -(SQRT_Q_OVR_L*SQRT_Q_OVR_L)*prev_bob.0.sin(); // alpha_n = -g/L*sin(theta_n)
    let theta_dot = prev_bob.1 + dt*theta_double_dot; // omega_n+1 = omega_n + dt*alpha_n
    (theta, theta_dot)
}

// Second-order Euler-Cromer method
pub fn euler_cromer(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
    // This method comes from Euler method but swap the order of position and velocity calculation
    // and then use the new velocity
    // The following calculation is inefficient if sqrt_g_ovr_L remains 1 because 1*1=1
    let theta_double_dot = -(SQRT_Q_OVR_L*SQRT_Q_OVR_L)*prev_bob.0.sin(); // alpha_n = -g/L*sin(theta_n)
    let theta_dot = prev_bob.1 + dt*theta_double_dot; // omega_n+1 = omega_n + dt*alpha_n
    let theta = prev_bob.0 + dt*theta_dot; // theta_n+1 = theta_n + dt*omega_n+1
    (theta, theta_dot)
}

// Runge-Kutta?
/*pub fn runge_kutta(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
    (0f32, 0f32)
}*/

#[derive(Clone)]
pub struct PhysicsVariables {
    t: Vec<f32>,
    theta: Vec<f32>,
    theta_dot: Vec<f32>,
    //theta_double_dot: Vec<f32>
}

impl PhysicsVariables {
    pub fn empty() -> Self {
        Self {
            t: Vec::new(),
            theta: Vec::new(),
            theta_dot: Vec::new()
        }
    }

    pub fn initialize(theta: f32) -> Self {
        Self {
            t: vec![0f32],
            theta: vec![theta],
            theta_dot: vec![0f32]
        }
    }
    
    // todo: add other data
    pub fn update(&mut self, dt: f32) {
        self.t.push( *self.t.last().expect("initialize() always called before update()") + dt );
    }

    // return global time
    pub fn now(self) -> f32 {
        *self.t.last().expect("now() should never be called before initialize()")
    }

    pub fn initial(self) -> (f32, f32) {
        (*self.theta.first().expect("initial() should not be called before initialize()"), 
         *self.theta_dot.first().expect("initial() should never be called before initialize()"))
    }
}
