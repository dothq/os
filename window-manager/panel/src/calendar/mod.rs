use chrono::{Datelike, Local};
use gdk::WindowTypeHint;
use gtk::{prelude::*, ApplicationWindow, AspectFrame, Button, Label};

use crate::{widget::Widget, HEIGHT, PADDING};

#[cfg(test)]
mod tests;

pub struct Calendar {
    window: ApplicationWindow,
    day: Label,
    date: Label,
}

impl Calendar {
    fn build(&self, builder: &gtk::Builder) {
        let date = Local::now().date().naive_local();

        let month = date.month() as usize;
        let year = date.year() as usize;
        let day = date.day() as usize;

        self.day.set_text(&date.format("%A").to_string());
        self.date.set_text(&date.format("%d %B %Y").to_string());

        let headers = vec!["S", "M", "T", "W", "T", "F", "S"]
            .iter()
            .map(|e| (format!("{} ", e), false))
            .collect();
        let mut calendar = vec![headers, vec![]];

        for _ in 0..day_of_week(0, month, year) + 1 {
            calendar
                .last_mut()
                .unwrap()
                .push((String::from("  "), false));
        }

        for d in 1..days_in_month(month, year) {
            let p = match d {
                0..=9 => format!("0{}", d),
                _ => d.to_string(),
            };

            let mut today = false;

            if d == day {
                today = true;
            }

            calendar.last_mut().unwrap().push((p, today));

            if day_of_week(d, month, year) == 6 {
                calendar.push(Vec::new());
            }
        }

        if calendar.last().unwrap().len() == 0 {
            calendar.pop();
        }

        let grid = builder
            .get_object::<gtk::Grid>("calendar_container")
            .unwrap();

        for (i, week) in calendar.iter().enumerate() {
            for (j, day) in week.iter().enumerate() {
                let label = Label::new(Some(&day.0));
                let aspect_frame = AspectFrame::new(None, 0.5, 0.5, 1.0, false);
                aspect_frame.add(&label);

                aspect_frame.get_style_context().add_class("flat");

                // If the day is today
                if day.1 {
                    // Add class calendar-today
                    label.get_style_context().add_class("calendar-today")
                }

                grid.attach(&aspect_frame, j as i32, i as i32, 1, 1);
            }
        }
    }
}

impl Widget for Calendar {
    fn new(
        builder: &gtk::Builder,
        app: &gtk::Application,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let window: ApplicationWindow = builder.get_object("calendar").unwrap();
        window.set_application(Some(app));
        window.set_type_hint(WindowTypeHint::Dock);

        let calender = Calendar {
            window,
            day: builder.get_object("calendar_day").unwrap(),
            date: builder.get_object("calendar_date").unwrap(),
        };

        calender.build(&builder);

        Ok(calender)
    }

    fn pin(&self, _x: i32, width: i32, height: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.window.show_all();

        let calendar_menu_size = self.window.get_size();

        self.window.move_(
            width - PADDING - calendar_menu_size.0,
            height - HEIGHT - calendar_menu_size.1 - PADDING * 5,
        );

        self.window.set_visible(false);

        Ok(())
    }

    fn add_interactions(&self, builder: &gtk::Builder) -> Result<(), Box<dyn std::error::Error>> {
        let open_calendar: Button = builder.get_object("open_calendar").unwrap();
        let calendar = self.window.clone();

        open_calendar.connect_clicked(move |_| calendar.set_visible(!calendar.is_visible()));

        Ok(())
    }
}

pub fn days_in_month(month: usize, year: usize) -> usize {
    match month {
        2 => {
            if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
                29
            } else {
                28
            }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 30,
    }
}

pub fn day_of_week(day: usize, month: usize, year: usize) -> usize {
    let a = (14 - month) / 12;
    let y = year - a;
    let m = month + (12 * a) - 2;

    (day + y + (y / 4) - (y / 100) + (y / 400) + ((31 * m) / 12)) % 7
}
