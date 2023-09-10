mod after;
use time::PrimitiveDateTime as DateTime;

fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    use time::{Date, Time};

    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}


fn main() {
    let start_date = dt(2023, 9, 10, 22,31, 50);
    let result = after::after(start_date);
    println!("Start time: {}", start_date);
    println!("Result After time: {}", result);
}