from typing import List, Dict
from lib import Coordinate, Direction

number_keys = {
    "0": Coordinate(1, 3),
    "A": Coordinate(2, 3),
    "1": Coordinate(0, 2),
    "2": Coordinate(1, 2),
    "3": Coordinate(2, 2),
    "4": Coordinate(0, 1),
    "5": Coordinate(1, 1),
    "6": Coordinate(2, 1),
    "7": Coordinate(0, 0),
    "8": Coordinate(1, 0),
    "9": Coordinate(2, 0),
}
number_gap = Coordinate(0, 3)

direction_keys = {
    "^": Coordinate(1, 0),
    "A": Coordinate(2, 0),
    "<": Coordinate(0, 1),
    "v": Coordinate(1, 1),
    ">": Coordinate(2, 1),
}
direction_gap = Coordinate(0, 0)

def get_instruction_prime(instructions: List[str], keys: Dict[str, Coordinate], gap: Coordinate) -> List[str]:
    all_possible_instructions = []
    def process_next(idx: int, instruction: str, position: Coordinate, acc: List[str]):
        if idx == len(instruction):
            all_possible_instructions.append("".join(acc))
            return
        next_char = instruction[idx]
        next_position = keys[next_char]
        dx = next_position.x - position.x
        dy = next_position.y - position.y

        if dx == 0:
            next_acc = acc + ["^" if dy < 0 else "v"] * abs(dy) + ["A"]
            return process_next(idx + 1, instruction, next_position, next_acc)
        elif dy == 0:
            next_acc = acc + ["<" if dx < 0 else ">"] * abs(dx) + ["A"]
            return process_next(idx + 1, instruction, next_position, next_acc)

        # We have a choice between left/right vs. up/down
        # left/right first
        current = position
        allowed = True
        next_acc = [] + acc
        for _ in range(abs(dx)):
            next_acc.append("<" if dx < 0 else ">")
            current = current.step(Direction(-1, 0) if dx < 0 else Direction(1, 0))
            if current == gap:
                allowed = False
        for _ in range(abs(dy)):
            next_acc.append("^" if dy < 0 else "v")
            current = current.step(Direction(0, -1) if dy < 0 else Direction(0, 1))
            if current == gap:
                allowed = False
        if allowed:
            next_acc.append("A")
            process_next(idx + 1, instruction, next_position, next_acc)

        # up/down first
        current = position
        allowed = True
        next_acc = [] + acc
        for _ in range(abs(dy)):
            next_acc.append("^" if dy < 0 else "v")
            current = current.step(Direction(0, -1) if dy < 0 else Direction(0, 1))
            if current == gap:
                allowed = False
        for _ in range(abs(dx)):
            next_acc.append("<" if dx < 0 else ">")
            current = current.step(Direction(-1, 0) if dx < 0 else Direction(1, 0))
            if current == gap:
                allowed = False
        if allowed:
            next_acc.append("A")
            process_next(idx + 1, instruction, next_position, next_acc)

    for ins in instructions:
        position = keys["A"]
        process_next(0, ins, position, [])
    return all_possible_instructions

def part1(instructions: List[str]):
    total_complexity = 0

    for original_instruction in instructions:
        instruction = get_instruction_prime([original_instruction], number_keys, number_gap)

        for _ in range(2):
            instruction_prime = get_instruction_prime(instruction, direction_keys, direction_gap)
            instruction = instruction_prime
        min_len = min([len(i) for i in instruction])
        complexity = int(original_instruction[:-1]) * min_len
        print(f"Complexity for {original_instruction}: {complexity}")
        total_complexity += complexity

    return total_complexity

def part2(instructions: List[str]):
    return "Unimplemented"

def main():
    with open("inputs/day21") as file:
        instructions = [line.strip() for line in file.readlines()]

    print(f"Part 1: {part1(instructions)}")
    print(f"Part 2: {part2(instructions)}")

if __name__=="__main__":
    main()
