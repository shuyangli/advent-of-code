from typing import Set, Tuple, List
import heapq
from lib import Grid, Coordinate, Direction, Directions

def part1(grid: Grid[str]):
    # Dijkstra
    starting_coordinates = grid.find("S")
    visited: Set[Tuple[Coordinate, Direction]] = set()

    # (score, coordinate, direction)
    next_positions: List[Tuple[int, Coordinate, Direction]] = []
    heapq.heappush(next_positions, (0, starting_coordinates, Directions.EAST))

    while next_positions:
        (score, coordinate, direction) = heapq.heappop(next_positions)
        if grid.get(coordinate) == "E":
            # We found the destination, this is the cheapest way to get there.
            return score
        visited.add((coordinate, direction))

        next_coord = coordinate.step(direction)
        if grid.get(next_coord) != "#" and (next_coord, direction) not in visited:
            heapq.heappush(next_positions, (score + 1, next_coord, direction))
        if (coordinate, direction.rotate(90)) not in visited:
            heapq.heappush(next_positions, (score + 1000, coordinate, direction.rotate(90)))
        if (coordinate, direction.rotate(-90)) not in visited:
            heapq.heappush(next_positions, (score + 1000, coordinate, direction.rotate(-90)))

    return "Something went wrong!"

def part2(grid: Grid[str]):
    starting_coordinates = grid.find("S")
    visited: Set[Tuple[Coordinate, Direction]] = set()
    cheapest_score = None
    coordinates_in_path: Set[Coordinate] = set()

    # (score, coordinate, direction, path so far)
    next_positions: List[Tuple[int, Coordinate, Direction, List[Coordinate]]] = []
    heapq.heappush(next_positions, (0, starting_coordinates, Directions.EAST, [starting_coordinates]))

    while next_positions:
        (score, coordinate, direction, path) = heapq.heappop(next_positions)
        if grid.get(coordinate) == "E":
            # We found the destination, this is the cheapest way to get there.
            if cheapest_score is None:
                cheapest_score = score
            if score == cheapest_score:
                coordinates_in_path.update(path)
            continue
        visited.add((coordinate, direction))
        if cheapest_score and score > cheapest_score:
            continue

        next_coord = coordinate.step(direction)
        if grid.get(next_coord) != "#" and (next_coord, direction) not in visited:
            heapq.heappush(next_positions, (score + 1, next_coord, direction, path + [next_coord]))
        if (coordinate, direction.rotate(90)) not in visited:
            heapq.heappush(next_positions, (score + 1000, coordinate, direction.rotate(90), path))
        if (coordinate, direction.rotate(-90)) not in visited:
            heapq.heappush(next_positions, (score + 1000, coordinate, direction.rotate(-90), path))

    return len(coordinates_in_path)

def main():
    grid = Grid.parse_file_as_grid("inputs/day16")
    print(f"Part 1: {part1(grid)}")
    print(f"Part 2: {part2(grid)}")

if __name__=="__main__":
    main()
