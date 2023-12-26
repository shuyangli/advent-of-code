use itertools::Itertools;

use crate::day::Day;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

pub struct Day22 {}

type BrickId = usize;

#[derive(Debug, Clone)]
struct Brick {
    brick_id: BrickId,
    // (x, y, z)
    start: (u32, u32, u32),
    end: (u32, u32, u32),
}

// Vec will be sorted by Z axis of the start piece.
fn parse_bricks(input: &str) -> Vec<Brick> {
    let mut bricks = vec![];
    for (brick_id, line) in input.lines().enumerate() {
        let (start, end) = line.split_once('~').unwrap();
        bricks.push(Brick {
            brick_id,
            start: start
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect_tuple()
                .unwrap(),
            end: end
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect_tuple()
                .unwrap(),
        });
    }
    bricks.sort_by(|a, b| a.start.2.cmp(&b.start.2));
    return bricks;
}

// Insert brick updated to current_layer into fallen_bricks, and update the layers[x][y] range to be the new layer with a brick + brick id.
fn place_brick_at_layer(
    layers: &mut HashMap<u32, HashMap<u32, (u32, BrickId)>>,
    brick: &Brick,
    current_layer: u32,
) {
    let new_end = current_layer + brick.end.2 - brick.start.2;

    for i in brick.start.0..=brick.end.0 {
        for j in brick.start.1..=brick.end.1 {
            layers
                .entry(i)
                .or_insert(HashMap::new())
                .insert(j, (new_end, brick.brick_id));
        }
    }
}

fn get_layer_for_brick(
    brick: &Brick,
    layers: &HashMap<u32, HashMap<u32, (u32, BrickId)>>,
) -> (u32, HashSet<BrickId>) {
    if brick.start.2 == 1 {
        return (1, HashSet::new());
    }

    let mut highest_layer = 0_u32;
    let mut supporting_brick_ids = HashSet::new();
    for i in brick.start.0..=brick.end.0 {
        for j in brick.start.1..=brick.end.1 {
            if let Some((layer, brick_id)) = layers.get(&i).and_then(|js| js.get(&j)) {
                if *layer > highest_layer {
                    highest_layer = *layer;
                    supporting_brick_ids.clear();
                    supporting_brick_ids.insert(*brick_id);
                } else if *layer == highest_layer {
                    supporting_brick_ids.insert(*brick_id);
                }
            }
        }
    }

    return (highest_layer + 1, supporting_brick_ids);
}

fn convert_supporting_bricks(
    bricks_supported_by_brick: &HashMap<BrickId, HashSet<BrickId>>,
) -> HashMap<BrickId, HashSet<BrickId>> {
    let mut bricks_supporting_other_bricks = HashMap::new();

    for (&k, vs) in bricks_supported_by_brick.iter() {
        for &v in vs {
            bricks_supporting_other_bricks
                .entry(v)
                .or_insert(HashSet::new())
                .insert(k);
        }
    }

    return bricks_supporting_other_bricks;
}

fn will_brick_fall_with_support_removed(
    brick_id: BrickId,
    bricks_removed: &HashSet<BrickId>,
    bricks_supporting_bricks: &HashMap<BrickId, HashSet<BrickId>>,
) -> bool {
    let supports = bricks_supporting_bricks.get(&brick_id).unwrap();
    return supports.intersection(bricks_removed).count() == supports.len();
}

impl Day for Day22 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let bricks = parse_bricks(input);

        // Simulate bricks falling and place them in a set of layers.
        let mut layers = HashMap::new(); // x => y => z
        let mut unique_bricks_supporting_others = HashSet::<BrickId>::new();
        for brick in bricks.iter() {
            let (layer, supporting_bricks) = get_layer_for_brick(brick, &layers);

            place_brick_at_layer(&mut layers, brick, layer);

            // If only one brick is supporting the one being placed, then we cannot remove the supporting brick.
            if supporting_bricks.len() == 1 {
                unique_bricks_supporting_others.extend(supporting_bricks.iter());
            }
        }

        let answer = bricks.len() - unique_bricks_supporting_others.len();

        return Ok(Box::new(answer));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let bricks = parse_bricks(input);

        // Simulate bricks falling and place them in a set of layers.
        let mut layers = HashMap::new(); // x => y => z

        let mut bricks_supported_by_brick = HashMap::<BrickId, HashSet<BrickId>>::new();

        for brick in bricks.iter() {
            let (layer, supporting_bricks) = get_layer_for_brick(brick, &layers);

            place_brick_at_layer(&mut layers, brick, layer);

            // If only one brick is supporting the one being placed, then when it's disintegrated, things will fall
            for i in supporting_bricks {
                bricks_supported_by_brick
                    .entry(i)
                    .or_insert(HashSet::new())
                    .insert(brick.brick_id);
            }
        }
        let bricks_supporting_bricks = convert_supporting_bricks(&bricks_supported_by_brick);

        let mut num_falling_bricks = 0;

        for brick in 0..bricks.len() {
            let mut fallen_bricks = HashSet::new();
            fallen_bricks.insert(brick);

            let mut bricks_to_check = VecDeque::new();
            bricks_to_check.push_back(brick);
            while let Some(brick_to_check) = bricks_to_check.pop_front() {
                if let Some(supported_bricks) = bricks_supported_by_brick.get(&brick_to_check) {
                    for supported in supported_bricks {
                        if !fallen_bricks.contains(supported)
                            && will_brick_fall_with_support_removed(
                                *supported,
                                &fallen_bricks,
                                &bricks_supporting_bricks,
                            )
                        {
                            fallen_bricks.insert(*supported);
                            bricks_to_check.push_back(*supported);
                        }
                    }
                }
            }

            num_falling_bricks += fallen_bricks.len() - 1;
        }

        return Ok(Box::new(num_falling_bricks));
    }
}
