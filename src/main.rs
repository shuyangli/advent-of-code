use std::env;
use std::fs;

use day::{Day, DayResult};
use day1::Day1;

pub mod day;
pub mod day1;

fn read_day_number_or_panic() -> i32 {
    let mut args = env::args();
    return match args.nth(1) {
        Some(day) => day.parse().expect("Cannot parse day number"),
        None => panic!("No argument at index 1"),
    };
}

fn read_input_or_panic(day: i32) -> String {
    let filepath = format!("./src/inputs/day{day}");
    println!("Reading from input: {filepath}");
    return fs::read_to_string(filepath.as_str()).expect("Input file not found!");
}

fn get_implementation_for_day_or_panic(day: i32) -> Box<dyn Day> {
    return match day {
        1 => Box::new(Day1 {}),
        _ => panic!("No implementation for provided day!"),
    };
}

// To run: cargo run -- {x} where x is the day number.
fn main() {
    let day_number = read_day_number_or_panic();
    println!("Advent of Code, Day {day_number}");

    let input = read_input_or_panic(day_number);
    let day = get_implementation_for_day_or_panic(day_number);

    match day.part1(input.as_str()) {
        DayResult::Ok(answer) => println!("Part 1 Answer: {answer}"),
        DayResult::Err(msg) => println!("Part 1 Error! {msg}"),
    };
    match day.part2(input.as_str()) {
        DayResult::Ok(answer) => println!("Part 2 Answer: {answer}"),
        DayResult::Err(msg) => println!("Part 2 Error! {msg}"),
    };
}
