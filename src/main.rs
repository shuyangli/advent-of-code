use std::env;
use std::fs;

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

// To run: cargo run -- {x} where x is the day number.
fn main() {
    let day_number = read_day_number_or_panic();
    println!("Advent of Code, Day {day_number}");

    let input = read_input_or_panic(day_number);
    let day = rust_advent_2023::get_implementation_for_day_or_panic(day_number);

    match day.part1(input.as_str()) {
        Ok(answer) => println!("Part 1 Answer: {answer}"),
        Err(msg) => println!("Part 1 Error! {msg}"),
    };
    match day.part2(input.as_str()) {
        Ok(answer) => println!("Part 2 Answer: {answer}"),
        Err(msg) => println!("Part 2 Error! {msg}"),
    };
}
