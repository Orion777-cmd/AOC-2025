use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let res = part_one(&lines);

    println!("first solution: {res}");

    let res_two = part_two(&lines);

    println!("second solution: {res_two}");
}

fn part_two(lines: &Vec<String>) -> i64 {
    let mut ranges: Vec<Vec<i64>> = Vec::new();
    let mut res = 0;

    for line in lines {
        if line.is_empty() {
            break;
        }
        let nums: Vec<i64> = line
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        ranges.push(nums);
    }

    ranges.sort();

    let mut merged: Vec<Vec<i64>> = Vec::new();
    merged.push(ranges[0].clone());

    for i in 1..ranges.len() {
        let last = merged.last_mut().unwrap();

        if ranges[i][0] > last[1] {
            merged.push(ranges[i].clone());
        } else {
            last[1] = last[1].max(ranges[i][1]);
        }
    }

    println!("{merged:?}");

    for range in merged {
        res += range[1] - range[0] + 1;
    }

    res
}

fn part_one(lines: &Vec<String>) -> i64 {
    let mut ranges: Vec<Vec<i64>> = Vec::new();
    let mut res = 0;

    let mut to_indredients = false;

    for line in lines {
        if line.is_empty() {
            to_indredients = true;
            continue;
        }
        if !to_indredients {
            let nums: Vec<i64> = line.split('-').map(|x| x.parse().unwrap()).collect();

            ranges.push(nums.clone());
        } else {
            let cur_num: i64 = line.parse().unwrap();

            for ingredient in &ranges{
                if cur_num >= ingredient[0] && cur_num <= ingredient[1] {
                    res += 1;
                    break;
                }
            }
        }
    }

    res

}
