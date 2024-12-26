from typing import List, Dict, Tuple, DefaultDict
from collections import defaultdict
from lib import Coordinate

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

transition_hypotheses: Dict[Tuple[str, str], str] = {}

def instruction_to_transitions(instruction: str) -> Dict[Tuple[str, str], int]:
    transitions = defaultdict(lambda: 0)
    current = "A"
    for c in instruction:
        transitions[(current, c)] += 1
        current = c
    return transitions

def get_instruction_length(transitions: DefaultDict[Tuple[str, str], int]) -> int:
    return sum(transitions.values())

# Instruction is represented as a dictionary of transition count
def get_expanded_transitions(
        instruction: Dict[Tuple[str, str], int],
        keys: Dict[str, Coordinate],
        gap: Coordinate,
        transition_hypotheses: Dict[Tuple[str, str], str]) -> Dict[Tuple[str, str], int]:
    out_transitions = defaultdict(lambda: 0)
    for (current_char, next_char), count in instruction.items():
        expanded = get_shortest_transition(current_char, next_char, keys, gap, transition_hypotheses)
        expanded = instruction_to_transitions(expanded)
        for (expanded_0, expanded_1), expanded_count in expanded.items():
            out_transitions[(expanded_0, expanded_1)] += count * expanded_count
    return out_transitions

def get_shortest_transition(
    current: str,
    next: str,
    keys: Dict[str, Coordinate],
    gap: Coordinate,
    transition_hypotheses: Dict[Tuple[str, str], str]) -> str:
    if (current, next) in transition_hypotheses:
        return transition_hypotheses[(current, next)]

    current_coord = keys[current]
    next_coord = keys[next]

    dx = next_coord.x - current_coord.x
    dy = next_coord.y - current_coord.y

    if dx == 0:
        transition_hypotheses[(current, next)] = ("^" if dy < 0 else "v") * abs(dy) + "A"
        return transition_hypotheses[(current, next)]
    elif dy == 0:
        transition_hypotheses[(current, next)] = ("<" if dx < 0 else ">") * abs(dx) + "A"
        return transition_hypotheses[(current, next)]

    all_options = []
    if not Coordinate(next_coord.x, current_coord.y) == gap:
        all_options.append(("<" if dx < 0 else ">") * abs(dx) + ("^" if dy < 0 else "v") * abs(dy) + "A")
    if not Coordinate(current_coord.x, next_coord.y) == gap:
        all_options.append(("^" if dy < 0 else "v") * abs(dy) + ("<" if dx < 0 else ">") * abs(dx) + "A")
    if len(all_options) == 1:
        transition_hypotheses[(current, next)] = all_options[0]
        return all_options[0]

    instructions_0 = instruction_to_transitions(all_options[0])
    instructions_1 = instruction_to_transitions(all_options[1])

    while get_instruction_length(instructions_0) == get_instruction_length(instructions_1):
        transition_hypotheses[(current, next)] = all_options[0]
        instructions_0 = get_expanded_transitions(instructions_0, direction_keys, direction_gap, transition_hypotheses)
        transition_hypotheses[(current, next)] = all_options[1]
        instructions_1 = get_expanded_transitions(instructions_1, direction_keys, direction_gap,
        transition_hypotheses)
        if get_instruction_length(instructions_0) != get_instruction_length(instructions_1):
            break
    shorter = all_options[0] if get_instruction_length(instructions_0) < get_instruction_length(instructions_1) else all_options[1]
    transition_hypotheses[(current, next)] = shorter
    return shorter

def part1(instructions: List[str]):
    total_complexity = 0
    for original_instruction in instructions:
        transitions = instruction_to_transitions(original_instruction)
        transitions = get_expanded_transitions(transitions, number_keys, number_gap, transition_hypotheses)
        for _ in range(2):
            transitions = get_expanded_transitions(transitions, direction_keys, direction_gap, transition_hypotheses)
        instruction_length = get_instruction_length(transitions)
        complexity = int(original_instruction[:-1]) * instruction_length
        total_complexity += complexity

    return total_complexity

def part2(instructions: List[str]):
    total_complexity = 0
    for original_instruction in instructions:
        transitions = instruction_to_transitions(original_instruction)
        transitions = get_expanded_transitions(transitions, number_keys, number_gap, transition_hypotheses)
        for _ in range(25):
            transitions = get_expanded_transitions(transitions, direction_keys, direction_gap, transition_hypotheses)
        instruction_length = get_instruction_length(transitions)
        complexity = int(original_instruction[:-1]) * instruction_length
        total_complexity += complexity

    return total_complexity

def main():
    with open("inputs/day21") as file:
        instructions = [line.strip() for line in file.readlines()]

    print(f"Part 1: {part1(instructions)}")
    print(f"Part 2: {part2(instructions)}")

if __name__=="__main__":
    main()
