use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let part_one_sol = part_one(&lines);

    println!("The Password is in the first part is => {part_one_sol}");

    let part_two_sol =  part_two(&lines);

    println!("The password using the 0x434C49434B password method is => {part_two_sol}");
}

fn part_two(lines: &Vec<String>) -> i64 {
    let mut password = 0i64;
    let mut pos = 50i32;

    for line in lines {
        let dir = line.as_bytes()[0];
        let steps: i64 = line[1..].parse().unwrap();

        let (full_laps, rem) = (steps / 100, steps % 100);
        password += full_laps;

        let mut temp_pos = pos;
        for _ in 0..rem {
            if dir == b'R' {
                temp_pos += 1;
                if temp_pos == 100 { temp_pos = 0; }
            } else {
                temp_pos -= 1;
                if temp_pos == -1 { temp_pos = 99; }
            }
            if temp_pos == 0 {
                password += 1;
            }
        }

        if dir == b'R' {
            pos = (pos + rem as i32) % 100;
        } else {
            pos = (pos - rem as i32 + 100) % 100;
        }
    }

    password
}
fn part_one(lines: &Vec<String>) -> i32 {
    let mut password = 0;

    let mut starting_position = 50;

    for line in lines {
        let mut chars = line.chars();

        let direction = chars.next().unwrap();

        let rest: String = chars.collect();

        let mut amount: i32 = rest.parse().unwrap();
        amount %= 100;

        match direction {
            'R' => {
                starting_position += amount;
                if starting_position > 99 {
                    starting_position -= 100;
                }

                if starting_position == 0 {
                    password += 1;
                }
            },
            'L' => {
                starting_position -= amount;
                if starting_position < 0 {
                    starting_position += 100;
                }

                if starting_position == 0 {
                    password += 1;
                }
            },
            _ => {
                println!("Do not now this direction");
            }
        }
    }


    password
}
