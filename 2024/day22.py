from typing import Tuple, Dict, DefaultDict, List
from collections import defaultdict
import math

secret_number_map: Dict[int, int] = {}

def calculate_next_step(number: int):
    if number in secret_number_map:
        return secret_number_map[number]
    initial_number = number

    number = ((number * 64) ^ number) % 16777216
    number = (math.floor(number / 32) ^ number) % 16777216
    number = ((number * 2048) ^ number) % 16777216
    secret_number_map[initial_number] = number
    return number

def part1(initial_secrets: List[int]):
    sum = 0
    for n in initial_secrets:
        for _ in range(2000):
            n = calculate_next_step(n)
        sum += n
    return sum

def part2(initial_secrets: List[int]):
    sequence_to_values: DefaultDict[Tuple[int, int, int, int], int] = defaultdict(lambda: 0)
    for n in initial_secrets:
        seen_sequences = set()
        price_sequence = []
        last_price = None
        for _ in range(2000):
            n = calculate_next_step(n)

            current_price = n % 10
            if last_price is not None:
                price_diff = last_price - current_price
                if len(price_sequence) == 4:
                    price_sequence.pop(0)
                price_sequence.append(price_diff)
            last_price = current_price

            if len(price_sequence) == 4:
                price_sequence_tuple = tuple(price_sequence)
                if price_sequence_tuple in seen_sequences:
                    continue
                seen_sequences.add(price_sequence_tuple)
                sequence_to_values[price_sequence_tuple] += current_price

    largest = 0
    for k, v in sequence_to_values.items():
        if v > largest:
            largest = v
    return largest

def main():
    with open("inputs/day22") as file:
        initial_secrets = [int(l.strip()) for l in file.readlines()]

    print(f"Part 1: {part1(initial_secrets)}")
    print(f"Part 2: {part2(initial_secrets)}")

if __name__=="__main__":
    main()
