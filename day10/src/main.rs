use std::fs::read_to_string;
use std::f64;

#[derive(Debug)]
struct Machine {
    final_state_diagram: Vec<u32>,
    button_wiring: Vec<Vec<u32>>,
    joltage_req: Vec<u32>,
}

fn main() {
    let content = read_to_string("input.txt").unwrap();

    let machines: Vec<Machine> = content
        .lines()
        .map(|line| parse_machine(line))
        .collect();

    let res_one = part_one(&machines);

    println!("solution for first problem is => {res_one}");

    let res_two = part_two(&machines);

    println!("solution for second problem is => {res_two}");
}


fn part_two(machines: &Vec<Machine>) -> u32 {
    machines.iter().map(|m| solve_machine(m)).sum()
}

fn solve_machine(m: &Machine) -> u32 {
    let rows = m.joltage_req.len();
    let cols = m.button_wiring.len();

    // Build matrix A|b in float64
    let mut mat = vec![vec![0f64; cols + 1]; rows];
    for r in 0..rows {
        mat[r][cols] = m.joltage_req[r] as f64;
    }
    for (c, wiring) in m.button_wiring.iter().enumerate() {
        for &r in wiring {
            mat[r as usize][c] += 1.0;
        }
    }

    const EPS: f64 = 1e-9;

    // Gaussian elimination
    let mut pivot_row = vec![-1isize; cols];
    let mut pr = 0;

    for c in 0..cols {
        if pr >= rows { break; }

        let mut sel = -1;
        for r in pr..rows {
            if mat[r][c].abs() > EPS {
                sel = r as isize;
                break;
            }
        }
        if sel == -1 { continue; }

        mat.swap(pr, sel as usize);

        let div = mat[pr][c];
        for k in c..=cols {
            mat[pr][k] /= div;
        }

        for r in 0..rows {
            if r != pr && mat[r][c].abs() > EPS {
                let f = mat[r][c];
                for k in c..=cols {
                    mat[r][k] -= f * mat[pr][k];
                }
            }
        }

        pivot_row[c] = pr as isize;
        pr += 1;
    }

    // Check for contradiction
    for r in pr..rows {
        if mat[r][cols].abs() > EPS {
            return 0; // No solution
        }
    }

    // Identify free variables
    let free_vars: Vec<usize> = (0..cols)
        .filter(|&c| pivot_row[c] == -1)
        .collect();

    // Search space for free variables
    // target maximum value per variable dimension
    let max_t = *m.joltage_req.iter().max().unwrap() as usize;
    let mut best = u32::MAX;
    let mut free_vals = vec![0usize; free_vars.len()];

    fn dfs_free(
        idx: usize,
        free_vars: &Vec<usize>,
        mat: &Vec<Vec<f64>>,
        pivot_row: &Vec<isize>,
        free_vals: &mut Vec<usize>,
        max_t: usize,
        best: &mut u32,
        cols: usize,
    ) {
        if idx == free_vars.len() {
            // Reconstruct full solution vector
            let mut x = vec![0f64; cols];

            // Set free vars
            for (i, &fv) in free_vars.iter().enumerate() {
                x[fv] = free_vals[i] as f64;
            }

            // Solve pivot vars from row equations
            for c in (0..cols).rev() {
                let pr = pivot_row[c];
                if pr == -1 { continue; }

                let r = pr as usize;
                let mut sum = mat[r][cols];

                for cc in (c + 1)..cols {
                    sum -= mat[r][cc] * x[cc];
                }
                x[c] = sum;
            }

            // Check integer + non-negative
            let mut total = 0u32;
            for &v in &x {
                let iv = v.round();
                if (v - iv).abs() > 1e-6 { return; }
                if iv < 0.0 { return; }
                total += iv as u32;
            }

            // Record best
            *best = (*best).min(total);
            return;
        }

        // Try free value from 0 to max range
        for v in 0..=max_t {
            if (*best) as usize <= v { break; }
            free_vals[idx] = v;
            dfs_free(idx + 1, free_vars, mat, pivot_row, free_vals, max_t, best, cols);
        }
    }

    dfs_free(0, &free_vars, &mat, &pivot_row, &mut free_vals, max_t, &mut best, cols);

    if best == u32::MAX { 0 } else { best }
}

fn part_one(machines: &Vec<Machine>) -> u32 {
    let mut res = 0;

    for machine in machines {
        let n = machine.button_wiring.len();
        let final_state = &machine.final_state_diagram;

        let mut best = u32::MAX;

        for mask in 0..(1u32 << n) {
            let mut state = vec![0u32; final_state.len()];

            for button in 0..n {
                if (mask & (1 << button)) != 0 {
                    // Press this button → toggle each index
                    for &idx in &machine.button_wiring[button] {
                        state[idx as usize] ^= 1;
                    }
                }
            }

            if state == *final_state {
                let presses = mask.count_ones();
                best = best.min(presses);
            }
        }

        res += best;
    }

    res
}

fn parse_machine(line: &str) -> Machine {
    let final_state_str = extract_between(line, "[", "]");
    let final_state_diagram = final_state_str
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("Invalid char in diagram"),
        })
        .collect();

    let button_wiring: Vec<Vec<u32>> = extract_all_between(line, "(", ")")
        .into_iter()
        .map(|group| {
            if group.is_empty() {
                vec![]
            } else {
                group.split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect()
            }
        })
        .collect();

    let joltage_str = extract_between(line, "{", "}");
    let joltage_req = joltage_str
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    Machine {
        final_state_diagram,
        button_wiring,
        joltage_req,
    }
}

fn extract_between<'a>(s: &'a str, start: &str, end: &str) -> String {
    let a = s.find(start).unwrap() + start.len();
    let b = s[a..].find(end).unwrap() + a;
    s[a..b].to_string()
}

fn extract_all_between<'a>(s: &'a str, start: &str, end: &str) -> Vec<String> {
    let mut out = vec![];
    let mut rest = s;

    while let Some(a) = rest.find(start) {
        let after_a = &rest[a + start.len()..];
        if let Some(b) = after_a.find(end) {
            out.push(after_a[..b].to_string());
            rest = &after_a[b + end.len()..];
        } else {
            break;
        }
    }

    out
}

