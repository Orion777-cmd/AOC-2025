use std::fs::read_to_string;
use geo::{Coord, Polygon, Rect, Covers, LineString};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let points: Vec<Point> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split(',').map(|y| y.parse::<i64>().unwrap()).collect();

            Point { x: nums[0], y: nums[1]}
        })
        .collect();

    let res_one = part_one(&points);

    println!("Solution for first problem is => {res_one}");

    let res_two = part_two(&points);

    println!("Solution for second problem is => {res_two}");
}


fn part_one(points: &Vec<Point>) -> i64 {
    let n = points.len();

    let mut max_area = 0;
    for i in 0..n {
        for j in 0.. n {
            if i == j {
                continue;
            }

            let length = points[i].x - points[j].x + 1;
            let width = points[i].y - points[j].y + 1;

            let area = (length * width).abs();

            max_area = max_area.max(area);
        }
    }

    max_area
}



fn part_two(points: &Vec<Point>) -> i64 {
    let coords: Vec<Coord<f64>> = points
        .iter()
        .map(|p| Coord { x: p.x as f64, y: p.y as f64 })
        .collect();

    let outer = LineString::from(coords);
    let poly: Polygon<f64> = Polygon::new(outer, vec![]);

    let n = points.len();
    let mut max_area = 0i64;

    for i in 0..n {
        for j in i + 1..n {
            if let Some(area) = area(
                (points[i].x, points[i].y),
                (points[j].x, points[j].y),
                &poly,
            ) {
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

fn area(
    (x1, y1): (i64, i64),
    (x2, y2): (i64, i64),
    poly: &Polygon<f64>,
) -> Option<i64> {
    let (left, right) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

    if left == right || top == bottom {
        return None;
    }

    let rect = Rect::new(
        Coord { x: left as f64, y: top as f64 },
        Coord { x: right as f64, y: bottom as f64 },
    );

    if poly.covers(&rect) {
        Some((right - left + 1) * (bottom - top + 1))
    } else {
        None
    }
}

