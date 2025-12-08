use std::fs::read_to_string;

#[derive(Clone)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb { return; }
        if self.size[pa] < self.size[pb] {
            self.parent[pa] = pb;
            self.size[pb] += self.size[pa];
        } else {
            self.parent[pb] = pa;
            self.size[pa] += self.size[pb];
        }
    }
}

fn main() {
    let coords: Vec<[f32; 3]> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let a: Vec<f32> = l.split(',').map(|x| x.parse().unwrap()).collect();
            [a[0], a[1], a[2]]
        })
        .collect();

    let n = coords.len();
    let mut edges: Vec<(f32, usize, usize)> = vec![];

    for i in 0..n {
        for j in i + 1..n {
            let d = ((coords[i][0] - coords[j][0]).powf(2.0)
                + (coords[i][1] - coords[j][1]).powf(2.0)
                + (coords[i][2] - coords[j][2]).powf(2.0))
            .sqrt();
            edges.push((d, i, j));
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut dsu = DSU::new(n);

    let limit = 1000;

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
    println!("Answer = {}", result);
}

