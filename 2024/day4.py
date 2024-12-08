from lib import Grid, Coordinate, Directions

def part1():
    grid = Grid.parse_file_as_grid("inputs/day4")

    count = 0
    for i in range(grid.num_rows()):
        for j in range(grid.num_columns()):
            center = Coordinate(i, j)
            for dir in Directions.ALL:
                if (
                    grid.matches(center, "X")
                    and grid.matches(center.step(dir, 1), "M")
                    and grid.matches(center.step(dir, 2), "A")
                    and grid.matches(center.step(dir, 3), "S")
                ):
                    count += 1

    return count

def part2():
    grid = Grid.parse_file_as_grid("inputs/day4")

    count = 0
    for i in range(grid.num_rows()):
        for j in range(grid.num_columns()):
            center = Coordinate(i, j)
            if not grid.matches(center, "A"):
                continue
            passing_directions = 0
            for dir in Directions.ORDINALS:
                if grid.matches(center.step(dir), "M") and grid.matches(center.step(dir, -1), "S"):
                    passing_directions += 1
            if passing_directions == 2:
                count += 1

    return count

def main():
    print(f"Part 1: {part1()}")
    print(f"Part 2: {part2()}")

if __name__=="__main__":
    main()
