use std::fs::read_to_string;
use std::collections::{HashSet};

fn main() {
    let grid: Vec<Vec<char>> = read_to_string("test_input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let mut seen = HashSet::new();
    let res = iterative_dfs(0, 0, &mut seen, &grid);

    println!("Result is {res}");
}

fn iterative_dfs(start_i: usize, start_j: usize, seen: &mut HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = vec![
        [1, 0], [-1, 0], [0, 1], [0, -1],
        [-1, -1], [1, 1], [-1, 1], [1, -1]
    ];

    let mut stack = vec![(start_i as isize, start_j as isize)];
    seen.insert((start_i, start_j));

    let mut res = 0;

    while let Some((i, j)) = stack.pop() {
        let i = i as usize;
        let j = j as usize;

        let mut papers = 0;
        if grid[i][j] == '@' {
            for dir in &directions {
                let ni = i as isize + dir[0];
                let nj = j as isize + dir[1];
                if ni >= 0 && nj >= 0 && ni < rows as isize && nj < cols as isize {
                    if grid[ni as usize][nj as usize] == '@' {
                        papers += 1;
                    }
                }
            }
            if papers < 4 {
                res += 1;
            }
        }

        for dir in &directions {
            let ni = i as isize + dir[0];
            let nj = j as isize + dir[1];
            if ni >= 0 && nj >= 0 && ni < rows as isize && nj < cols as isize {
                let ni = ni as usize;
                let nj = nj as usize;
                if !seen.contains(&(ni, nj)) {
                    seen.insert((ni, nj));
                    stack.push((ni as isize, nj as isize));
                }
            }
        }
    }

    res
}
