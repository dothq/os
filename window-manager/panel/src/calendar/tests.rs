use super::{day_of_week, days_in_month};

#[test]
fn days_in_month_test() {
    assert_eq!(days_in_month(1, 2021), 31);
    assert_eq!(days_in_month(2, 2021), 28);
    assert_eq!(days_in_month(2, 2020), 29);
}

#[test]
fn day_of_week_test() {
    assert_eq!(day_of_week(17, 1, 2021), 0); // Sunday
    assert_eq!(day_of_week(18, 1, 2021), 1); // Monday
    assert_eq!(day_of_week(19, 1, 2021), 2); // Tuesday
    assert_eq!(day_of_week(20, 1, 2021), 3); // Wednesday
    assert_eq!(day_of_week(21, 1, 2021), 4); // Thursday
    assert_eq!(day_of_week(22, 1, 2021), 5); // Friday
    assert_eq!(day_of_week(23, 1, 2021), 6); // Saturday
}
