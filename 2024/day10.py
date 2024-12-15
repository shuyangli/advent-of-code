from typing import List, Set
from lib import Grid, Coordinate, Directions

def part1(grid: Grid[int]):
    all_trailheads: List[Coordinate] = []
    for value, coords in grid.elements():
        if value == 0:
            all_trailheads.append(coords)
    def dfs(position: Coordinate, expected_value: int, nines_reached: Set[Coordinate]):
        if not grid.is_in_bounds(position):
            return 0
        if grid.get(position) != expected_value:
            return 0
        if expected_value == 9 and position not in nines_reached:
            nines_reached.add(position)
            return 1
        value = sum([dfs(position.step(direction), expected_value + 1, nines_reached) for direction in Directions.CARDINALS])
        return value

    total_score = 0
    for trailhead in all_trailheads:
        total_score += dfs(trailhead, 0, set())
    return total_score

def part2(grid: Grid[int]):
    all_trailheads: List[Coordinate] = []
    for value, coords in grid.elements():
        if value == 0:
            all_trailheads.append(coords)

    # Number of 9's one can reach from the given position, using the rules.
    memoized_ratings: Grid[int | None] = Grid(grid=[[None for _ in range(len(grid[0]))] for _ in range(len(grid))])

    def dfs(position: Coordinate, expected_value: int):
        if not grid.is_in_bounds(position):
            return 0
        if grid.get(position) != expected_value:
            return 0
        if memoized_ratings.get(position) is not None:
            return memoized_ratings.get(position)
        if expected_value == 9:
            memoized_ratings.set(position, 1)
            return 1
        value = sum([dfs(position.step(direction), expected_value + 1) for direction in Directions.CARDINALS])
        memoized_ratings.set(position, value)
        return value

    for value, position in grid.elements():
        dfs(position, value)

    total_score = 0
    for trailhead in all_trailheads:
        score = memoized_ratings.get(trailhead)
        total_score += score

    return total_score

def main():
    map = Grid.parse_file_as_grid("inputs/day10")
    map = Grid(grid=[[int(v) for v in row] for row in map.grid])

    print(f"Part 1: {part1(map)}")
    print(f"Part 2: {part2(map)}")

if __name__=="__main__":
    main()
