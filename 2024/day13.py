from lib import Coordinate, Direction
from typing import List
import re
import math

class Machine():
    def __init__(self, a: Direction, b: Direction, prize: Coordinate):
        self.button_a = a
        self.button_b = b
        self.prize = prize

    def __str__(self):
        return f"A: {self.button_a}; B: {self.button_b}, Prize: {self.prize}"

def part1(machines: List[Machine]):
    total_tokens = 0
    for m in machines:
        # Solve two equations
        a = (m.prize.x / m.button_b.dx - m.prize.y / m.button_b.dy) / (m.button_a.dx / m.button_b.dx - m.button_a.dy / m.button_b.dy)
        b = (m.prize.x / m.button_a.dx - m.prize.y / m.button_a.dy) / (m.button_b.dx / m.button_a.dx - m.button_b.dy / m.button_a.dy)
        if a < 0 or b < 0 or a > 100 or b > 100 or not math.isclose(a, round(a), rel_tol=0, abs_tol=1e-5) or not math.isclose(b, round(b), rel_tol=0,  abs_tol=1e-5):
            continue
        total_tokens += round(a) * 3 + round(b) * 1
    return total_tokens

def part2(machines: List[Machine]):
    total_tokens = 0
    for m in machines:
        prize_x = 10000000000000 + m.prize.x
        prize_y = 10000000000000 + m.prize.y
        a = (prize_x / m.button_b.dx - prize_y / m.button_b.dy) / (m.button_a.dx / m.button_b.dx - m.button_a.dy / m.button_b.dy)
        b = (prize_x / m.button_a.dx - prize_y / m.button_a.dy) / (m.button_b.dx / m.button_a.dx - m.button_b.dy / m.button_a.dy)
        if a < 0 or b < 0 or not math.isclose(a, round(a), rel_tol=0, abs_tol=1e-4) or not math.isclose(b, round(b), rel_tol=0, abs_tol=1e-4):
            continue
        total_tokens += round(a) * 3 + round(b) * 1
    return total_tokens

def main():
    with open("inputs/day13") as file:
        regex_a = re.compile(r'Button A: X\+([\d]+), Y\+([\d]+)')
        regex_b = re.compile(r'Button B: X\+([\d]+), Y\+([\d]+)')
        regex_prize = re.compile(r'Prize: X=([\d]+), Y=([\d]+)')

        machines: List[Machine] = []
        lines = file.readlines()
        for i in range(0, len(lines) + 1, 4):
            button_a_x, button_a_y = regex_a.match(lines[i]).groups((1, 2))
            button_b_x, button_b_y = regex_b.match(lines[i + 1]).groups((1, 2))
            prize_x, prize_y = regex_prize.match(lines[i + 2]).groups((1, 2))
            machine = Machine(Direction(int(button_a_x), int(button_a_y)),
                              Direction(int(button_b_x), int(button_b_y)),
                              Coordinate(int(prize_x), int(prize_y)))
            machines.append(machine)

    print(f"Part 1: {part1(machines)}")
    print(f"Part 2: {part2(machines)}")

if __name__=="__main__":
    main()
