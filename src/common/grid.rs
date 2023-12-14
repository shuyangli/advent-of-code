pub type Grid = Vec<Vec<char>>;

pub fn parse_grid(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn parse_grids_separated_by_newline(input: &str) -> Vec<Grid> {
    input.split("\n\n").map(parse_grid).collect()
}

pub fn transpose_grid(grid: &Grid) -> Grid {
    (0..grid[0].len())
        .map(|col_idx| grid.iter().map(|row| row[col_idx]).collect())
        .collect()
}

pub fn print_grid(grid: &Grid) {
    for row in grid {
        println!("{}", String::from_iter(row.iter()));
    }
}
