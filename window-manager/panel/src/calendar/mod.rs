use gtk::ApplicationWindow;

#[cfg(test)]
mod tests;

pub struct Calendar {
    window: ApplicationWindow,
}

impl Calendar {}

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
