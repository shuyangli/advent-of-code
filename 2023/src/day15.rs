use crate::day::Day;
use std::fmt::Display;

pub struct Day15 {}

#[derive(Clone, PartialEq, Eq)]
struct LensBox {
    label: String,
    focal_length: i32,
}

fn hash_string(input: &str) -> usize {
    let mut hash = 0;

    for char in input.chars() {
        if !char.is_ascii() {
            panic!("Char {char} is not ascii!");
        }
        hash += char as usize;
        hash *= 17;
        hash %= 256;
    }

    return hash;
}

impl Day for Day15 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let answer: usize = input.split(',').map(hash_string).sum::<usize>();
        return Ok(Box::new(answer));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut boxes: Vec<Vec<LensBox>> = vec![vec![]; 256];

        for instruction in input.split(',') {
            if instruction.ends_with('-') {
                let label = instruction.trim_end_matches('-');
                let box_idx = hash_string(label);

                if let Some((i, _)) = boxes[box_idx]
                    .iter()
                    .enumerate()
                    .find(|(_, lens_box)| lens_box.label == label)
                {
                    boxes[box_idx].remove(i);
                }
            } else if let Some((label, focal_length)) = instruction.split_once('=') {
                let box_idx = hash_string(label);
                let focal_length = focal_length.parse::<i32>().unwrap();

                if let Some((i, _)) = boxes[box_idx]
                    .iter()
                    .enumerate()
                    .find(|(_, lens_box)| lens_box.label == label)
                {
                    boxes[box_idx][i].focal_length = focal_length;
                } else {
                    boxes[box_idx].push(LensBox {
                        label: label.to_string(),
                        focal_length,
                    });
                }
            } else {
                panic!("Unknown instruction");
            }
        }

        let answer: i32 = boxes
            .iter()
            .enumerate()
            .map(|(i, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(j, lens)| (i as i32 + 1) * (j as i32 + 1) * lens.focal_length)
                    .sum::<i32>()
            })
            .sum();

        return Ok(Box::new(answer));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[googletest::test]
    fn hash_returns_correctly() {
        expect_that!(hash_string("HASH"), eq(52));
        expect_that!(hash_string("rn=1"), eq(30));
    }
}
