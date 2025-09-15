use fltk::prelude::*;
use fltk::menu;
use fltk::app;
use fltk::enums::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::world;
use crate::pendulum;

fn control_callback(m: &mut impl MenuExt) {
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            // TODO: Do stuff
            "Play\t" => {},
            "Pause\t" => {},
            "Reset\t" => {},
            "Quit\t" => { app::quit(); },
            _ => {}
        }
    }
}

pub struct TopMenuBar {
    menubar_widget: menu::MenuBar,
    bg_color: Color,
    text_color: Color,
    sel_color: Color,
    //width: i32,
    //height: i32
}

impl TopMenuBar {
    pub fn new(width: i32, height: i32, bg_color: Color, text_color: Color, sel_color: Color) -> Self {
        let menubar = menu::MenuBar::default().with_size(width, height); 
        Self {
            menubar_widget: menubar,
            bg_color: bg_color,
            text_color: text_color,
            sel_color: sel_color,
            //width: width,
            //height: height
        }
    }

    pub fn setup(&mut self, world: &mut Rc<RefCell<world::World>>) {
        self.menubar_widget.set_text_font(Font::Screen);
        self.menubar_widget.set_text_size(20);
        self.menubar_widget.set_text_color(self.text_color);
        self.menubar_widget.set_frame(FrameType::PlasticThinDownBox);
        self.menubar_widget.set_color(self.bg_color);
        self.menubar_widget.set_selection_color(self.sel_color);
        // Control
        self.menubar_widget.add(
            "Controls/Play\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            {
                let world_clone = world.clone();
                move |_| {
                    world_clone.borrow_mut().play = true;
                    world_clone.borrow_mut().started = true;
                    if world_clone.borrow_mut().systems.len() > 0 {
                        world_clone.borrow_mut().initialize();
                    }
                }
            }
        );
        self.menubar_widget.add(
            "Controls/Pause\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            {
                let world_clone = world.clone();
                move |_| {
                    world_clone.borrow_mut().play = false;
                }
            }

        );
        self.menubar_widget.add(
            "Controls/Reset\t",
            Shortcut::None,
            menu::MenuFlag::MenuDivider,
            {
                let world_clone = world.clone();
                move |_| {
                    // todo: reinitialize pendulums
                    world_clone.borrow_mut().play = false;
                    world_clone.borrow_mut().started = false;
                }
            }

        );
        self.menubar_widget.add(
            "Controls/Quit\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            {
                move |_| {
                    app::quit();
                }
            }

        );
        // Pendulums
        self.menubar_widget.add(
            "Pendulums/Small angle approximation\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            {
                let world_clone = world.clone();
                let pendulum_length = world_clone.borrow().len as f32;
                move |_| {
                    world::World::add_remove_system(pendulum_length,
                        pendulum::ApproximationMethods::SmallAngle,
                        &mut world_clone.borrow_mut().systems);
                }
            }
        );
        self.menubar_widget.add(
            "Pendulums/Euler method\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            {
                let world_clone = world.clone();
                let pendulum_length = world_clone.borrow().len as f32;
                move |_| {
                    world::World::add_remove_system(pendulum_length, 
                        pendulum::ApproximationMethods::Euler,
                        &mut world_clone.borrow_mut().systems);
                }
            }
        );
        self.menubar_widget.add(
            "Pendulums/Euler-Cromer method\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            {
                let world_clone = world.clone();
                let pendulum_length = world_clone.borrow().len as f32;
                move |_| {
                    world::World::add_remove_system(pendulum_length, 
                        pendulum::ApproximationMethods::EulerCromer,
                        &mut world_clone.borrow_mut().systems);
                }
            }
        );
        self.menubar_widget.add(
            "Pendulums/Runge-Kutta method\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            {
                let world_clone = world.clone();
                let pendulum_length = world_clone.borrow().len as f32;
                move |_| {
                    world::World::add_remove_system(pendulum_length, 
                        pendulum::ApproximationMethods::RungeKutta,
                        &mut world_clone.borrow_mut().systems);
                }
            }
        );
        // Graphs
        self.menubar_widget.add(
            "Graphs/Energies\t",
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            control_callback // temporary: does nothing
        );
        self.menubar_widget.add(
            "Graphs/Velocities\t",
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            control_callback // temporary: does nothing
        );
        // Learn
        self.menubar_widget.add(
            "Learn/Pendulum problem\t",
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            control_callback // temporary: does nothing
        );
        self.menubar_widget.add(
            "Learn/Pendulum approximations\t",
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            control_callback // temporary: does nothing
        );
        self.menubar_widget.add(
            "Learn/Simulation app\t", // Is this menu option even necessary?
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            control_callback // temporary: does nothing
        );
    }
}
