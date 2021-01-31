use std::{process::Command, thread, time::Duration};

use gtk::{
    prelude::*, ApplicationWindow, Button, IconSize, Image, Justification, Label, Orientation,
};
use regex::Regex;

use crate::{widget::Widget, HEIGHT, PADDING};

pub use self::helpers::{get_system_apps, SystemApps};

mod helpers;

pub struct StartMenu {
    window: ApplicationWindow,
    apps: ApplicationWindow,
}

impl StartMenu {
    pub fn add_apps(builder: &gtk::Builder, apps_hash: SystemApps) {
        println!("Adding apps");
        let exec_match = Regex::new(r"%(f|F|u|U|d|D|n|N|i|c|k|v|m)").unwrap();

        let mut apps = Vec::new();
        for (catagories, app) in apps_hash {
            apps.push((catagories, app));
        }

        apps.sort_by(|a, b| {
            a.0.to_string()
                .to_lowercase()
                .cmp(&b.0.to_string().to_lowercase())
        });
        apps.iter_mut().for_each(|category| {
            category.1.sort_by(|a, b| {
                a.0.to_string()
                    .to_lowercase()
                    .cmp(&b.0.to_string().to_lowercase())
            })
        });

        let grid: gtk::FlowBox = builder.get_object("app_catagories").unwrap();

        for item in grid.get_children() {
            grid.remove(&item);
        }

        for (category, apps) in apps {
            let label = Label::new(Some(&category.to_string()));
            label.set_focus_on_click(false);
            grid.add(&label);
            let clear = Label::new(None);
            clear.set_focus_on_click(false);
            grid.add(&clear);

            for app in &apps {
                let image = Image::from_icon_name(Some(&app.1), IconSize::Dialog);
                image.set_pixel_size(64);

                let label = Label::new(Some(&app.0));
                label.set_line_wrap(true);
                label.set_max_width_chars(15);
                label.set_justify(Justification::Center);

                let button = Button::new();
                button.set_hexpand(false);
                button.set_property_width_request(80);
                button.get_style_context().add_class("start_apps_button");

                let button_box = gtk::Box::new(Orientation::Vertical, 0);

                button.add(&button_box);
                button_box.add(&image);
                button_box.add(&label);

                grid.add(&button);

                let exec = exec_match.replace_all(&app.2, "").to_string();
                button.connect_clicked(move |_| {
                    println!("{}", exec);
                    let _ = Command::new("sh")
                        .args(vec!["-c", &exec])
                        .spawn()
                        .expect("Bad times launching that app");
                });
            }

            if apps.len() % 2 != 0 {
                // There are an odd number of apps, we need to make it even
                let clear = Label::new(None);
                clear.set_focus_on_click(false);
                grid.add(&clear);
            }
        }
        grid.show_all();
    }
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

        let apps: ApplicationWindow = builder.get_object("start_apps").unwrap();
        window.set_application(Some(app));

        Ok(StartMenu { window, apps })
    }

    fn pin(&self, x: i32, _width: i32, height: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Set position for the start menu
        self.window.show_all();
        let start_menu_size = self.window.get_size();

        self.window.move_(
            x + PADDING,
            height - HEIGHT - start_menu_size.1 - PADDING * 5,
        );

        self.window.set_visible(false);

        // Set the position for the apps display
        self.apps.show_all();

        thread::sleep(Duration::from_millis(10));

        let apps_size = self.apps.get_size();

        self.apps.move_(
            x + PADDING * 2 + start_menu_size.0,
            height - HEIGHT - apps_size.1 - PADDING * 5,
        );

        self.apps.set_visible(false);

        Ok(())
    }

    fn add_interactions(&self, builder: &gtk::Builder) -> Result<(), Box<dyn std::error::Error>> {
        let open_start_menu: Button = builder.get_object("open_start_menu").unwrap();
        let start_menu = self.window.clone();
        let apps = self.apps.clone();

        open_start_menu.connect_clicked(move |_| {
            start_menu.set_visible(!start_menu.is_visible());
            if !start_menu.is_visible() {
                apps.set_visible(false);
            }
        });

        let apps = self.apps.clone();
        let open_apps: Button = builder.get_object("open_apps").unwrap();

        open_apps.connect_clicked(move |_| {
            // get_system_apps().unwrap();
            apps.set_visible(!apps.is_visible())
        });

        Ok(())
    }
}
