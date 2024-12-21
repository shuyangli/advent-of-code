from typing import Tuple, Dict, DefaultDict, List
from collections import defaultdict
from lib import Coordinate, Grid, Direction, Directions

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
def shortcut_with_cheats(allowed_cheats: int, path_without_cheats: List[Coordinate]) -> DefaultDict[int, int]:
    length_without_cheats = len(path_without_cheats) - 1
    coord_to_remaining_distance: Dict[Coordinate, int] = {}
    for steps_so_far, coord in enumerate(path_without_cheats):
        coord_to_remaining_distance[coord] = length_without_cheats - steps_so_far

    num_steps_with_cheating: DefaultDict[int, int] = defaultdict(lambda: 0)
    for steps_so_far, coord in enumerate(path_without_cheats):
        for di in range(-allowed_cheats, allowed_cheats + 1):
            remaining_cheats = allowed_cheats - abs(di)
            for dj in range(-remaining_cheats, remaining_cheats + 1):
                if di == 0 and dj == 0:
                    continue
                d = Direction(di, dj)
                coord_after_cheat = coord.step(d)
                if coord_after_cheat not in coord_to_remaining_distance:
                    continue
                num_total_steps = steps_so_far + abs(di) + abs(dj) + coord_to_remaining_distance[coord_after_cheat]
                num_steps_with_cheating[num_total_steps] += 1

    return num_steps_with_cheating

def part1(path: List[Coordinate]):
    num_steps_without_cheating = len(path) - 1
    num_steps_with_cheating = shortcut_with_cheats(allowed_cheats=2, path_without_cheats=path)

    num_options_saving_one_hundred_ps = 0
    for steps, num_options in num_steps_with_cheating.items():
        if num_steps_without_cheating - steps >= 100:
            num_options_saving_one_hundred_ps += num_options

    return num_options_saving_one_hundred_ps

def part2(path: List[Coordinate]):
    num_steps_without_cheating = len(path) - 1
    num_steps_with_cheating = shortcut_with_cheats(allowed_cheats=20, path_without_cheats=path)

    num_options_saving_one_hundred_ps = 0
    for steps, num_options in num_steps_with_cheating.items():
        if num_steps_without_cheating - steps >= 100:
            num_options_saving_one_hundred_ps += num_options

    return num_options_saving_one_hundred_ps

def main():
    grid: Grid[str] = Grid.parse_file_as_grid("inputs/day20")
    starting_coord = grid.find("S")
    path = bfs(grid, starting_coord)

    print(f"Part 1: {part1(path)}")
    print(f"Part 2: {part2(path)}")

if __name__=="__main__":
    main()
