use std::{cmp::Reverse, collections::BinaryHeap, fmt::Debug};

struct UnionFind {
    parent: Vec<usize>,
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
        let (mut x, mut y) = (self.find(x), self.find(y));
        if x == y {
            return false;
        }

        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;

        if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
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
    pub fn distance(&self, other: &Self) -> isize {
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
    value: isize,
    start: usize,
    end: usize,
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
        write!(f, "{} -> {}: {}", self.start, self.end, self.value)
    }
}

pub fn solve(input: &str) -> String {
    let boxes: Vec<_> = input
        .lines()
        .map(|line| {
            let coords: Vec<_> = line
                .split(",")
                .map(|number| number.parse().unwrap())
                .collect();
            JunctionBox {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    let mut union_find = UnionFind::new(boxes.len());

    let distances: Vec<_> = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            boxes
                .iter()
                .skip(i + 1)
                .map(move |b| a.distance(b))
                .enumerate()
                .map(move |(j, distance)| Distance {
                    value: distance,
                    start: i,
                    end: i + 1 + j,
                })
        })
        .collect();

    let mut priority_queue: BinaryHeap<_> = distances.into_iter().map(Reverse).collect();

    let mut unioned = 1;
    let mut result = 0;

    while let Some(Reverse(dist)) = priority_queue.pop() {
        if union_find.union(dist.start, dist.end) {
            unioned += 1;
            if unioned == boxes.len() {
                result = boxes[dist.start].x * boxes[dist.end].x;
                break;
            }
        }
    }

    format!("Result is: {}", result)
}
