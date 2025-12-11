use std::fs::read_to_string;
use std::collections::{HashMap, VecDeque};

fn main() {
    let content: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in &content {
        let splited: Vec<&str> = line.split(':').collect();

        let key = splited[0].trim();
        let values: Vec<&str> = splited[1].split_whitespace().collect();

        graph.insert(key, values);
    }

    let res_one = part_one(&graph);

    println!("the number of paths towards the output is => {res_one}");
}

fn part_one(graph: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut paths = 0;

    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("you");

   while let Some(parent) = queue.pop_front() {
        if parent == "out" {
            paths += 1;
        }

        if let Some(neighbors) = graph.get(parent) {
            for &neigh in neighbors {
                queue.push_back(neigh);
            }
        }
    }

    paths
}
