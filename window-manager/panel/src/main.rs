// Important note: I am using the lld linker because it compiles faster, you will need to install it manually

extern crate gdk;
// extern crate gio;
// extern crate glib;
extern crate gtk;

use std::{
    env::args,
    error::Error,
    thread::{self, JoinHandle},
    time::Duration,
};

use gdk::WindowTypeHint;
use gio::prelude::*;
use glib::MainContext;
use gtk::{prelude::*, Application, ApplicationWindow, Builder, StyleContext};

use calendar::Calendar;
use start_menu::{get_system_apps, StartMenu, SystemApps};
use widget::Widget;

mod calendar;
mod start_menu;
mod widget;

const RESOURCE_PREFIX: &str = "/co/dothq/os/panel";

pub const PADDING: i32 = 8;
pub const HEIGHT: i32 = 16;

fn resource(name: &str) -> String {
    format!("{}/{}", RESOURCE_PREFIX, name)
}

enum ThreadMessage {
    AppsUpdate(SystemApps),
}

struct Panel {
    window: ApplicationWindow,
    start_menu: StartMenu,
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
        let start_menu = StartMenu::new(&builder, &app).unwrap();
        let calendar = Calendar::new(&builder, app).unwrap();

        Panel {
            window,
            builder,
            calendar,
            start_menu,
        }
    }

    fn pin(&self) {
        let screen = self.window.get_screen().unwrap();

        let current_monitor = screen.get_display().get_primary_monitor().unwrap();
        let current_monitor_geometry = current_monitor.get_geometry();

        let x = current_monitor_geometry.x;
        let width = current_monitor_geometry.width;
        let height = current_monitor_geometry.height;

        self.window.move_(x + PADDING, height - HEIGHT - 32);
        self.window.resize(width - PADDING * 2, HEIGHT);
        self.window.show_all();

        self.start_menu.pin(x, width, height).unwrap();
        self.calendar.pin(x, width, height).unwrap();
    }

    fn add_interactions(&self) {
        let builder = &self.builder;

        self.start_menu.add_interactions(builder).unwrap();
        self.calendar.add_interactions(builder).unwrap();
    }

    fn tick_start(self) -> JoinHandle<()> {
        // Sender and receiver context that allows us to pass data between threads
        let (low_sender, low_receiver) = MainContext::channel(glib::PRIORITY_LOW);

        // So we can't do most of our code on this thread because you can only access
        // gtk libraries on the main thread, but we can do a bunch of the work on
        // this thread and then send the data back go another thread. So that is
        // what we are going to do, because it works
        let thread = thread::spawn(move || loop {
            // Do tick stuff here
            low_sender
                .send(ThreadMessage::AppsUpdate(get_system_apps().unwrap()))
                .unwrap();

            thread::sleep(Duration::from_secs(30));
        });

        let receiver_builder = self.builder.clone();
        low_receiver.attach(None, move |msg| {
            match msg {
                ThreadMessage::AppsUpdate(apps) => StartMenu::add_apps(&receiver_builder, apps),
            };

            glib::Continue(true)
        });

        thread
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let application = gtk::Application::new(Some("co.dothq.os.panel"), Default::default())?;

    // let mut threads = Vec::new();

    // let active_threads = &mut threads;
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
        panel.tick_start();
    });

    application.run(&args().collect::<Vec<_>>());

    unreachable!()
}
