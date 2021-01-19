extern crate gdk;
extern crate gtk;

use std::error::Error;

use gdk::{Rectangle, WindowTypeHint};
use gtk::{prelude::*, Builder, Window};

const GLADE_SRC: &str = include_str!("layout.glade");

const PADDING: i32 = 8;
const HEIGHT: i32 = 16;

struct Panel {
    window: Window,
    builder: Builder,
}

impl Panel {
    fn new() -> Self {
        // Initialize the window
        let builder = Builder::from_string(GLADE_SRC);
        let window: Window = builder.get_object("panel").unwrap();

        window.set_skip_taskbar_hint(true);
        window.set_decorated(false);
        window.set_type_hint(WindowTypeHint::Dock);

        Panel { window, builder }
    }

    fn pin(&self) {
        let screen = self.window.get_screen().unwrap();
        let width = screen.get_width();
        println!("Width: {}", width);

        let current_monitor = screen.get_primary_monitor();
        let current_monitor: Rectangle = screen
            .get_display()
            .get_monitor(current_monitor)
            .unwrap()
            .get_geometry();

        let x = current_monitor.x;
        let y = current_monitor.y;
        let width = current_monitor.width;
        let height = current_monitor.height;

        self.window.move_(x, y);
        self.window.resize(width, HEIGHT);

        self.window.show_all();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let panel = Panel::new();
    panel.pin();

    gtk::main();

    Ok(())
}
