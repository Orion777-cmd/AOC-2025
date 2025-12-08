use std::fs::read_to_string;

#[derive(Clone)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb { return false; }

        if self.size[pa] < self.size[pb] {
            self.parent[pa] = pb;
            self.size[pb] += self.size[pa];
        } else {
            self.parent[pb] = pa;
            self.size[pa] += self.size[pb];
        }

        self.components -= 1;
        true
    }
}

fn main() {
let coords: Vec<[i32; 3]> = read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(|l| {
        let a: Vec<i32> = l.split(',').map(|x| x.parse().unwrap()).collect();
        [a[0], a[1], a[2]]
    })
    .collect();


    let result = part_one(&coords, 10);

    println!("Answer = {}", result);

    let result_two = part_two(&coords);

    println!("Answer for second one is => {result_two}");
}

fn part_two(coords: &Vec<[i32; 3]>) -> i64 {
    let n = coords.len();

    let mut edges: Vec<(i128, usize, usize)> =
        Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in i + 1..n {
            let dx = coords[i][0] as i128 - coords[j][0] as i128;
            let dy = coords[i][1] as i128 - coords[j][1] as i128;
            let dz = coords[i][2] as i128 - coords[j][2] as i128;

            let dist2 = dx*dx + dy*dy + dz*dz;
            edges.push((dist2, i, j));
        }
    }

    edges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut dsu = DSU::new(n);
    let mut last_pair = None;

    for (_, a, b) in edges {
        if dsu.union(a, b) {
            last_pair = Some((a, b));
            if dsu.components == 1 {
                break;
            }
        }
    }

    let (a, b) = last_pair.expect("No merge occurred");

    let x1 = coords[a][0] as i64;
    let x2 = coords[b][0] as i64;

    x1 * x2
}

fn part_one(coords: &Vec<[i32; 3]>, limit: usize) -> usize{
    let n = coords.len();
    let mut edges: Vec<(f32, usize, usize)> = vec![];

    for i in 0..n {
        for j in i + 1..n {
            let d = (((coords[i][0] - coords[j][0]) as f32).powf(2.0)
                + ((coords[i][1] - coords[j][1]) as f32).powf(2.0)
                + ((coords[i][2] - coords[j][2]) as f32).powf(2.0))
            .sqrt();
            edges.push((d, i, j));
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut dsu = DSU::new(n);

    for i in 0..limit {
        let (_, a, b) = edges[i];
        dsu.union(a, b);
    }

    let mut sizes = vec![];
    for i in 0..n {
        if dsu.find(i) == i {
            sizes.push(dsu.size[i]);
        }
    }

    sizes.sort();
    sizes.reverse();

    let result = sizes[0] * sizes[1] * sizes[2];

    result
}

