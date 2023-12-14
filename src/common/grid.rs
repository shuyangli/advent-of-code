pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", String::from_iter(row.iter()));
    }
}
