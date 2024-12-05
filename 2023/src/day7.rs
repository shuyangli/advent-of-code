use crate::day::Day;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

pub struct Day7 {}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    hand_type: HandType,
    cards: String,
}

fn char_to_value_part1(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        v => v.to_digit(10).unwrap(),
    }
}

fn char_to_value_part2(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        v => v.to_digit(10).unwrap(),
    }
}

impl Hand {
    fn parse_from_hand_part1(hand: &str) -> Self {
        // cards is a map from card value to count
        let mut cards = HashMap::<u32, u32>::new();
        for c in hand.chars() {
            let value = char_to_value_part1(c);
            *cards.entry(value).or_insert(0) += 1;
        }

        // cards_vec is sorted by count and then value
        let mut cards_vec: Vec<_> = cards.iter().collect();
        cards_vec.sort_by(|a: &(&u32, &u32), b| {
            if b.1.cmp(a.1) != Ordering::Equal {
                return b.1.cmp(a.1);
            }
            return b.0.cmp(a.0);
        });

        return match cards_vec.len() {
            1 => Hand {
                hand_type: HandType::FiveOfAKind,
                cards: hand.to_owned(),
            },
            2 => {
                if *cards_vec[0].1 == 4 {
                    Hand {
                        hand_type: HandType::FourOfAKind,
                        cards: hand.to_owned(),
                    }
                } else {
                    Hand {
                        hand_type: HandType::FullHouse,
                        cards: hand.to_owned(),
                    }
                }
            }
            3 => {
                if *cards_vec[0].1 == 3 {
                    Hand {
                        hand_type: HandType::ThreeOfAKind,
                        cards: hand.to_owned(),
                    }
                } else {
                    Hand {
                        hand_type: HandType::TwoPair,
                        cards: hand.to_owned(),
                    }
                }
            }
            4 => Hand {
                hand_type: HandType::OnePair,
                cards: hand.to_owned(),
            },
            5 => Hand {
                hand_type: HandType::HighCard,
                cards: hand.to_owned(),
            },
            _ => panic!("Unexpected length for cards_vec"),
        };
    }

    fn parse_from_hand_part2(hand: &str) -> Self {
        // cards is a map from card value to count
        let mut cards = HashMap::<u32, u32>::new();
        let mut num_jokers = 0;
        for c in hand.chars() {
            if c == 'J' {
                num_jokers += 1;
                continue;
            }
            let value = char_to_value_part2(c);
            *cards.entry(value).or_insert(0) += 1;
        }

        // cards_vec is sorted by count and then value
        let mut cards_vec: Vec<_> = cards.into_iter().collect();
        cards_vec.sort_by(|a, b| {
            if b.1.cmp(&a.1) != Ordering::Equal {
                return b.1.cmp(&a.1);
            }
            return b.0.cmp(&a.0);
        });

        // assign jokers to the best card in the hand
        if cards_vec.len() == 0 {
            // There are only jokers
            return Hand {
                hand_type: HandType::FiveOfAKind,
                cards: hand.to_owned(),
            };
        } else {
            cards_vec[0] = (cards_vec[0].0, cards_vec[0].1 + num_jokers);
        }

        return match cards_vec.len() {
            1 => Hand {
                hand_type: HandType::FiveOfAKind,
                cards: hand.to_owned(),
            },
            2 => {
                if cards_vec[0].1 == 4 {
                    Hand {
                        hand_type: HandType::FourOfAKind,
                        cards: hand.to_owned(),
                    }
                } else {
                    Hand {
                        hand_type: HandType::FullHouse,
                        cards: hand.to_owned(),
                    }
                }
            }
            3 => {
                if cards_vec[0].1 == 3 {
                    Hand {
                        hand_type: HandType::ThreeOfAKind,
                        cards: hand.to_owned(),
                    }
                } else {
                    Hand {
                        hand_type: HandType::TwoPair,
                        cards: hand.to_owned(),
                    }
                }
            }
            4 => Hand {
                hand_type: HandType::OnePair,
                cards: hand.to_owned(),
            },
            5 => Hand {
                hand_type: HandType::HighCard,
                cards: hand.to_owned(),
            },
            _ => panic!("Unexpected length for cards_vec"),
        };
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Player {
    cards: Hand,
    bid: i32,
}

impl Day for Day7 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut players = vec![];
        for line in input.lines() {
            let mut items = line.split_ascii_whitespace();
            players.push(Player {
                cards: Hand::parse_from_hand_part1(items.next().unwrap()),
                bid: items.next().unwrap().parse().unwrap(),
            })
        }

        players.sort_by(|a, b| {
            let type_ordering = a.cards.hand_type.cmp(&b.cards.hand_type);
            if type_ordering != Ordering::Equal {
                return type_ordering;
            }

            for (s, o) in a.cards.cards.chars().zip(b.cards.cards.chars()) {
                let card_ordering = char_to_value_part1(s).cmp(&char_to_value_part1(o));
                if card_ordering != Ordering::Equal {
                    return card_ordering;
                }
            }

            return Ordering::Equal;
        });

        let total_score = players.iter().enumerate().fold(0, |acc, (i, player)| {
            let rank = i as i32 + 1;
            acc + rank * player.bid
        });
        return Ok(Box::new(total_score));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut players = vec![];
        for line in input.lines() {
            let mut items = line.split_ascii_whitespace();
            players.push(Player {
                cards: Hand::parse_from_hand_part2(items.next().unwrap()),
                bid: items.next().unwrap().parse().unwrap(),
            })
        }

        players.sort_by(|a, b| {
            let type_ordering = a.cards.hand_type.cmp(&b.cards.hand_type);
            if type_ordering != Ordering::Equal {
                return type_ordering;
            }

            for (s, o) in a.cards.cards.chars().zip(b.cards.cards.chars()) {
                let card_ordering = char_to_value_part2(s).cmp(&char_to_value_part2(o));
                if card_ordering != Ordering::Equal {
                    return card_ordering;
                }
            }

            return Ordering::Equal;
        });

        let total_score = players.iter().enumerate().fold(0, |acc, (i, player)| {
            let rank = i as i32 + 1;
            acc + rank * player.bid
        });
        return Ok(Box::new(total_score));
    }
}
