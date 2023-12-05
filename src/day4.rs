use crate::day::Day;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashSet, fmt::Display};

pub struct Day4 {}

#[derive(PartialEq, Debug)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    numbers_on_card: HashSet<i32>,
}

impl Card {
    fn get_num_matching_numbers(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.numbers_on_card)
            .count() as u32
    }

    fn get_score(&self) -> i32 {
        match self.get_num_matching_numbers() {
            0 => 0,
            n => 2_i32.pow(n - 1),
        }
    }
}

impl Day for Day4 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let cards = parse_card_or_panic(input);
        return Ok(Box::new(cards.iter().map(|c| c.get_score()).sum::<i32>()));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let cards = parse_card_or_panic(input);
        let mut cards_count = vec![1; cards.len()];

        for (i, card) in cards.iter().enumerate() {
            let num_copies = cards_count[i];
            let num_cards_won = card.get_num_matching_numbers() as usize;

            for j in (i + 1)..=(i + num_cards_won) {
                if j < cards_count.len() {
                    cards_count[j] += num_copies;
                }
            }
        }

        return Ok(Box::new(cards_count.iter().sum::<i32>()));
    }
}

static CARD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Card +(?<card_id>\d+): (?<winning_numbers>[\d ]+) \| (?<card_numbers>[\d ]+)")
        .unwrap()
});

fn parse_card_or_panic(input: &str) -> Vec<Card> {
    let mut cards_vec = vec![];

    for (_, [card_id, winning_numbers, card_numbers]) in
        CARD_REGEX.captures_iter(input).map(|f| f.extract())
    {
        let mut card = Card {
            id: card_id.parse().unwrap(),
            winning_numbers: HashSet::new(),
            numbers_on_card: HashSet::new(),
        };
        for winning_number in winning_numbers.trim().split_ascii_whitespace() {
            card.winning_numbers.insert(winning_number.parse().unwrap());
        }
        for card_number in card_numbers.trim().split_ascii_whitespace() {
            card.numbers_on_card.insert(card_number.parse().unwrap());
        }
        cards_vec.push(card);
    }

    return cards_vec;
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[googletest::test]
    fn parses_single_card_correctly() {
        let cards = parse_card_or_panic(
            r#"\
Card   1: 69 61 27 58 89 52 81 94 40 51 | 43 40 52 90 37 97 89 80 69 42 51 70 94 58 10 73 21 29 61 63 57 79 81 27 35
"#,
        );
        expect_that!(
            cards,
            elements_are![eq(Card {
                id: 1,
                winning_numbers: HashSet::from([69, 61, 27, 58, 89, 52, 81, 94, 40, 51]),
                numbers_on_card: HashSet::from([
                    43, 40, 52, 90, 37, 97, 89, 80, 69, 42, 51, 70, 94, 58, 10, 73, 21, 29, 61, 63,
                    57, 79, 81, 27, 35
                ])
            })]
        );
    }

    #[googletest::test]
    fn parses_multiple_cards_correctly() {
        let cards = parse_card_or_panic(
            r#"\
Card 1: 41 48 83 86 17  | 83 86  6 31 17  9 48 53
Card 2: 0 1 2 3 | 1 2     3 4 5   6   7 8 9 10
"#,
        );
        expect_that!(
            cards,
            elements_are![
                eq(Card {
                    id: 1,
                    winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
                    numbers_on_card: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
                }),
                eq(Card {
                    id: 2,
                    winning_numbers: HashSet::from([0, 1, 2, 3]),
                    numbers_on_card: HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
                })
            ]
        );
    }

    #[googletest::test]
    fn scores_cards_correctly() {
        expect_that!(
            Card {
                id: 1,
                winning_numbers: HashSet::from([1, 2]),
                numbers_on_card: HashSet::from([3, 4]),
            }
            .get_score(),
            eq(0)
        );
        expect_that!(
            Card {
                id: 1,
                winning_numbers: HashSet::from([1, 2]),
                numbers_on_card: HashSet::from([2, 3, 4]),
            }
            .get_score(),
            eq(1)
        );
        expect_that!(
            Card {
                id: 1,
                winning_numbers: HashSet::from([1, 2]),
                numbers_on_card: HashSet::from([1, 2, 3, 4]),
            }
            .get_score(),
            eq(2)
        );
        expect_that!(
            Card {
                id: 1,
                winning_numbers: HashSet::from([1, 2, 3]),
                numbers_on_card: HashSet::from([1, 2, 3, 4]),
            }
            .get_score(),
            eq(4)
        );
        expect_that!(
            Card {
                id: 1,
                winning_numbers: HashSet::from([1, 2, 3, 4]),
                numbers_on_card: HashSet::from([1, 2, 3, 4]),
            }
            .get_score(),
            eq(8)
        );
    }
}
