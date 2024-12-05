from typing import TypeVar, Generic, List
from enum import Enum

T = TypeVar('T')

class Direction(Generic[T]):
    def __init__(self, dx: T, dy: T) -> None:
        self.dx = dx
        self.dy = dy

    def dx(self) -> T:
        return self.dx

    def dy(self) -> T:
        return self.dy

class Directions:
    NORTH = Direction(0, -1)
    SOUTH = Direction(0, 1)
    EAST = Direction(1, 0)
    WEST = Direction(-1, 0)
    NORTHEAST = Direction(1, -1)
    NORTHWEST = Direction(-1, -1)
    SOUTHEAST = Direction(1, 1)
    SOUTHWEST = Direction(-1, 1)
    FOUR_DIRECTIONS: List[Direction] = [
        NORTH,
        SOUTH,
        EAST,
        WEST,
    ]
    EIGHT_DIRECTIONS: List[Direction] = [
        NORTH,
        SOUTH,
        EAST,
        WEST,
        NORTHEAST,
        NORTHWEST,
        SOUTHEAST,
        SOUTHWEST,
    ]

class Coordinate(Generic[T]):
    def __init__(self, x: T, y: T) -> None:
        self.x = x
        self.y = y
    
    def i(self) -> T:
        return self.x

    def j(self) -> T:
        return self.y

    def x(self) -> T:
        return self.x

    def y(self) -> T:
        return self.y

    def move(self, direction: Direction) -> "Coordinate[T]":
        return Coordinate(self.x + direction.dx, self.y + direction.dy)
    
    def step(self, direction: Direction, steps: int = 1) -> "Coordinate[T]":
        return Coordinate(self.x + steps * direction.dx, self.y + steps * direction.dy)

class Grid(Generic[T]):
    def __init__(self, grid: List[List[T]]) -> None:
        self.grid = grid

    # If (i, j) is out of bounds, return False.
    def matches(self, coord: Coordinate[int], value: T) -> bool:
        if coord.i() < 0 or coord.i() >= len(self.grid) or coord.j() < 0 or coord.j() >= len(self.grid[0]):
            return False
        return self.grid[coord.i()][coord.j()] == value
    
    def num_rows(self) -> int:
        return len(self.grid)

    def num_columns(self) -> int:
        if len(self.grid) == 0:
            return 0
        return len(self.grid[0])

def parse_file_as_grid(filename: str) -> Grid[str]:
    with open(filename, "r") as file:
        lines = file.readlines()
    return Grid([list(l.strip()) for l in lines])
