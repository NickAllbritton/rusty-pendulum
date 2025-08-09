use fltk::prelude::*;
use fltk::window;
use fltk::app;
use fltk::frame::Frame;
use fltk::menu;
use fltk::enums::*;

//const APP_ID: &str = "org.physics_sim.rusty_pendulum";

fn menu_cb(m: &mut impl MenuExt) {
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            //TODO: Do stuff
            "Play\t" => {},
            "Pause\t" => {},
            "Reset\t" => {},
            "Quit\t" => { app::quit(); },
            "Double pendulum\t" => {},
            _ => {}
        }
    }
}

fn main() -> Result<(), String> {
    // TODO: Use different window proportions
    let wnd_width: i32 = 1000;
    let wnd_height: i32 = 1000;
    let menu_bar_height: i32 = 30;
    // Colors to use
    let dark_brown: Color = Color::from_rgb(30, 24, 19);
    let graph_paper: Color = Color::from_rgb(195, 190, 175);
    let rust_color: Color = Color::from_rgb(150, 80, 50);
    // Create application
    let app = app::App::default().with_scheme(app::Scheme::Oxy);
    // Create a window
    let mut wnd = window::Window::new(800, 200, wnd_width, wnd_height, "RustyPendulum");

    // Create a title at the top of the app
    let mut title = Frame::default().with_label("RustyPendulum");
    title.set_frame(FrameType::PlasticThinDownBox);
    title.set_label_font(Font::ScreenBold);
    title.set_label_size(30);
    title.set_label_color(dark_brown);
    let label_dimensions: (i32, i32) = title.measure_label();
    title.set_size(label_dimensions.0 + 5, label_dimensions.1);
    title.set_pos(0, menu_bar_height + 1); // Set below menu, but temp x value
    //let title_w = title.clone().w();
    let title_h = title.clone().h();
    title.set_color(graph_paper);
    title.center_x(&wnd);

    // Create a menubar
    let mut menubar = menu::MenuBar::default().with_size(wnd_width - 9, menu_bar_height);
    menubar.set_text_font(Font::Screen);
    menubar.set_text_size(20);
    menubar.set_text_color(dark_brown);
    menubar.set_frame(FrameType::PlasticThinDownBox);
    menubar.set_color(graph_paper);
    menubar.set_selection_color(rust_color);
    menubar.add(
        "Controls/Play\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb
    );
    menubar.add(
        "Controls/Pause\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb
    );
    menubar.add(
        "Controls/Reset\t",
        Shortcut::None,
        menu::MenuFlag::MenuDivider,
        menu_cb
    );
    menubar.add(
        "Controls/Quit\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb
    );
    // Link single and double so that they can't be used together. 
    // Clicking one unclicks the other
    // Or use one button that toggles
    menubar.add(
        "Pendulums/Single pendulum\t",
        Shortcut::None,
        menu::MenuFlag::Toggle,
        menu_cb
    );
    menubar.add(
        "Pendulums/Double pendulum\t",
        Shortcut::None,
        menu::MenuFlag::Toggle | menu::MenuFlag::MenuDivider,
        menu_cb
    );
    menubar.add(
        "Pendulums/Small angle approximation\t",
        Shortcut::None,
        menu::MenuFlag::Toggle,
        menu_cb
    );
    menubar.add(
        "Pendulums/Euler method\t",
        Shortcut::None,
        menu::MenuFlag::Toggle,
        menu_cb
    );
    menubar.add(
        "Pendulums/Euler-Cromer method\t",
        Shortcut::None,
        menu::MenuFlag::Toggle,
        menu_cb
    );
    menubar.add(
        "Pendulums/Runge-Kutta method\t",
        Shortcut::None,
        menu::MenuFlag::Toggle,
        menu_cb
    );
    menubar.add(
        "Graphs/Energies\t",
        Shortcut::None,
        menu::MenuFlag::Toggle,
        menu_cb
    );
    menubar.add(
        "Graphs/Velocities\t",
        Shortcut::None,
        menu::MenuFlag::Toggle,
        menu_cb
    );
    menubar.add(
        "Learn/Pendulum problem\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb
    );
    menubar.add(
        "Learn/Pendulum approximations\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb
    );
    menubar.add(
        "Learn/Simulation app\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb
    );
    // Create a window to contain the simulation
    let mut pendulum_wnd = window::Window::default()
        .with_size(wnd_width - 50, wnd_height - title_h - menu_bar_height - 20)
        .with_pos(0, title_h + menu_bar_height - 2);
    pendulum_wnd.set_color(graph_paper);
    pendulum_wnd.end();
    pendulum_wnd.center_x(&wnd);

    // Window settings details
    wnd.set_color(dark_brown);
    wnd.make_resizable(false); // To prevent xmonad from tiling it
    wnd.end();
    wnd.show();
    // Run the application
    app.run().unwrap();

    Ok(())
}
