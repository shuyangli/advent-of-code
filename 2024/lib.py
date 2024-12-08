from typing import TypeVar, Generic, List, Generator, Tuple

T = TypeVar('T')

class Direction(Generic[T]):
    def __init__(self, dx: T, dy: T) -> None:
        self._dx = dx
        self._dy = dy

    @property
    def dx(self) -> T:
        return self._dx

    @property
    def dy(self) -> T:
        return self._dy
    
    def rotate(self: "Direction[int]", degrees: int) -> "Direction[int]":
        starting_index = Directions.ALL.index(self)
        return Directions.ALL[(starting_index + degrees // 45) % len(Directions.ALL)]

    def __str__(self) -> str:
        return f"(dx={self.dx}, dy={self.dy})"
    
    def __eq__(self, other) -> bool:
        if not isinstance(other, Direction):
            return NotImplemented
        return self.dx == other.dx and self.dy == other.dy

    def __hash__(self) -> int:
        return hash((self.dx, self.dy))

class Directions:
    NORTH = Direction(-1, 0)
    SOUTH = Direction(1, 0)
    EAST = Direction(0, 1)
    WEST = Direction(0, -1)
    NORTHEAST = Direction(-1, 1)
    NORTHWEST = Direction(-1, -1)
    SOUTHEAST = Direction(1, 1)
    SOUTHWEST = Direction(1, -1)

    CARDINALS: List[Direction] = [
        NORTH,
        SOUTH,
        EAST,
        WEST,
    ]
    ORDINALS: List[Direction] = [
        NORTHEAST,
        NORTHWEST,
        SOUTHEAST,
        SOUTHWEST,
    ]
    ALL: List[Direction] = [
        NORTH,
        NORTHEAST,
        EAST,
        SOUTHEAST,
        SOUTH,
        SOUTHWEST,
        WEST,
        NORTHWEST,
    ]

class Coordinate(Generic[T]):
    def __init__(self, x: T, y: T) -> None:
        self.x = x
        self.y = y
    
    @property
    def i(self) -> T:
        return self.x
    
    @i.setter
    def i(self, value):
        self.x = value

    @property
    def j(self) -> T:
        return self.y
    
    @j.setter
    def j(self, value):
        self.y = value

    def __str__(self) -> str:
        return f"(x={self.x}, y={self.y})"
    
    def __eq__(self, other) -> bool:
        if not isinstance(other, Coordinate):
            return NotImplemented
        return self.x == other.x and self.y == other.y
    
    def __hash__(self):
        return hash((self.x, self.y))

    def move(self, direction: Direction) -> "Coordinate[T]":
        return Coordinate(self.x + direction.dx, self.y + direction.dy)
    
    def step(self, direction: Direction, steps: int = 1) -> "Coordinate[T]":
        return Coordinate(self.x + steps * direction.dx, self.y + steps * direction.dy)

class Grid(Generic[T]):
    def __init__(self, grid: List[List[T]]) -> None:
        self.grid = grid

    def parse_file_as_grid(filename: str) -> "Grid[str]":
        with open(filename, "r") as file:
            lines = file.readlines()
        return Grid([list(l.strip()) for l in lines])
    
    def at(self, coord: Coordinate[int]) -> T:
        if not self.is_in_bounds(coord):
            raise ValueError(f"Coordinate {coord} is out of bounds!")
        return self.grid[coord.i][coord.j]
    
    def find(self, value: T) -> Coordinate:
        for i in range(len(self.grid)):
            for j in range(len(self.grid[0])):
                if self.grid[i][j] == value:
                    return Coordinate(i, j)
        raise ValueError(f"Value '{value}' not found!")

    # If (i, j) is out of bounds, return False.
    def matches(self, coord: Coordinate[int], value: T) -> bool:
        if not self.is_in_bounds(coord):
            return False
        return self.grid[coord.i][coord.j] == value
    
    def is_in_bounds(self, coord: Coordinate[int]) -> bool:
        return coord.i >= 0 and coord.i < len(self.grid) and coord.j >= 0 and coord.j < len(self.grid[0])
    
    def num_rows(self) -> int:
        return len(self.grid)

    def num_columns(self) -> int:
        if len(self.grid) == 0:
            return 0
        return len(self.grid[0])
    
    def elements(self) -> Generator[Tuple[T, Coordinate[int]], None, None]:
        for i in range(len(self.grid)):
            for j in range(len(self.grid[0])):
                yield (self.grid[i][j], Coordinate(i, j))
