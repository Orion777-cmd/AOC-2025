use std::fs::read_to_string;

fn main() {
    let points: Vec<Vec<i64>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.split(',').map(|y| y.parse::<i64>().unwrap()).collect())
        .collect();

    let res_one = part_one(&points);

    println!("Solution for first problem is => {res_one}");
}


fn part_one(points: &Vec<Vec<i64>>) -> i64 {
    let n = points.len();

    let mut max_area = 0;
    for i in 0..n {
        for j in 0.. n {
            if i == j {
                continue;
            }

            let length = points[i][0] - points[j][0] + 1;
            let width = points[i][1] - points[j][1] + 1;

            let area = (length * width).abs();

            max_area = max_area.max(area);
        }
    }
    
    max_area
}
