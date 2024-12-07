from typing import List, Tuple, Generator

def parse_input() -> List[Tuple[int, List[str]]]:
    equations = []
    with open("inputs/day7", "r") as file:
        for line in file.readlines():
            target, values = line.split(":")
            target = int(target.strip())
            values = [v for v in values.strip().split()]
            equations.append((target, values))
    return equations

def can_evaluate_to_target(target: int, values: List[str], enable_concat=False) -> bool:
    def evaluate_rest(acc: int, next_values: List[str]) -> bool:
        if acc == target:
            return True
        elif acc > target:
            return False
        elif not next_values:
            return False
        return (evaluate_rest(acc + int(next_values[0]), next_values[1:])
            or evaluate_rest(acc * int(next_values[0]), next_values[1:])
            or (enable_concat and evaluate_rest(int(str(acc) + next_values[0]), next_values[1:])))
    if not values:
        return False
    return evaluate_rest(int(values[0]), values[1:])

def part1(equations: List[Tuple[int, List[str]]]):
    sum = 0
    for target, values in equations:
        if can_evaluate_to_target(target, values):
            sum += target
    return sum

def part2(equations: List[Tuple[int, List[str]]]):
    sum = 0
    for target, values in equations:
        if can_evaluate_to_target(target, values, enable_concat=True):
            sum += target
    return sum

def main():
    equations = parse_input()
    print(f"Part 1: {part1(equations)}")
    print(f"Part 2: {part2(equations)}")

if __name__=="__main__":
    main()
