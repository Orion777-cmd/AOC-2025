use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let grid: Vec<Vec<char>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let mut seen = HashSet::new();
    let res = iterative_dfs(0, 0, &mut seen, &grid);

    println!("Result is {res}");

    let mut modified_grid = grid.clone();
    let mut res_two = 0;

    loop {
        let mut new_seen = HashSet::new();
        let mut ans = 0;

        for i in 0..modified_grid.len() {
            for j in 0..modified_grid[0].len() {
                if modified_grid[i][j] == '@' && !new_seen.contains(&(i,j)) {
                    let (count, new_grid) = iterative_dfs_two(i, j, &mut new_seen, modified_grid.clone());
                    ans += count;
                    modified_grid = new_grid;
                }
            }
        }

        if ans == 0 {
            break;
        }

        res_two += ans;
    }

    println!("Answer for second is {res_two}");

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


fn iterative_dfs_two(
    start_i: usize,
    start_j: usize,
    seen: &mut HashSet<(usize, usize)>,
    mut grid: Vec<Vec<char>>, // take ownership so we can modify
) -> (i32, Vec<Vec<char>>) {
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

        if grid[i][j] == '@' {
            let mut papers = 0;
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
                grid[i][j] = '.';
            }
        }

        for dir in &directions {
            let ni = i as isize + dir[0];
            let nj = j as isize + dir[1];
            if ni >= 0 && nj >= 0 && ni < rows as isize && nj < cols as isize {
                let ni_usize = ni as usize;
                let nj_usize = nj as usize;
                if !seen.contains(&(ni_usize, nj_usize)) {
                    seen.insert((ni_usize, nj_usize));
                    stack.push((ni, nj));
                }
            }
        }
    }

    (res, grid)
}
