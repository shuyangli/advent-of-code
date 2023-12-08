pub mod day;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub fn get_implementation_for_day_or_panic(day: i32) -> Box<dyn day::Day> {
    return match day {
        1 => Box::new(day1::Day1 {}),
        2 => Box::new(day2::Day2 {}),
        3 => Box::new(day3::Day3 {}),
        4 => Box::new(day4::Day4 {}),
        5 => Box::new(day5::Day5 {}),
        6 => Box::new(day6::Day6 {}),
        7 => Box::new(day7::Day7 {}),
        _ => panic!("No implementation for provided day!"),
    };
}
