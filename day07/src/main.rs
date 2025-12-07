use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let grid: Vec<Vec<char>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let res_one = part_one(&grid);
    println!("first res is => {res_one}");
}

fn part_one(grid: &Vec<Vec<char>>) -> i32 {
    let start = grid[0].iter().position(|x| *x == 'S').unwrap();

    println!("{start:?}");
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let beam_split = dfs(0, start, &grid, &mut seen);

    beam_split
}

fn dfs(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    seen: &mut HashSet<(usize, usize)>
) -> i32 {
    if x >= grid.len() || y >= grid[0].len() || seen.contains(&(x, y)) {
        return 0;
    }

    seen.insert((x, y));

    let mut res = 0;

    if grid[x][y] == '^' {
        res += 1;
        if y > 0 {
            res +=  dfs(x, y - 1, grid, seen);
        }

        if y + 1 < grid[0].len() {
            res += dfs(x, y + 1, grid, seen);
        }

    } else {
        if x + 1 < grid.len() {
            res += dfs(x + 1, y, grid, seen);
        }
    }

    res
}

