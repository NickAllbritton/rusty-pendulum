use fltk::prelude::*;
use fltk::window;
use fltk::app;

const APP_ID: &str = "org.physics_sim.rusty_pendulum";

fn main() -> Result<(), String> {
    // TODO: Use different window proportions
    let wnd_width: i32 = 1000;
    let wnd_height: i32 = 1000;
    // Create application
    let app = app::App::default();
    // Create a window
    let mut wnd = window::Window::new(100, 100, wnd_width, wnd_height, "RustyPendulum");
    wnd.end();
    wnd.show();
    // Run the application
    app.run().unwrap();

    Ok(())
}
