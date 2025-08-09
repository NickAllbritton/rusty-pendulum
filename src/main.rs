use fltk::prelude::*;
use fltk::window;
use fltk::app;
use fltk::frame::Frame;
use fltk::enums::*;

const APP_ID: &str = "org.physics_sim.rusty_pendulum";

fn main() -> Result<(), String> {
    // TODO: Use different window proportions
    let wnd_width: i32 = 1000;
    let wnd_height: i32 = 1000;
    // Create application
    //let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let app = app::App::default();
    // Create a window
    let mut wnd = window::Window::new(800, 200, wnd_width, wnd_height, "RustyPendulum");
    // TODO: Create a menubar
    // ...
    // Create a title at the top of the app
    let mut title = Frame::default().with_label("RustyPendulum");
    title.set_frame(FrameType::RoundedBox);
    title.set_label_font(Font::ScreenBold);
    title.set_label_size(30);
    title.set_label_color(Color::from_rgb(55, 50, 42));
    let label_dimensions: (i32, i32) = title.measure_label();
    title.set_size(label_dimensions.0 + 5, label_dimensions.1);
    //let title_w = title.clone().w();
    let title_h = title.clone().h();
    title.set_color(Color::from_rgb(200, 200, 190));
    title.center_x(&wnd);
    // Create a window to contain the simulation
    let mut pendulum_wnd = window::Window::new(20, title_h + 4, 
                    wnd_width - 50, wnd_height - title_h - 20, 
                    None);
    pendulum_wnd.set_color(Color::from_rgb(200, 200, 190));
    pendulum_wnd.end();

    // Window settings details
    wnd.set_color(Color::from_rgb(55, 50, 42));
    wnd.make_resizable(false); // To prevent xmonad from tiling it
    wnd.end();
    wnd.show();
    // Run the application
    app.run().unwrap();

    Ok(())
}
