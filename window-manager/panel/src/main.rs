extern crate gdk;
extern crate gtk;

use std::error::Error;

use gdk::{Rectangle, WindowTypeHint};
use gtk::{prelude::*, Builder, Button, Window};

const GLADE_SRC: &str = include_str!("layout.glade");

const PADDING: i32 = 8;
const HEIGHT: i32 = 16;

struct Panel {
    window: Window,
    start_menu: Window,
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

        let start_menu: Window = builder.get_object("start_menu").unwrap();

        Panel {
            window,
            builder,
            start_menu,
        }
    }

    fn pin(&self) {
        let screen = self.window.get_screen().unwrap();

        let current_monitor = screen.get_display().get_primary_monitor().unwrap();
        let current_monitor_geometry = current_monitor.get_geometry();

        let x = current_monitor_geometry.x;
        let y = current_monitor_geometry.y;
        let width = current_monitor_geometry.width;
        let height = current_monitor_geometry.height;

        println!("mon: x {} y {} width {} height: {}", x, y, width, height);

        self.window.move_(x + PADDING, height - HEIGHT - 32);
        self.window.resize(width - PADDING * 2, HEIGHT);

        let start_menu_size = self.start_menu.get_size();

        self.start_menu
            .move_(x + PADDING, height - HEIGHT - 48 - start_menu_size.1);

        self.window.show_all();
        self.start_menu.show_all();

        self.start_menu.set_visible(false);
    }

    fn add_interactions(&self) {
        let mut start_menu_open = false;

        let open_start_menu: Button = self.builder.get_object("open_start_menu").unwrap();
        let start_menu = self.start_menu.clone();

        open_start_menu.connect_clicked(move |_| {
            println!("Start menu");
            // start_menu.

            start_menu.set_visible(!start_menu.is_visible());
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let panel = Panel::new();
    panel.pin();
    panel.add_interactions();

    gtk::main();

    Ok(())
}
