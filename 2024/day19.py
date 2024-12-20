from typing import List, Dict

def part1(towels: List[str], patterns: List[str]):
    memoized: Dict[str, bool] = {}

    def is_possible(pattern: str) -> bool:
        if not pattern:
            return True
        for t in towels:
            if not pattern.endswith(t):
                continue
            remaining_pattern = pattern[:-len(t)]
            if is_possible(remaining_pattern):
                memoized[pattern] = True
                return True
        memoized[pattern] = False
        return False

    possible_patterns = 0
    for p in patterns:
        if is_possible(p):
            possible_patterns += 1
    return possible_patterns

def part2(towels: List[str], patterns: List[str]):
    memoized: Dict[str, int] = {}

    def get_num_configurations(pattern: str) -> int:
        if not pattern:
            return 1
        if pattern in memoized:
            return memoized[pattern]

        num_configurations = 0
        for t in towels:
            if not pattern.endswith(t):
                continue
            remaining_pattern = pattern[:-len(t)]
            num_configurations += get_num_configurations(remaining_pattern)
        memoized[pattern] = num_configurations
        return num_configurations

    sum_num_configurations = 0
    for p in patterns:
        sum_num_configurations += get_num_configurations(p)
    return sum_num_configurations

def main():
    with open("inputs/day19") as file:
        lines = file.readlines()
        towels = lines[0].strip().split(", ")
        patterns = [l.strip() for l in lines[2:]]

    print(f"Part 1: {part1(towels, patterns)}")
    print(f"Part 2: {part2(towels, patterns)}")

if __name__=="__main__":
    main()
