const sqrt_g_ovr_L: f32 = 1f32;

// Small-angle analytic approximation
fn small_angle(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
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
    (prev_bob.0*(dt*sqrt_g_ovr_L).cos(), 
     -prev_bob.1*sqrt_g_ovr_L*(dt*sqrt_g_ovr_L).sin())
}

// Second-order Euler method
fn euler_method(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
    let theta = prev_bob.0 + dt*prev_bob.1; // theta_n+1 = theta_n + dt*omega_n
    // The following calculation is inefficient if sqrt_g_ovr_L remains 1 because 1*1=1
    let theta_double_dot = -(sqrt_g_ovr_L*sqrt_g_ovr_L)*prev_bob.0.sin(); // alpha_n = -g/L*sin(theta_n)
    let theta_dot = prev_bob.1 + dt*theta_double_dot; // omega_n+1 = omega_n + dt*alpha_n
    (theta, theta_dot)
}

// Second-order Euler-Cromer method
fn euler_cromer(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
    // This method comes from Euler method but swap the order of position and velocity calculation
    // and then use the new velocity
    // The following calculation is inefficient if sqrt_g_ovr_L remains 1 because 1*1=1
    let theta_double_dot = -(sqrt_g_ovr_L*sqrt_g_ovr_L)*prev_bob.0.sin(); // alpha_n = -g/L*sin(theta_n)
    let theta_dot = prev_bob.1 + dt*theta_double_dot; // omega_n+1 = omega_n + dt*alpha_n
    let theta = prev_bob.0 + dt*theta_dot; // theta_n+1 = theta_n + dt*omega_n+1
    (theta, theta_dot)
}

// Runge-Kutta?
/*fn runge_kutta(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
    (0f32, 0f32)
}*/
