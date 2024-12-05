import re

def part1():
    with open("inputs/day3", "r") as file:
        instruction = file.read()

    regex = re.compile(r'mul\((\d+),(\d+)\)')
    matches = regex.finditer(instruction)

    total = 0
    for match in matches:
        total += int(match.group(1)) * int(match.group(2))

    return total

def part2():
    with open("inputs/day3", "r") as file:
        instruction = file.read()

    regex = re.compile(r'mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)')
    matches = regex.finditer(instruction)

    enabled = True
    total = 0
    for match in matches:
        if match.group(0) == "do()":
            enabled = True
        elif match.group(0) == "don't()":
            enabled = False
        elif enabled:
            total += int(match.group(1)) * int(match.group(2))

    return total

def main():
    print(f"Part 1: {part1()}")
    print(f"Part 2: {part2()}")

if __name__=="__main__":
    main()
