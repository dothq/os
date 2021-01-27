extern crate chrono;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use std::{env::args, error::Error};

use gdk::WindowTypeHint;
use gio::prelude::*;
use gtk::{prelude::*, Application, ApplicationWindow, Builder, Button, StyleContext};

use calendar::Calendar;
use widget::Widget;

mod calendar;
mod widget;

const RESOURCE_PREFIX: &str = "/co/dothq/os/panel";

pub const PADDING: i32 = 8;
pub const HEIGHT: i32 = 16;

fn resource(name: &str) -> String {
    format!("{}/{}", RESOURCE_PREFIX, name)
}

struct Panel {
    window: ApplicationWindow,
    start_menu: ApplicationWindow,
    calendar: Calendar,
    builder: Builder,
}

impl Panel {
    fn new(app: &Application) -> Self {
        // Initialize the window
        let builder = Builder::from_resource(&resource("panel.glade"));
        let window: ApplicationWindow = builder.get_object("panel").unwrap();

        window.set_application(Some(app));
        window.set_skip_taskbar_hint(true);
        window.set_decorated(false);
        window.set_type_hint(WindowTypeHint::Dock);

        // Start menu window
        let start_menu: ApplicationWindow = builder.get_object("start_menu").unwrap();
        start_menu.set_application(Some(app));
        start_menu.set_skip_taskbar_hint(true);
        start_menu.set_type_hint(WindowTypeHint::Dock);

        let calendar = Calendar::new(&builder, app).unwrap();

        Panel {
            window,
            builder,
            calendar,
            start_menu,
        }
    }

    fn pin(&self) {
        self.build_calendar();

        let screen = self.window.get_screen().unwrap();

        let current_monitor = screen.get_display().get_primary_monitor().unwrap();
        let current_monitor_geometry = current_monitor.get_geometry();

        let x = current_monitor_geometry.x;
        let width = current_monitor_geometry.width;
        let height = current_monitor_geometry.height;

        self.window.move_(x + PADDING, height - HEIGHT - 32);
        self.window.resize(width - PADDING * 2, HEIGHT);

        let start_menu_size = self.start_menu.get_size();

        self.start_menu.move_(
            x + PADDING,
            height - HEIGHT - start_menu_size.1 * 2 - PADDING,
        );

        self.window.show_all();
        self.start_menu.show_all();

        self.start_menu.set_visible(false);

        self.calendar.pin(x, width, height).unwrap();
    }

    fn build_calendar(&self) {}

    fn add_interactions(&self) {
        let open_start_menu: Button = self.builder.get_object("open_start_menu").unwrap();
        let start_menu = self.start_menu.clone();

        open_start_menu.connect_clicked(move |_| start_menu.set_visible(!start_menu.is_visible()));

        self.calendar.add_interactions(&self.builder).unwrap();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let application = gtk::Application::new(Some("co.dothq.os.panel"), Default::default())?;

    application.connect_activate(move |app| {
        // Load compiled resources
        let resources_bytes = include_bytes!("../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);

        // Load application css
        let style = gtk::CssProvider::new();
        style.load_from_resource(&resource("panel.css"));
        StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing css provider"),
            &style,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let panel = Panel::new(&app);
        panel.pin();
        panel.add_interactions();
    });

    application.run(&args().collect::<Vec<_>>());

    unreachable!()
}
