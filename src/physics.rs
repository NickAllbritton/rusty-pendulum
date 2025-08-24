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
    (0f32, 0f32)
}

// Second-order Euler-Cromer method
fn euler_cromer(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
    (0f32, 0f32)
}

// Runge-Kutta?
fn runge_kutta(dt: f32, prev_bob: (f32, f32)) -> (f32, f32) {
    (0f32, 0f32)
}
