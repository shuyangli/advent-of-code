use crate::day::Day;
use std::fmt::Display;

pub struct Day5 {}

#[derive(Debug)]
struct Range {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

#[derive(Default, Debug)]
struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    fn get_range_containing(&self, input: u64) -> Option<&Range> {
        for range in &self.ranges {
            if input >= range.source_start && input < range.source_start + range.length {
                return Some(range);
            }
        }
        return None;
    }

    fn get_destination(&self, input: u64) -> u64 {
        if let Some(range) = self.get_range_containing(input) {
            return range.dest_start + (input - range.source_start);
        }
        return input;
    }
}

#[derive(Default, Debug)]
struct Almanac {
    seed_to_soil: Ranges,
    soil_to_fertilizer: Ranges,
    fertilizer_to_water: Ranges,
    water_to_light: Ranges,
    light_to_temperature: Ranges,
    temperature_to_humidity: Ranges,
    humidity_to_location: Ranges,
}

impl Day for Day5 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let seeds: Vec<u64> = parse_seeds(input);
        let almanac = parse_almanac_or_panic(input);

        let lowest_location = seeds
            .iter()
            .map(|seed| almanac.seed_to_soil.get_destination(*seed))
            .map(|soil| almanac.soil_to_fertilizer.get_destination(soil))
            .map(|fertilizer| almanac.fertilizer_to_water.get_destination(fertilizer))
            .map(|water| almanac.water_to_light.get_destination(water))
            .map(|light| almanac.light_to_temperature.get_destination(light))
            .map(|temperature| almanac.temperature_to_humidity.get_destination(temperature))
            .map(|humidity| almanac.humidity_to_location.get_destination(humidity))
            .min();

        return Ok(Box::new(lowest_location.unwrap()));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let seeds: Vec<u64> = parse_seeds(input);
        let almanac = parse_almanac_or_panic(input);

        let lowest_location = seeds
            .iter()
            .step_by(2)
            .zip(seeds.iter().skip(1).step_by(2))
            .map(|(seed_start, length)| {
                let range = *seed_start..(seed_start + length);
                return range
                    .map(|seed| almanac.seed_to_soil.get_destination(seed))
                    .map(|soil| almanac.soil_to_fertilizer.get_destination(soil))
                    .map(|fertilizer| almanac.fertilizer_to_water.get_destination(fertilizer))
                    .map(|water| almanac.water_to_light.get_destination(water))
                    .map(|light| almanac.light_to_temperature.get_destination(light))
                    .map(|temperature| almanac.temperature_to_humidity.get_destination(temperature))
                    .map(|humidity| almanac.humidity_to_location.get_destination(humidity))
                    .min()
                    .unwrap();
            })
            .min();

        return Ok(Box::new(lowest_location.unwrap()));
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect()
}

fn parse_almanac_or_panic(input: &str) -> Almanac {
    let mut almanac = Almanac::default();
    let mut lines = input.lines();

    lines.next();
    lines.next();
    lines.next();

    while let Some(range) = lines.next().and_then(|l| parse_range(l)) {
        almanac.seed_to_soil.ranges.push(range);
    }
    lines.next();
    while let Some(range) = lines.next().and_then(|l| parse_range(l)) {
        almanac.soil_to_fertilizer.ranges.push(range);
    }
    lines.next();
    while let Some(range) = lines.next().and_then(|l| parse_range(l)) {
        almanac.fertilizer_to_water.ranges.push(range);
    }
    lines.next();
    while let Some(range) = lines.next().and_then(|l| parse_range(l)) {
        almanac.water_to_light.ranges.push(range);
    }
    lines.next();
    while let Some(range) = lines.next().and_then(|l| parse_range(l)) {
        almanac.light_to_temperature.ranges.push(range);
    }
    lines.next();
    while let Some(range) = lines.next().and_then(|l| parse_range(l)) {
        almanac.temperature_to_humidity.ranges.push(range);
    }
    lines.next();
    while let Some(range) = lines.next().and_then(|l| parse_range(l)) {
        almanac.humidity_to_location.ranges.push(range);
    }

    return almanac;
}

fn parse_range(line: &str) -> Option<Range> {
    if line.is_empty() {
        return None;
    }

    let range_values: Vec<u64> = line
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    return Some(Range {
        dest_start: range_values[0],
        source_start: range_values[1],
        length: range_values[2],
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
