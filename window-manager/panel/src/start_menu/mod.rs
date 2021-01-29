use std::{thread, time::Duration};

use gdk::WindowTypeHint;
use gtk::{prelude::*, ApplicationWindow, Button};

use crate::{widget::Widget, HEIGHT, PADDING};

pub struct StartMenu {
    window: ApplicationWindow,
}

impl Widget for StartMenu {
    fn new(
        builder: &gtk::Builder,
        app: &gtk::Application,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let window: ApplicationWindow = builder.get_object("start_menu").unwrap();
        window.set_application(Some(app));
        window.set_type_hint(WindowTypeHint::Dock);

        Ok(StartMenu { window })
    }

    fn pin(&self, x: i32, _width: i32, height: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.window.show_all();

        let start_menu_size = self.window.get_size();

        self.window.move_(
            x + PADDING,
            height - HEIGHT - start_menu_size.1 - PADDING * 5,
        );

        self.window.set_visible(false);

        Ok(())
    }

    fn add_interactions(&self, builder: &gtk::Builder) -> Result<(), Box<dyn std::error::Error>> {
        let open_start_menu: Button = builder.get_object("open_start_menu").unwrap();
        let start_menu = self.window.clone();

        open_start_menu.connect_clicked(move |_| start_menu.set_visible(!start_menu.is_visible()));

        Ok(())
    }
}
