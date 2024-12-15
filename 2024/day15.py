from lib import Coordinate, Direction, Directions, Grid
from typing import List, Dict

def char_to_direction(char: str) -> Direction:
    if char == "^":
        return Directions.NORTH
    elif char == "<":
        return Directions.WEST
    elif char == ">":
        return Directions.EAST
    elif char == "v":
        return Directions.SOUTH
    else:
        raise ValueError(f"Invalid operation {char}")

def move_in_direction_part_1(grid: Grid[str], robot_coords: Coordinate, direction: Direction) -> Coordinate:
    next_coords = robot_coords.step(direction)
    item_at_next_coords = grid.get(next_coords)
    if item_at_next_coords == ".":
        return next_coords
    elif item_at_next_coords == "#":
        return robot_coords
    elif item_at_next_coords != "O":
        raise ValueError(f"Invalid item found: {item_at_next_coords}")

    final_coords = next_coords
    while grid.get(final_coords) == "O":
        final_coords = final_coords.step(direction)
    if grid.get(final_coords) == "#":
        # We hit a wall, nothing happens
        return robot_coords
    elif grid.get(final_coords) == ".":
        # Found an empty space
        grid.set(final_coords, "O")
        grid.set(next_coords, ".")
        return next_coords
    else:
        raise ValueError(f"Invalid item found: {grid.get(final_coords)}")

def part1(grid: Grid[str], instructions: List[Direction]):
    robot_position = grid.find("@")
    grid.set(robot_position, ".")

    for direction in instructions:
        robot_position = move_in_direction_part_1(grid, robot_position, direction)

    coordinate_sum = 0
    for value, coordinate in grid.elements():
        if value == "O":
            coordinate_sum += coordinate.i * 100 + coordinate.j
    return coordinate_sum

def move_in_direction_part_2(grid: Grid[str], robot_coords: Coordinate, direction: Direction) -> Coordinate:
    next_robot_coords = robot_coords.step(direction)

    # Treat positions_to_check as an ordered set
    positions_to_check: List[Coordinate] = [next_robot_coords]
    positions_to_move: List[Coordinate] = []
    i = 0
    can_move = True
    while True:
        if i >= len(positions_to_check):
            break
        next_coords = positions_to_check[i]
        item_at_next_coords = grid.get(next_coords)

        if item_at_next_coords == ".":
            # Do nothing, since we can move there
            pass
        elif item_at_next_coords == "#":
            # We've hit a wall, nothing happens at all
            can_move = False
            break
        elif item_at_next_coords == "[":
            if direction == Directions.EAST:
                positions_to_check.append(next_coords.step(direction, 2))
            elif direction == Directions.WEST:
                raise ValueError("Unexpectedly encountered [ when traversing west!")
            else:
                positions_to_check.extend([next_coords.step(direction), next_coords.step(direction).step(Directions.EAST)])

            # Annoying! This should be an ordered_set.
            if next_coords not in positions_to_move:
                positions_to_move.append(next_coords)
            if next_coords.step(Directions.EAST) not in positions_to_move:
                positions_to_move.append(next_coords.step(Directions.EAST))
        elif item_at_next_coords == "]":
            if direction == Directions.WEST:
                positions_to_check.append(next_coords.step(direction, 2))
            elif direction == Directions.EAST:
                raise ValueError("Unexpectedly encountered ] when traversing east!")
            else:
                positions_to_check.extend([next_coords.step(direction), next_coords.step(direction).step(Directions.WEST)])

            # Annoying! This should be an ordered_set.
            if next_coords not in positions_to_move:
                positions_to_move.append(next_coords)
            if next_coords.step(Directions.WEST) not in positions_to_move:
                positions_to_move.append(next_coords.step(Directions.WEST))
        i += 1
    if not can_move:
        return robot_coords

    # In reverse order, shift everything by direction
    for p in reversed(positions_to_move):
        grid.set(p.step(direction), grid.get(p))
        grid.set(p, ".")
    return next_robot_coords

def part2(grid: Grid[str], instructions: List[Direction]):
    robot_position = grid.find("@")
    grid.set(robot_position, ".")

    for direction in instructions:
        robot_position = move_in_direction_part_2(grid, robot_position, direction)

    coordinate_sum = 0
    for value, coordinate in grid.elements():
        if value == "[":
            coordinate_sum += coordinate.i * 100 + coordinate.j
    return coordinate_sum

def main():
    with open("inputs/day15") as file:
        lines = file.readlines()
        grid_str = ""
        i = 0
        while i < len(lines):
            if not lines[i].strip():
                break
            grid_str += lines[i]
            i += 1

        grid: Grid[str] = Grid.parse_string_as_grid(grid_str)
        i += 1

        instructions = ""

        while i < len(lines):
            instructions += lines[i].strip()
            i += 1

    instructions = [char_to_direction(char) for char in instructions]

    grid_part_2 = []
    for line in grid:
        new_line = []
        for char in line:
            if char == "#":
                new_line.extend(["#", "#"])
            elif char == ".":
                new_line.extend([".", "."])
            elif char == "O":
                new_line.extend(["[", "]"])
            elif char == "@":
                new_line.extend(["@", "."])
        grid_part_2.append(new_line)
    grid_part_2 = Grid(grid=grid_part_2)

    print(f"Part 1: {part1(grid, instructions)}")
    print(f"Part 2: {part2(grid_part_2, instructions)}")

if __name__=="__main__":
    main()
