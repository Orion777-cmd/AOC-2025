use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    
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

    println!("{res}");
}
