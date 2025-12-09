use std::mem::discriminant;

use anyhow::Result;
use aoc_2025::{input_path, input_path_test, lines, read_to_string};

fn main() -> Result<()> {
    let input = read_to_string(input_path(8))?;
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    for line in lines(&input) {
        let coordinate: Vec<&str> = line.split(',').collect();
        junction_boxes.push(JunctionBox {
            x: coordinate[0].parse::<f32>().unwrap(),
            y: coordinate[1].parse::<f32>().unwrap(),
            z: coordinate[2].parse::<f32>().unwrap(),
        });
    }
    println!("Part 1: {}", part1(&junction_boxes)?);
    println!("Part 2: {}", part2(&junction_boxes)?);
    Ok(())
}

fn part1(junction_boxes: &Vec<JunctionBox>) -> Result<f32> {
    let n = junction_boxes.len();

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let d = junction_boxes[i].distance_to(&junction_boxes[j]);
            edges.push(Edge {
                a: i,
                b: j,
                dist: d,
            });
        }
    }

    edges.sort_by(|e1, e2| e1.dist.partial_cmp(&e2.dist).unwrap());

    let k = 1000;
    let mut circuits: Vec<Circuit> = Vec::new();

    for edge in edges.into_iter().take(k) {
        let a = edge.a;
        let b = edge.b;

        let idx_a = find_circuit_index(&circuits, a);
        let idx_b = find_circuit_index(&circuits, b);

        match (idx_a, idx_b) {
            (None, None) => {
                circuits.push(Circuit {
                    members: vec![a, b],
                });
            }
            (Some(i), None) => {
                circuits[i].members.push(b);
            }
            (None, Some(i)) => {
                circuits[i].members.push(a);
            }
            (Some(i), Some(j)) if i != j => {
                let (keep, remove) = if i < j { (i, j) } else { (j, i) };
                let mut other = circuits.remove(remove);
                circuits[keep].members.append(&mut other.members);
            }
            _ => {}
        }
    }

    let mut sizes: Vec<usize> = circuits.iter().map(|c| c.members.len()).collect();

    let mut in_any: Vec<bool> = vec![false; n];
    for c in &circuits {
        for &idx in &c.members {
            in_any[idx] = true;
        }
    }
    let singletons = in_any.iter().filter(|&&b| !b).count();
    sizes.extend(std::iter::repeat(1).take(singletons));

    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let answer = sizes[0] * sizes[1] * sizes[2];
    Ok(answer as f32)
}

fn find_circuit_index(circuits: &Vec<Circuit>, idx: usize) -> Option<usize> {
    circuits.iter().position(|c| c.members.contains(&idx))
}

fn part2(junction_boxes: &Vec<JunctionBox>) -> Result<i64> {
    let n = junction_boxes.len();

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let d = junction_boxes[i].distance_to(&junction_boxes[j]);
            edges.push(Edge { a: i, b: j, dist: d });
        }
    }

    edges.sort_by(|e1, e2| e1.dist.partial_cmp(&e2.dist).unwrap());

    let mut dsu = DSU::new(n);
    let mut answer: Option<i64> = None;

    for edge in edges {
        if dsu.union(edge.a, edge.b) {
            if dsu.components == 1 {
                let xa = junction_boxes[edge.a].x as i64;
                let xb = junction_boxes[edge.b].x as i64;
                answer = Some(xa * xb);
                break;
            }
        }
    }

    Ok(answer.expect("Graph never became fully connected"))
}


#[derive(Debug)]
struct Circuit {
    members: Vec<usize>,
}

#[derive(Debug)]
struct Edge {
    a: usize,
    b: usize,
    dist: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct JunctionBox {
    x: f32,
    y: f32,
    z: f32,
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.components -= 1;
        true
    }
}
