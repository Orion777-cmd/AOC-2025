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
        let values: Vec<&str> = splited[1]
            .split_whitespace()
            .map(|v| v.trim())
            .collect();

        graph.insert(key, values);
    }

    let res_one = part_one(&graph);

    println!("the number of paths towards the output is => {res_one}");

    let res_two = part_two(&graph);

    println!("The number of paths pasing through fft and dac is => {res_two}");
}

fn part_one(graph: &HashMap<&str, Vec<&str>>) -> u64 {
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

fn dfs(
    start: &str,
    end: &str,
    graph: &HashMap<&str, Vec<&str>>,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if start == end {
        return 1
    }

    if let Some(val) = memo.get(start) {
        return *val
    }

    let mut res = 0;
    if let Some(neighbors) = graph.get(start) {
        for  &neigh in neighbors {
            res += dfs(neigh, end, graph, memo);
        }
    }

    memo.insert(start.to_string(), res);

    res
}


fn part_two(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut memo1 = HashMap::new();
    let mut memo2 = HashMap::new();
    let mut memo3 = HashMap::new();
    let mut memo4 = HashMap::new();
    let mut memo5 = HashMap::new();
    let mut memo6 = HashMap::new();

    let p1 = dfs("svr", "dac", graph, &mut memo1)
           * dfs("dac", "fft", graph, &mut memo2)
           * dfs("fft", "out", graph, &mut memo3);

    let p2 = dfs("svr", "fft", graph, &mut memo4)
           * dfs("fft", "dac", graph, &mut memo5)
           * dfs("dac", "out", graph, &mut memo6);

    p1 + p2
}


