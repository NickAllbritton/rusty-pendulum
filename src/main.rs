use fltk::prelude::*;
use fltk::window;
use fltk::app;
use fltk::frame::Frame;
use fltk::enums::*;

use std::rc::Rc;
use std::cell::RefCell;

mod menu_bar;
mod world;
mod pendulum;
mod physics;

//const APP_ID: &str = "org.physics_sim.rusty_pendulum";

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

    // Create a window to contain the simulation
    let mut pendulum_world = world::World::new((wnd_width - 50, wnd_height - title_h - menu_bar_height - 20),
                    graph_paper, dark_brown);
    pendulum_world.setup(&wnd, title_h + menu_bar_height - 2);
    //pendulum_world.draw();

    // Create a refcell to contain the world so it can be borrowed safely by the menubar
    let mut world_cell: Rc<RefCell<world::World>> = Rc::new(RefCell::new(pendulum_world));

    // Create a menubar
    let mut main_menu = menu_bar::TopMenuBar::new(wnd_width - 8, menu_bar_height, 
                                    graph_paper, dark_brown, rust_color);
    main_menu.setup(&mut world_cell);

        // Window settings details
    wnd.set_color(dark_brown);
    wnd.make_resizable(false); // To prevent xmonad from tiling it
    wnd.end();
    wnd.show();
    // Run the application
    //app.run().unwrap();

    while app.wait() {
        world_cell.borrow_mut().draw();
    }

    Ok(())
}
