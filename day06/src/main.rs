use std::fs::read_to_string;

fn main() {
    let content = read_to_string("input.txt").unwrap();

    let lines: Vec<Vec<&str>> = content
        .lines()
        .map(|x| x.split_whitespace().collect())
        .collect();

    let res_one = part_one(lines);

    println!("part one solution is => {res_one}");
}

fn part_one(lines: Vec<Vec<&str>>) -> i64 {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut res:i64 = 0;
    let ops = &lines[rows-1];

    for col in 0..cols {
        let op = ops[col].chars().next().unwrap();
        let mut temp: i64 = if op == '*' { 1 } else { 0 };
        for row in 0..rows-1 {

            match op {
                '*' => temp *= lines[row][col].parse::<i64>().unwrap(),
                '+' => temp += lines[row][col].parse::<i64>().unwrap(),
                _ => unreachable!(),
            }
        }

        res += temp;
    }

    res
}
