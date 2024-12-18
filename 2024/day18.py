from typing import List, Dict
from lib import Coordinate, Grid, Directions

def bfs(grid: Grid[bool]) -> int | None:
    visited = set()
    next_positions = [(Coordinate(0, 0), 0)]
    while next_positions:
        coordinate, num_steps = next_positions.pop(0)
        if coordinate in visited:
            continue
        if not grid.is_in_bounds(coordinate):
            continue
        if not grid.get(coordinate):
            continue
        visited.add(coordinate)
        if coordinate == Coordinate(70, 70):
            return num_steps
        for d in Directions.CARDINALS:
            next_coord = coordinate.step(d)
            next_positions.append((next_coord, num_steps + 1))
    return None

def part1(obstacles: List[Coordinate]):
    grid = Grid(width=71, height=71, init_value=True)
    for o in obstacles[:1024]:
        grid.set(o, False)
    return bfs(grid)

def part2(obstacles: List[Coordinate]):
    start_idx = 1024
    end_idx = len(obstacles)
    while start_idx < end_idx:
        next_idx = (start_idx + end_idx) // 2
        grid = Grid(width=71, height=71, init_value=True)
        for o in obstacles[:next_idx]:
            grid.set(o, False)
        num_steps = bfs(grid)
        if num_steps is None:
            end_idx = next_idx - 1
        else:
            start_idx = next_idx + 1
    return obstacles[start_idx]

def main():
    obstacles = []
    with open("inputs/day18") as file:
        for line in file.readlines():
            x, y = line.strip().split(",")
            obstacles.append(Coordinate(int(x), int(y)))

    print(f"Part 1: {part1(obstacles)}")
    print(f"Part 2: {part2(obstacles)}")

if __name__=="__main__":
    main()
