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

    let res_two = part_two(&grid);
    println!("solution for second problem => {res_two}");
}

fn part_two(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    if rows == 0 { return 0; }
    let cols = grid[0].len();

    // find S
    let mut start_row = None;
    let mut start_col = 0usize;
    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|&ch| ch == 'S') {
            start_row = Some(r);
            start_col = c;
            break;
        }
    }
    let srow = match start_row {
        Some(r) => r,
        None => return 0,
    };

    let start_x = srow + 1;
    let start_y = start_col;

    let mut memo: Vec<Vec<Option<usize>>> = vec![vec![None; cols]; rows];
    let mut visiting: HashSet<(usize, usize)> = HashSet::new();

    if start_x >= rows || start_y >= cols { return 0; }

    count_from(start_x, start_y, grid, &mut memo, &mut visiting)
}

fn count_from(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    memo: &mut Vec<Vec<Option<usize>>>,
    visiting: &mut HashSet<(usize, usize)>,
) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    // if we've fallen off the bottom or off the sides, that is a completed timeline
    if x >= rows || y >= cols {
        return 1;
    }

    if visiting.contains(&(x, y)) {
        return 0;
    }

    if let Some(val) = memo[x][y] {
        return val;
    }

    visiting.insert((x, y));

    let res = match grid[x][y] {
        '^' => {
            let left = if y > 0 {
                count_from(x, y - 1, grid, memo, visiting)
            } else {
                1
            };
            let right = if y + 1 < cols {
                count_from(x, y + 1, grid, memo, visiting)
            } else {
                1
            };
            left + right
        }
        _ => {
            if x + 1 < rows {
                count_from(x + 1, y, grid, memo, visiting)
            } else {
                1
            }
        }
    };

    visiting.remove(&(x, y));
    memo[x][y] = Some(res);
    res
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

