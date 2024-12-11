from lib import Grid, Coordinate, Directions

class LoopError(Exception):
    pass

def traverse(grid: Grid):
    position = grid.find("^")
    direction = Directions.NORTH

    visited = set()
    visited.add((position.i, position.j))

    past_states = set()
    past_states.add((position.i, position.j, direction))

    while position.i >= 0 and position.i < grid.num_rows():
        while position.j >= 0 and position.j < grid.num_columns():
            next = position.step(direction)
            if (next.i, next.j, direction) in past_states:
                raise LoopError
            try:
                if grid.get(next) == "#":
                    # Currently facing an obstacle
                    direction = direction.rotate(90)
                else:
                    position = next
                    visited.add((position.i, position.j))
                    past_states.add((position.i, position.j, direction))
            except ValueError:
                return visited

def part1(grid: Grid):
    visited = traverse(grid)
    return len(visited)

def part2(grid: Grid):
    visited = traverse(grid)
    num_possible_new_obstructions = 0

    for i in range(grid.num_rows()):
        for j in range(grid.num_columns()):
            if (i, j) not in visited:
                # If the guard doesn't hit that position, we don't need to try anyways
                continue
            if grid.get(Coordinate(i, j)) != ".":
                # Probably don't need this but good to check
                continue

            # Try replacing that position with an obstacle and play it out
            grid.grid[i][j] = "#"
            try:
                _ = traverse(grid)
            except LoopError:
                num_possible_new_obstructions += 1
            finally:
                grid.grid[i][j] = "."

    return num_possible_new_obstructions

def main():
    grid = Grid.parse_file_as_grid("inputs/day6")
    print(f"Part 1: {part1(grid)}")
    print(f"Part 2: {part2(grid)}")

if __name__=="__main__":
    main()
