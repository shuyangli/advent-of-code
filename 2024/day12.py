from typing import Set, Tuple
from lib import Grid, Coordinate, Direction, Directions

def part1(grid: Grid[str]):
    visited: Set[Coordinate] = set()
    # For each position in the grid, we floodfill until the border, and calculate area and perimeter during the floodfill.
    total_price = 0
    for value, coords in grid.elements():
        if coords in visited:
            continue

        # Floodfill from here
        visited.add(coords)
        area = 1
        perimeter = 0
        next_positions = [coords.step(d) for d in Directions.CARDINALS]
        while next_positions:
            next_coords = next_positions.pop()
            if not grid.is_in_bounds(next_coords):
                perimeter += 1
            elif grid.get(next_coords) != value:
                perimeter += 1
            elif next_coords in visited:
                continue
            else:
                visited.add(next_coords)
                area += 1
                next_positions.extend([next_coords.step(d) for d in Directions.CARDINALS])
        total_price += area * perimeter

    return total_price

def part2(grid: Grid[str]):
    visited: Set[Coordinate] = set()
    total_price = 0
    for value, coords in grid.elements():
        if coords in visited:
            continue

        # { (coordinate in region, direction) }
        borders: Set[Tuple[Coordinate, Direction]] = set()

        # Floodfill from here
        visited.add(coords)
        area = 1
        next_positions = [(coords.step(d), d) for d in Directions.CARDINALS]
        while next_positions:
            (next_coords, direction) = next_positions.pop()
            if not grid.is_in_bounds(next_coords):
                borders.add((next_coords, direction))
            elif grid.get(next_coords) != value:
                borders.add((next_coords, direction))
            elif next_coords in visited:
                continue
            else:
                visited.add(next_coords)
                area += 1
                next_positions.extend([(next_coords.step(d), d) for d in Directions.CARDINALS])

        num_sides = 0
        while borders:
            # Reconcile the number of border elements
            border_coord, direction = borders.pop()
            num_sides += 1

            for next_direction in [direction.rotate(90), direction.rotate(-90)]:
                next_coord = border_coord.step(next_direction)
                while (next_coord, direction) in borders:
                    borders.discard((next_coord, direction))
                    next_coord = next_coord.step(next_direction)

        total_price += area * num_sides

    return total_price

def main():
    grid = Grid.parse_file_as_grid("inputs/day12")
    print(f"Part 1: {part1(grid)}")
    print(f"Part 2: {part2(grid)}")

if __name__=="__main__":
    main()
