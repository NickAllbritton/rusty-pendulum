use fltk::prelude::*;
use fltk::menu;
use fltk::app;
use fltk::enums::*;

fn menu_cb(m: &mut impl MenuExt) {
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

    pub fn setup(&mut self) {
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
            menu_cb
        );
        self.menubar_widget.add(
            "Controls/Pause\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            menu_cb
        );
        self.menubar_widget.add(
            "Controls/Reset\t",
            Shortcut::None,
            menu::MenuFlag::MenuDivider,
            menu_cb
        );
        self.menubar_widget.add(
            "Controls/Quit\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            menu_cb
        );
        // Pendulums
        self.menubar_widget.add(
            "Pendulums/Small anle approximation\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            menu_cb
        );
        self.menubar_widget.add(
            "Pendulums/Euler method\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            menu_cb
        );
        self.menubar_widget.add(
            "Pendulums/Euler-Cromer method\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            menu_cb
        );
        self.menubar_widget.add(
            "Pendulums/Runge-Kutta method\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            menu_cb
        );
        // Graphs
        self.menubar_widget.add(
            "Graphs/Energies\t",
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            menu_cb
        );
        self.menubar_widget.add(
            "Graphs/Velocities\t",
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            menu_cb
        );
        // Learn
        self.menubar_widget.add(
            "Learn/Pendulum problem\t",
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            menu_cb
        );
        self.menubar_widget.add(
            "Learn/Pendulum approximations\t",
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            menu_cb
        );
        self.menubar_widget.add(
            "Learn/Simulation app\t", // Is this menu option even necessary?
            Shortcut::None,
            menu::MenuFlag::Normal, // maybe Toggle?
            menu_cb
        );
    }
}
