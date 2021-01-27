use std::error::Error;

use gtk::{Application, Builder};

pub trait Widget {
    /**
     * Create the widget and return Self
     */
    fn new(builder: &Builder, app: &Application) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    /**
     * Set the location of the widget on the screen
     */
    fn pin(&self, x: i32, width: i32, height: i32) -> Result<(), Box<dyn Error>>;

    /**
     * Add all of the interactions for the widget
     */
    fn add_interactions(&self, builder: &Builder) -> Result<(), Box<dyn Error>>;

    /**
     *  On gtk tick (or equivalent)
     */
    fn tick(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
