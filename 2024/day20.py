from typing import Tuple, Dict, DefaultDict, List
from collections import defaultdict
from lib import Coordinate, Grid, Directions

def bfs(grid: Grid[bool], starting_coord: Coordinate) -> List[Coordinate]:
    visited = set()
    # Next_positions: (coord, path)
    next_positions = [(starting_coord, [starting_coord])]
    while next_positions:
        coordinate, path = next_positions.pop(0)
        if not grid.is_in_bounds(coordinate):
            continue
        if grid.get(coordinate) == "E":
            return path
        if coordinate in visited:
            continue
        if grid.get(coordinate) == "#":
            continue
        visited.add(coordinate)
        for d in Directions.CARDINALS:
            next_coord = coordinate.step(d)
            next_positions.append((next_coord, path + [next_coord]))
    raise ValueError("Cannot find a path!")

# Returns {steps => number of cheats}
def bfs_with_cheats(grid: Grid[bool], starting_coord: Coordinate, allowed_cheats: int) -> DefaultDict[int, int]:
    # First do a BFS without any cheats
    path_without_cheats = bfs(grid, starting_coord)
    length_without_cheats = len(path_without_cheats) - 1

    visited: Dict[Coordinate, int] = {}
    for i, coord in enumerate(path_without_cheats):
        visited[coord] = length_without_cheats - i
    print(visited)

    # Then do a BFS with cheats
    # Number of steps => number of options
    num_steps_with_cheating: DefaultDict[int, int] = defaultdict(lambda: 0)

    # (current coordinate, number of cheats, currently cheating, number of steps taken)
    visited_without_cheats = set()
    options: List[Tuple[Coordinate, int, bool, int]] = []
    options.append((starting_coord, allowed_cheats, False, 0))
    while options:
        (coordinate, remaining_cheats, cheating, num_steps) = options.pop()
        if not grid.is_in_bounds(coordinate):
            continue
        if grid.get(coordinate) == "#" and not cheating:
            continue

        # If we've finished cheating, stop iterating.
        if remaining_cheats <= 0:
            if coordinate in visited:
                num_steps_with_cheating[num_steps + visited[coordinate]] += 1
            continue
        # Otherwise maybe terminate if we have visited before
        if not cheating and coordinate in visited_without_cheats:
            continue
        if not cheating and remaining_cheats == allowed_cheats:
            visited_without_cheats.add(coordinate)

        for d in Directions.CARDINALS:
            next_coord = coordinate.step(d)
            if cheating:
                options.append((next_coord, remaining_cheats - 1, remaining_cheats - 1 > 0, num_steps + 1))
            else:
                options.append((next_coord, remaining_cheats, False, num_steps + 1))
                options.append((next_coord, remaining_cheats - 1, True, num_steps + 1))

    return num_steps_with_cheating


def part1(grid: Grid[str], starting_coord: Coordinate):
    num_steps_without_cheating = len(bfs(grid, starting_coord)) - 1
    num_steps_with_cheating = bfs_with_cheats(grid, starting_coord, allowed_cheats=2)

    num_options_saving_one_hundred_ps = 0
    for steps, num_options in num_steps_with_cheating.items():
        if num_steps_without_cheating - steps >= 100:
            num_options_saving_one_hundred_ps += num_options

    return num_options_saving_one_hundred_ps

def part2(grid: Grid[str], starting_coord: Coordinate):
    return "Unimplemented"

def main():
    grid: Grid[str] = Grid.parse_file_as_grid("inputs/day20")
    starting_coord = grid.find("S")

    print(f"Part 1: {part1(grid, starting_coord)}")
    print(f"Part 2: {part2(grid, starting_coord)}")

if __name__=="__main__":
    main()
