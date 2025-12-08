use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

struct UnionFind {
    pub parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        true
    }
}

struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
}

impl JunctionBox {
    pub fn distance(&self, other: &JunctionBox) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl Debug for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Eq, PartialEq)]
struct Distance {
    pub value: isize,
    pub start_node: usize,
    pub end_node: usize,
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}: {}",
            self.start_node, self.end_node, self.value
        )
    }
}

pub fn solve(input: &str) -> String {
    let junction_boxes: Vec<_> = input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line
                .split(",")
                .map(|number| number.parse().unwrap())
                .collect();
            JunctionBox {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
        .collect();

    let mut union_find = UnionFind::new(junction_boxes.len());

    let distances: Vec<_> = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            junction_boxes[(i + 1)..]
                .iter()
                .map(move |b| a.distance(b))
                .enumerate()
                .map(move |(j, distance)| Distance {
                    value: distance,
                    start_node: i,
                    end_node: i + 1 + j,
                })
        })
        .collect();

    let mut priority_queue: BinaryHeap<_> =
        distances.iter().map(|distance| Reverse(distance)).collect();

    let n = 1000;
    (0..n)
        .filter_map(|_| priority_queue.pop())
        .for_each(|distance| {
            let distance = distance.0;
            if union_find.union(distance.start_node, distance.end_node) {}
        });

    let mut frequencies = HashMap::new();
    (0..junction_boxes.len())
        .map(|index| union_find.find(index))
        .for_each(|parent| {
            *frequencies.entry(parent).or_insert(0usize) += 1;
        });
    let mut counts: Vec<_> = frequencies.values().copied().collect();
    counts.sort_by(|a, b| b.cmp(a));

    let product = counts[0] * counts[1] * counts[2];

    format!("Product is: {}", product)
}
