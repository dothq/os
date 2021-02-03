use gtk::{prelude::*, ApplicationWindow, Button, Scale};

use crate::{widget::Widget, HEIGHT, PADDING};

use self::helpers::{get_volume, set_volume};

mod helpers;

#[derive(Clone)]
pub struct Volume {
    window: ApplicationWindow,
}

impl Widget for Volume {
    fn new(
        builder: &gtk::Builder,
        app: &gtk::Application,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let window: ApplicationWindow = builder.get_object("volume").unwrap();
        window.set_application(Some(app));

        Ok(Volume { window })
    }

    fn pin(&self, x: i32, width: i32, height: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.window.show_all();

        let calendar_menu_size = self.window.get_size();

        self.window.move_(
            x + width - PADDING - calendar_menu_size.0 - 36,
            height - HEIGHT - calendar_menu_size.1 - PADDING * 5,
        );

        self.window.set_visible(false);

        Ok(())
    }

    fn add_interactions(&self, builder: &gtk::Builder) -> Result<(), Box<dyn std::error::Error>> {
        let open_volume: Button = builder.get_object("open_volume").unwrap();
        let volume_slider: Scale = builder.get_object("volume_slider").unwrap();
        let volume = self.window.clone();

        open_volume.connect_clicked(move |_| {
            let current_volume = get_volume().expect(
                "Error retrieving system volume. Make sure that you have pulse audio installed",
            );

            volume_slider.set_value(current_volume as f64);

            volume.set_visible(!volume.is_visible())
        });

        let volume_slider: Scale = builder.get_object("volume_slider").unwrap();

        volume_slider.connect_value_changed(move |volume_slider| {
            set_volume(volume_slider.get_value() as u8)
                .expect("Error settings volume. Make sure that you have pulse audio installed");
        });

        Ok(())
    }
}
