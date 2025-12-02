use std::fs::read_to_string;

fn main() {
   let lines  = read_to_string("input.txt")
       .unwrap()
       .trim_end()
       .to_string();

    let ranges: Vec<&str> = lines.split(',').collect();

    let part_one_sol = part_one(&ranges);

    println!("part one solution is => {part_one_sol}");

    let part_two_sol = part_two(&ranges);

    println!("part two solution is => {part_two_sol}");
}

fn part_two(ranges: &Vec<&str>) -> i64 {
    let mut res: i64 = 0;
    for range in ranges {
        let mut parts = range.split('-');
        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();

        for cur_id in start..=end {
            let s = cur_id.to_string();
            let n = s.len();
            for i in 0..=n / 2 {
                let pat = &s[..=i];
                if is_repeated(&s, pat) {
                    res += cur_id;
                    break;
                }
            }

        }

    }

    res

}

fn is_repeated(num: &String, pat: &str) -> bool {
    let count = num.matches(pat).count();

    count >= 2 && count * pat.len() == num.len()
}

fn part_one(ranges: &Vec<&str>) -> i64 {
    let mut res: i64 = 0;
    for range in ranges {
        let mut parts = range.split('-');
        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();

        for cur_id in start..=end {
            let s = cur_id.to_string();

            let mid = s.len() / 2;

            let left = &s[..mid];
            let right = &s[mid..];

            if left == right {
                res += cur_id;
            }
        }

    }

    res

}

