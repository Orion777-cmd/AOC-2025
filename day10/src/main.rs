use std::fs::read_to_string;

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

    println!("{machines:#?}");

    let res_one = part_one(&machines);

    println!("solution for first problem is => {res_one}");
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

