pub mod day;
pub mod day1;
pub mod day2;

pub fn get_implementation_for_day_or_panic(day: i32) -> Box<dyn day::Day> {
    return match day {
        1 => Box::new(day1::Day1 {}),
        2 => Box::new(day2::Day2 {}),
        _ => panic!("No implementation for provided day!"),
    };
}
