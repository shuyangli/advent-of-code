from collections import defaultdict
from typing import DefaultDict, List
from lib import Grid, Coordinate, Direction
from math import gcd

def part1(grid: Grid[str]):
    letters_to_coords: DefaultDict[str, List[Coordinate]] = defaultdict(lambda: [])

    for value, coords in grid.elements():
        if value != ".":
            letters_to_coords[value].append(coords)
    
    antinode_coords = set()
    for _, coords in letters_to_coords.items():
        for i in range(len(coords)):
            for j in range(i + 1, len(coords)):
                # For every pair, calculate the positions
                delta_vec = Direction(coords[i].i - coords[j].i, coords[i].j - coords[j].j)
                coords_1 = coords[i].step(delta_vec, 1)
                if grid.is_in_bounds(coords_1):
                    antinode_coords.add(coords_1)
                coords_2 = coords[j].step(delta_vec, -1)
                if grid.is_in_bounds(coords_2):
                    antinode_coords.add(coords_2)
    return len(antinode_coords)

def part2(grid: Grid[str]):
    letters_to_coords: DefaultDict[str, List[Coordinate]] = defaultdict(lambda: [])

    for value, coords in grid.elements():
        if value != ".":
            letters_to_coords[value].append(coords)
    
    antinode_coords = set()
    for _, coords in letters_to_coords.items():
        for i in range(len(coords)):
            for j in range(i + 1, len(coords)):
                # For every pair, calculate the positions
                # Reduce the delta to the smallest fraction
                d_i = coords[i].i - coords[j].i
                d_j = coords[i].j - coords[j].j
                vec_gcd = gcd(d_i, d_j)
                delta_vec = Direction(d_i // vec_gcd, d_j // vec_gcd)

                scale = 0
                while True:
                    new_coords = coords[i].step(delta_vec, scale)
                    if not grid.is_in_bounds(new_coords):
                        break
                    antinode_coords.add(new_coords)
                    scale += 1

                scale = -1
                while True:
                    new_coords = coords[i].step(delta_vec, scale)
                    if not grid.is_in_bounds(new_coords):
                        break
                    antinode_coords.add(new_coords)
                    scale -= 1
    return len(antinode_coords)

def main():
    grid = Grid.parse_file_as_grid("inputs/day8")
    print(f"Part 1: {part1(grid)}")
    print(f"Part 2: {part2(grid)}")

if __name__=="__main__":
    main()
