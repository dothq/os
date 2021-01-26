use std::error::Error;

use gtk::Builder;

pub trait Widget {
    /**
     * Create the widget and return Self
     */
    fn new() -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    /**
     * Set the location of the widget on the screen
     */
    fn pin(x: i32, width: i32, height: i32) -> Result<(), Box<dyn Error>>;

    /**
     * Add all of the interactions for the widget
     */
    fn add_interactions(builder: &Builder) -> Result<(), Box<dyn Error>>;

    /**
     *  On gtk tick (or equivalent)
     */
    fn tick() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
