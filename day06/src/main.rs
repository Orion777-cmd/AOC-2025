use std::fs::read_to_string;

fn main() {
    let content = read_to_string("input.txt").unwrap();

    let lines: Vec<Vec<&str>> = content
        .lines()
        .map(|x| x.split_whitespace().collect())
        .collect();

    let res_one = part_one(&lines);

    println!("part one solution is => {res_one}");

    let mut lines_two: Vec<Vec<char>> = content
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let max_width = lines_two.iter().map(|r| r.len()).max().unwrap();

    for row in lines_two.iter_mut() {
        row.resize(max_width, ' ');
    }
    let parsed_lines= parse_columns(lines_two);

    let res_two = part_two(&parsed_lines);

    println!("Part two solution is => {res_two}");
}

fn part_one(lines: &Vec<Vec<&str>>) -> i64 {
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

fn part_two(lines: &Vec<Vec<String>>) -> i64 {
    let rows = lines.len();
    let mut res: i64 = 0;

    for row in 0..rows{
        let op = lines[row].last().unwrap();
        let mut temp: i64 = if *op == '*'.to_string() { 1 } else { 0 };

        for i in 0..lines[row][0].len() {
            let mut cur_num = String::new();

            for num in &lines[row] {
                let num_chars: Vec<char> = num.chars().collect();

                if num_chars.len() == 1 {
                    continue;
                }
                if num_chars[i] != '0' {
                    cur_num.push_str(&num_chars[i].to_string());
                }
            }
            let val: i64 = match cur_num.parse() {
                Ok(v) => v,
                Err(_) => continue, // skip invalid numbers
            };
            match op.chars().next().unwrap() {
                '*' => temp *= val,
                '+' =>  temp += val,
                _ => unreachable!(),
            }
        }

        res += temp;
    }

    res
}


fn parse_columns(mut lines: Vec<Vec<char>>) -> Vec<Vec<String>> {
    let rows = lines.len();

    // Normalize all rows to the same width
    let max_width = lines.iter().map(|r| r.len()).max().unwrap();
    for row in lines.iter_mut() {
        row.resize(max_width, ' ');
    }

    let width = max_width;
    let mut columns: Vec<Vec<String>> = Vec::new();
    let mut c = 0;

    while c < width {
        // Skip blank space until hitting first real column
        if !lines.iter().any(|r| r[c].is_ascii_digit()) {
            c += 1;
            continue;
        }

        // Determine end of column
        let start = c;
        let mut end = c;
        while end < width && lines.iter().any(|r| !r[end].is_whitespace()) {
            end += 1;
        }

        // Extract column and pad numbers
        let mut col: Vec<String> = Vec::new();
        // Compute max length of digits in this column (excluding operator row)
        let max_len = lines[..rows - 1]
            .iter()
            .map(|r| r[start..end].iter().filter(|ch| !ch.is_whitespace()).count())
            .max()
            .unwrap();

        for r in 0..rows {
            let slice: String = lines[r][start..end].iter().collect();
            let trimmed = slice.trim();

            let padded = if r < rows - 1 {
                // Pad left if original slice starts with spaces, else pad right
                if slice.starts_with(' ') {
                    format!("{:0>width$}", trimmed, width = max_len)
                } else {
                    format!("{:0<width$}", trimmed, width = max_len)
                }
            } else {
                // Operator row
                trimmed.to_string()
            };

            col.push(padded);
        }

        columns.push(col);
        c = end;
    }

    columns
}

