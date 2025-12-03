use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("test_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let res = part_one(&lines);

    println!("The answer is => {res}");

    let res_two = part_two(&lines);

    println!("The answer for second problem is => {res_two}");

}

fn part_one(lines: &Vec<String>) -> i32 {
    let mut res = 0;

    for line in lines {
        let nums: Vec<char> = line.chars().collect();

        let mut cur = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                let cur_num: i32 = format!("{}{}", nums[i], nums[j]).parse().unwrap();

                cur = cur.max(cur_num);
            }
        }

        res += cur;
    }

    res
}

fn part_two(lines: &Vec<String>) -> i64 {
    let mut res: i64 = 0;

    for line in lines {
        let num = largest_k_digit_number(&line, 12).unwrap();
        let value: i64 = num.parse().unwrap();

        println!("value is => {value}");

        res += value;
    }

    res
}

fn largest_k_digit_number(line: &str, k: usize) -> Option<String> {
    let digits: Vec<i32> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i32))
        .collect();

    if digits.len() < k {
        return None;
    }

    let mut result = String::new();
    let mut start = 0;

    for remaining in (0..k).rev() {
        // end index inclusive for the search window
        let end = digits.len() - remaining;
        let mut best_digit = -1;
        let mut best_index = start;

        for i in start..end {
            if digits[i] > best_digit {
                best_digit = digits[i];
                best_index = i;
            }
        }

        result.push(char::from_digit(best_digit as u32, 10).unwrap());
        start = best_index + 1;
    }

    Some(result)
}
