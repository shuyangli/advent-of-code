from lib import Coordinate, Direction, Grid
from typing import List
import re

class Robot():
    def __init__(self, position: Coordinate, velocity: Direction):
        self.position = position
        self.velocity = velocity

    def __str__(self):
        return f"(position={self.position}, velocity={self.velocity})"

    def __repr__(self):
        return self.__str__()

def part1(robots: List[Robot], width: int, height: int):
    quadrant_count = [[0, 0],
                      [0, 0]]

    for robot in robots:
        new_position = robot.position.step(robot.velocity, 100)
        new_position.x %= width
        new_position.y %= height

        if new_position.x == width // 2 or new_position.y == height // 2:
            continue
        quadrant_i = 0 if new_position.x < width // 2 else 1
        quadrant_j = 0 if new_position.y < height // 2 else 1
        quadrant_count[quadrant_i][quadrant_j] += 1

    return quadrant_count[0][0] * quadrant_count[0][1] * quadrant_count[1][0] * quadrant_count[1][1]

def part2(robots: List[Robot], width, height):
    iteration = 0
    while True:
        iteration += 1
        grid = Grid(width=width, height=height, init_value=".")
        for robot in robots:
            new_position = robot.position.step(robot.velocity, iteration)
            new_position.x %= width
            new_position.y %= height

            if grid[new_position.y][new_position.x] == ".":
                grid[new_position.y][new_position.x] = 1
            else:
                grid[new_position.y][new_position.x] += 1

        grid_str = grid.__str__()
        if "11111111111" in grid_str:
            break

    print(f"Iteration {iteration}:\n{grid}\n\n")
    return iteration

def main():
    with open("inputs/day14") as file:
        regex = re.compile(r'p=(-?[\d]+),(-?[\d]+) v=(-?[\d]+),(-?[\d]+)')

        robots: List[Robot] = []
        for line in file.readlines():
            matches = regex.match(line)
            machine = Robot(Coordinate(int(matches.group(1)), int(matches.group(2))),
                              Direction(int(matches.group(3)), int(matches.group(4))))
            robots.append(machine)

    area_width = 101
    area_height = 103

    print(f"Part 1: {part1(robots, area_width, area_height)}")
    print(f"Part 2: {part2(robots, area_width, area_height)}")

if __name__=="__main__":
    main()
