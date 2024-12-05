use crate::day::Day;
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

pub struct Day12 {}

struct HotSpring {
    chars: Vec<char>,
    numbers: Vec<usize>,
}

impl HotSpring {
    fn insert_into_entry(
        memoization: &mut HashMap<usize, HashMap<Vec<usize>, HashMap<bool, i64>>>,
        starting_idx: usize,
        numbers: &Vec<usize>,
        require_spring: bool,
        result: i64,
    ) -> i64 {
        *memoization
            .entry(starting_idx)
            .or_insert(HashMap::new())
            .entry(numbers.clone())
            .or_insert(HashMap::new())
            .entry(require_spring)
            .or_insert(result)
    }

    fn get_num_arrangements_helper(
        &self,
        memoization: &mut HashMap<usize, HashMap<Vec<usize>, HashMap<bool, i64>>>,
        starting_idx: usize,
        numbers: &Vec<usize>,
        require_spring: bool,
    ) -> i64 {
        if let Entry::Occupied(occupied) = memoization
            .entry(starting_idx)
            .or_insert(HashMap::new())
            .entry(numbers.clone())
            .or_insert(HashMap::new())
            .entry(require_spring)
        {
            return *occupied.get();
        }

        if starting_idx == self.chars.len() {
            if numbers.is_empty() || numbers.len() == 1 && numbers[0] == 0 {
                return Self::insert_into_entry(
                    memoization,
                    starting_idx,
                    numbers,
                    require_spring,
                    1,
                );
            } else {
                return Self::insert_into_entry(
                    memoization,
                    starting_idx,
                    numbers,
                    require_spring,
                    0,
                );
            }
        }

        match self.chars[starting_idx] {
            '.' => {
                if require_spring {
                    return 0;
                } else if !numbers.is_empty() && numbers[0] == 0 {
                    let new_numbers = numbers[1..].to_vec();
                    let result = self.get_num_arrangements_helper(
                        memoization,
                        starting_idx + 1,
                        &new_numbers,
                        /*require_spring=*/ false,
                    );
                    return Self::insert_into_entry(
                        memoization,
                        starting_idx,
                        numbers,
                        require_spring,
                        result,
                    );
                } else {
                    let result = self.get_num_arrangements_helper(
                        memoization,
                        starting_idx + 1,
                        numbers,
                        /*require_spring=*/ false,
                    );
                    return Self::insert_into_entry(
                        memoization,
                        starting_idx,
                        numbers,
                        require_spring,
                        result,
                    );
                }
            }
            '#' => {
                if numbers.is_empty() || numbers[0] == 0 {
                    return Self::insert_into_entry(
                        memoization,
                        starting_idx,
                        numbers,
                        require_spring,
                        0,
                    );
                } else {
                    let mut new_numbers = numbers.clone();
                    new_numbers[0] -= 1;

                    let result = self.get_num_arrangements_helper(
                        memoization,
                        starting_idx + 1,
                        &new_numbers,
                        /*require_spring=*/ new_numbers[0] != 0,
                    );
                    return Self::insert_into_entry(
                        memoization,
                        starting_idx,
                        numbers,
                        require_spring,
                        result,
                    );
                }
            }
            '?' => {
                if require_spring && (numbers.is_empty() || numbers[0] == 0) {
                    return Self::insert_into_entry(
                        memoization,
                        starting_idx,
                        numbers,
                        require_spring,
                        0,
                    );
                }
                if numbers.is_empty() {
                    // If there are no more numbers, this is '.', so there's only one option.
                    let result = self.get_num_arrangements_helper(
                        memoization,
                        starting_idx + 1,
                        numbers,
                        /*require_spring=*/ false,
                    );
                    return Self::insert_into_entry(
                        memoization,
                        starting_idx,
                        numbers,
                        require_spring,
                        result,
                    );
                } else if numbers[0] == 0 {
                    // If this needs to be the end of a sequence, it has to be '.', so there's only one option.
                    let new_numbers = numbers[1..].to_vec();
                    let result = self.get_num_arrangements_helper(
                        memoization,
                        starting_idx + 1,
                        &new_numbers,
                        /*require_spring=*/ false,
                    );
                    return Self::insert_into_entry(
                        memoization,
                        starting_idx,
                        numbers,
                        require_spring,
                        result,
                    );
                } else {
                    // Otherwise, it can be '.' or '#'.
                    let mut new_numbers: Vec<usize> = numbers.clone();
                    new_numbers[0] -= 1;
                    let options_for_space = if require_spring {
                        0
                    } else {
                        self.get_num_arrangements_helper(
                            memoization,
                            starting_idx + 1,
                            numbers,
                            /*require_spring=*/ false,
                        )
                    };
                    let options_for_spring = self.get_num_arrangements_helper(
                        memoization,
                        starting_idx + 1,
                        &new_numbers,
                        /*require_spring=*/ new_numbers[0] != 0,
                    );
                    return Self::insert_into_entry(
                        memoization,
                        starting_idx,
                        numbers,
                        require_spring,
                        options_for_space + options_for_spring,
                    );
                }
            }
            _ => panic!("Unexpected input character."),
        }
    }

    fn get_num_arrangements(&self) -> i64 {
        let mut memoization: HashMap<usize, HashMap<Vec<usize>, HashMap<bool, i64>>> =
            HashMap::new();

        self.get_num_arrangements_helper(
            &mut memoization,
            0,
            &self.numbers,
            /*require_spring=*/ false,
        )
    }
}

fn parse_spring_part1(line: &str) -> HotSpring {
    let mut items = line.split_ascii_whitespace();
    HotSpring {
        chars: items.next().unwrap().chars().collect(),
        numbers: items
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect(),
    }
}

fn parse_spring_part2(line: &str) -> HotSpring {
    let mut items = line.split_ascii_whitespace();
    let chars = items.next().unwrap();
    let chars = (0..5).map(|_| chars).collect::<Vec<&str>>().join("?");

    let numbers = items.next().unwrap();
    let numbers = (0..5).map(|_| numbers).collect::<Vec<&str>>().join(",");

    HotSpring {
        chars: chars.chars().collect(),
        numbers: numbers.split(',').map(|n| n.parse().unwrap()).collect(),
    }
}

impl Day for Day12 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let hot_springs: Vec<_> = input.lines().map(|l| parse_spring_part1(l)).collect();
        let num_arrangements: i64 = hot_springs
            .into_iter()
            .map(|spring| spring.get_num_arrangements())
            .sum();
        return Ok(Box::new(num_arrangements));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let hot_springs: Vec<_> = input.lines().map(|l| parse_spring_part2(l)).collect();
        let num_arrangements: i64 = hot_springs
            .into_iter()
            .map(|spring| spring.get_num_arrangements())
            .sum();
        return Ok(Box::new(num_arrangements));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    fn get_num_arrangements_helper(pattern: &str, numbers: Vec<usize>) -> i64 {
        let chars: Vec<_> = pattern.chars().collect();
        return HotSpring { chars, numbers }.get_num_arrangements();
    }

    #[googletest::test]
    fn failed_test() {
        expect_that!(get_num_arrangements_helper("???", vec![2]), eq(2));
    }

    #[googletest::test]
    fn gets_num_arrangements_correctly() {
        expect_that!(get_num_arrangements_helper(".", vec![1]), eq(0));
        expect_that!(get_num_arrangements_helper("#", vec![0]), eq(0));
        expect_that!(get_num_arrangements_helper("#", vec![1]), eq(1));
        expect_that!(get_num_arrangements_helper("?", vec![1]), eq(1));
        expect_that!(get_num_arrangements_helper("?", vec![0]), eq(1));

        expect_that!(get_num_arrangements_helper(".?", vec![0]), eq(1));
        expect_that!(get_num_arrangements_helper("#?", vec![0]), eq(0));
        expect_that!(get_num_arrangements_helper("#?", vec![1]), eq(1));
        expect_that!(get_num_arrangements_helper("??", vec![1]), eq(2));
        expect_that!(get_num_arrangements_helper("??#", vec![1]), eq(1));
        expect_that!(get_num_arrangements_helper("#??#", vec![1]), eq(0));

        expect_that!(get_num_arrangements_helper("#??#", vec![1, 1]), eq(1));
        expect_that!(get_num_arrangements_helper("#??#", vec![1, 2]), eq(1));
        expect_that!(get_num_arrangements_helper("#??#", vec![2, 1]), eq(1));
        expect_that!(get_num_arrangements_helper("#??#", vec![2, 2]), eq(0));
        expect_that!(get_num_arrangements_helper("#???#", vec![2, 2]), eq(1));
        expect_that!(get_num_arrangements_helper(".???#", vec![2, 2]), eq(0));

        expect_that!(get_num_arrangements_helper("???.###", vec![1, 1, 3]), eq(1));
        expect_that!(
            get_num_arrangements_helper(".??..??...?##.", vec![1, 1, 3]),
            eq(4)
        );
        expect_that!(
            get_num_arrangements_helper("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]),
            eq(1)
        );
        expect_that!(
            get_num_arrangements_helper("????.#...#...", vec![4, 1, 1]),
            eq(1)
        );
        expect_that!(
            get_num_arrangements_helper("????.######..#####.", vec![1, 6, 5]),
            eq(4)
        );
        expect_that!(
            get_num_arrangements_helper("?###????????", vec![3, 2, 1]),
            eq(10)
        );
    }
}
