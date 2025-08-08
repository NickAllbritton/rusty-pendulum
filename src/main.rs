use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.physics_sim.rusty_pendulum";

fn main() -> glib::ExitCode {
    // TODO: Use different window proportions
    let _wnd_width: u32 = 1000;
    let _wnd_height: u32 = 1000;
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("RustyPendulum")
        .build();

    // Present window
    window.present();
}
