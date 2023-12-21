pub mod common;
pub mod day;
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn get_implementation_for_day_or_panic(day: i32) -> Box<dyn day::Day> {
    return match day {
        1 => Box::new(day1::Day1 {}),
        2 => Box::new(day2::Day2 {}),
        3 => Box::new(day3::Day3 {}),
        4 => Box::new(day4::Day4 {}),
        5 => Box::new(day5::Day5 {}),
        6 => Box::new(day6::Day6 {}),
        7 => Box::new(day7::Day7 {}),
        8 => Box::new(day8::Day8 {}),
        9 => Box::new(day9::Day9 {}),
        10 => Box::new(day10::Day10 {}),
        11 => Box::new(day11::Day11 {}),
        12 => Box::new(day12::Day12 {}),
        13 => Box::new(day13::Day13 {}),
        14 => Box::new(day14::Day14 {}),
        15 => Box::new(day15::Day15 {}),
        16 => Box::new(day16::Day16 {}),
        17 => Box::new(day17::Day17 {}),
        18 => Box::new(day18::Day18 {}),
        19 => Box::new(day19::Day19 {}),
        20 => Box::new(day20::Day20 {}),
        21 => Box::new(day21::Day21 {}),
        _ => panic!("No implementation for provided day!"),
    };
}
