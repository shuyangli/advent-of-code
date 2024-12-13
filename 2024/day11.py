from typing import List, Dict, Tuple, DefaultDict
from collections import defaultdict

def apply_rules(stones: Dict[int, int]) -> DefaultDict[int, int]:
    new_stones = defaultdict(lambda: 0)
    for stone, count in stones.items():
        if stone == 0:
            new_stones[1] += count
            continue
        stone_str = str(stone)
        if len(stone_str) % 2 == 0:
            new_stones[int(stone_str[:len(stone_str) // 2])] += count
            new_stones[int(stone_str[len(stone_str) // 2:])] += count
            continue
        new_stones[stone * 2024] += count
    return new_stones

def part1(stones: Dict[int, int]):
    for _ in range(25):
        stones = apply_rules(stones)
    return sum(stones.values())

def part2(stones: Dict[int, int]):
    for _ in range(75):
        stones = apply_rules(stones)
    return sum(stones.values())

def main():
    with open("inputs/day11") as file:
        stones = {int(x): 1 for x in file.read().split()}

    print(f"Part 1: {part1(stones)}")
    print(f"Part 2: {part2(stones)}")

if __name__=="__main__":
    main()
