use petgraph::{graph::NodeIndex, prelude::UnGraph};
use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    io,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Voxel {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseVoxelError;

impl FromStr for Voxel {
    type Err = ParseVoxelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(ParseVoxelError);
        }

        let x = parts[0].parse::<i32>().map_err(|_| ParseVoxelError)?;
        let y = parts[1].parse::<i32>().map_err(|_| ParseVoxelError)?;
        let z = parts[2].parse::<i32>().map_err(|_| ParseVoxelError)?;

        Ok(Voxel { x, y, z })
    }
}

impl Add for Voxel {
    type Output = Voxel;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Voxel {
    type Output = Voxel;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Voxel {
    fn is_close(&self, other: &Voxel) -> bool {
        let mut v = *self - *other;
        v.x = v.x.abs();
        v.y = v.y.abs();
        v.z = v.z.abs();

        if v.x == 1 && v.y == 0 && v.z == 0 {
            return true;
        }
        if v.x == 0 && v.y == 1 && v.z == 0 {
            return true;
        }
        if v.x == 0 && v.y == 0 && v.z == 1 {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
struct System {
    graph: UnGraph<Voxel, ()>,
    nodes: Vec<NodeIndex>,
}

impl System {
    fn new(voxels: &Vec<Voxel>) -> System {
        let mut graph = UnGraph::<Voxel, ()>::default();
        let mut nodes = vec![];
        for v in voxels {
            let id = graph.add_node(*v);
            nodes.push(id);
        }

        let mut seen = HashSet::new();
        let mut edges = Vec::new();
        for (i1, w1) in graph.node_weights().enumerate() {
            for (i2, w2) in graph.node_weights().enumerate() {
                if w1 == w2 {
                    continue;
                }
                if seen.contains(&(i1, i2)) || seen.contains(&(i2, i1)) {
                    continue;
                }
                seen.insert((i1, i2));
                seen.insert((i2, i1));
                if w1.is_close(&w2) {
                    // println!("{:?} ({}) <-> {:?} ({})", w1, i1, w2, i2);
                    let n1 = nodes[i1].clone();
                    let n2 = nodes[i2].clone();
                    edges.push((n1, n2));
                }
            }
        }

        for (a, b) in edges {
            graph.add_edge(a, b, ());
        }

        Self { graph, nodes }
    }
}

fn read_input() -> Vec<Voxel> {
    io::stdin()
        .lines()
        .map(|line| match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => value.as_str().parse::<Voxel>().unwrap(),
        })
        .collect()
}

fn part_one(sys: &System) -> u32 {
    // 6 neighbors -> all sides of the voxel are adjacent with others
    // 0 neighbors -> voxel is completely separate
    // 1 neighbors -> voxel has one heighbor
    // 2, 3, 4 neighbors -> voxel has N heighbors
    let mut sum = 0u32;
    for node in &sys.nodes {
        let nu = sys.graph.neighbors(*node).count() as u32;
        sum += nu;
    }
    sum
}

fn main() {
    let items = read_input();
    let sys = System::new(&items);

    let result = part_one(&sys);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{ParseVoxelError, Voxel, System, part_one};

    #[test]
    fn parse_voxel() {
        let result = "1,1,1".parse::<Voxel>();
        assert_eq!(result, Ok(Voxel { x: 1, y: 1, z: 1 }));

        let result = "-1,1,1".parse::<Voxel>();
        assert_eq!(result, Ok(Voxel { x: -1, y: 1, z: 1 }));

        let result = "1-1,1".parse::<Voxel>();
        assert_eq!(result, Err(ParseVoxelError));

        let result = "".parse::<Voxel>();
        assert_eq!(result, Err(ParseVoxelError));
    }

    #[test]
    fn voxel_is_close() {
        let a = Voxel { x: 2, y: 2, z: 2 };
        let b = Voxel { x: 1, y: 2, z: 2 };
        assert_eq!(a.is_close(&b), true);

        let b = Voxel { x: 3, y: 2, z: 2 };
        assert_eq!(a.is_close(&b), true);

        let b = Voxel { x: 2, y: 2, z: 4 };
        assert_eq!(a.is_close(&b), false);
    }

    #[test]
    fn part_one_64() {
        let items = vec![
            Voxel { x: 2, y: 2, z: 2 },
            Voxel { x: 1, y: 2, z: 2 },
            Voxel { x: 3, y: 2, z: 2 },
            Voxel { x: 2, y: 1, z: 2 },
            Voxel { x: 2, y: 3, z: 2 },
            Voxel { x: 2, y: 2, z: 1 },
            Voxel { x: 2, y: 2, z: 3 },
            Voxel { x: 2, y: 2, z: 4 },
            Voxel { x: 2, y: 2, z: 6 },
            Voxel { x: 1, y: 2, z: 5 },
            Voxel { x: 3, y: 2, z: 5 },
            Voxel { x: 2, y: 1, z: 5 },
            Voxel { x: 2, y: 3, z: 5 },
        ];
        let sys = System::new(&items);
        let result = part_one(&sys);
        assert_eq!(result, 64);
    }
}
