pub type Grid<T> = Vec<Vec<T>>;

pub fn parse_grid(input: &str) -> Grid<char> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn parse_transform_grid<F, T>(input: &str, transform: F) -> Grid<T>
where
    F: Fn(char) -> T,
{
    input
        .lines()
        .map(|l| l.chars().map(|c| transform(c)).collect())
        .collect()
}

pub fn parse_grids_separated_by_newline(input: &str) -> Vec<Grid<char>> {
    input.split("\n\n").map(parse_grid).collect()
}

pub fn transpose_grid<T: Copy>(grid: &Grid<T>) -> Grid<T> {
    (0..grid[0].len())
        .map(|col_idx| grid.iter().map(|row| row[col_idx]).collect())
        .collect()
}

pub fn print_grid(grid: &Grid<char>) {
    for row in grid {
        println!("{}", String::from_iter(row.iter()));
    }
}
