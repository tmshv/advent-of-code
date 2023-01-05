use petgraph::{algo::kosaraju_scc, graph::NodeIndex, prelude::UnGraph};
use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    io,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, PartialOrd, Ord, Hash, PartialEq, Eq, Copy, Clone)]
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

        Ok(Voxel::new(x, y, z))
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
    fn new(x: i32, y: i32, z: i32) -> Voxel {
        Voxel { x, y, z }
    }

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
    // graph: UnGraphMap<Voxel, ()>,
    graph: UnGraph<Voxel, ()>,
    nodes: Vec<NodeIndex>,
}

impl System {
    fn new(voxels: &Vec<Voxel>) -> System {
        let mut graph = UnGraph::<Voxel, ()>::default();
        let mut nodes = Vec::new();
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

    fn primary_island(&self) -> Vec<NodeIndex> {
        let mut islands = kosaraju_scc(&self.graph);
        islands.sort_by_key(|island| island.len());
        islands.last().unwrap().clone()
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

fn get_bounding_box(items: &Vec<Voxel>) -> (i32, i32, i32, i32, i32, i32) {
    let min_x = items.iter().min_by_key(|v| v.x).unwrap().x;
    let max_x = items.iter().max_by_key(|v| v.x).unwrap().x;
    let min_y = items.iter().min_by_key(|v| v.y).unwrap().y;
    let max_y = items.iter().max_by_key(|v| v.y).unwrap().y;
    let min_z = items.iter().min_by_key(|v| v.z).unwrap().z;
    let max_z = items.iter().max_by_key(|v| v.z).unwrap().z;
    (min_x, max_x, min_y, max_y, min_z, max_z)
}

fn extend_bounding_box(bb: (i32, i32, i32, i32, i32, i32)) -> (i32, i32, i32, i32, i32, i32) {
    let v = 1;
    let (min_x, max_x, min_y, max_y, min_z, max_z) = bb;
    (
        min_x - v,
        max_x + v,
        min_y - v,
        max_y + v,
        min_z - v,
        max_z + v,
    )
}

fn invert_voxels(items: &Vec<Voxel>) -> Vec<Voxel> {
    // extend bounding box by 1 in each side
    // so thats mean solid will be completely within air
    let (min_x, max_x, min_y, max_y, min_z, max_z) = extend_bounding_box(get_bounding_box(items));
    let mut others = vec![];
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                let v = Voxel::new(x, y, z);
                if !items.contains(&v) {
                    others.push(v);
                }
            }
        }
    }
    others
}

fn part_one(items: &Vec<Voxel>) -> u32 {
    let sys = System::new(items);
    // 6 neighbors -> all sides of the voxel are adjacent with others
    // 0 neighbors -> voxel is completely separate
    // 1 neighbors -> voxel has one heighbor
    // 2, 3, 4 neighbors -> voxel has N heighbors
    let mut sum = 0u32;
    for node in sys.nodes {
        let edges = sys.graph.neighbors(node).count() as u32;
        sum += 6 - edges;
    }
    sum
}

fn part_two(items: &Vec<Voxel>) -> u32 {
    let sys = System::new(items);
    // 6 neighbors -> all sides of the voxel are adjacent with others
    // 0 neighbors -> voxel is completely separate
    // 1 neighbors -> voxel has one heighbor
    // 2, 3, 4 neighbors -> voxel has N heighbors

    // create an anlternative graph of AIR
    // that is inverted version of SOLID graph
    // calculate additional volume as in previous part
    // take all AIR voxels that are not a BIG AIR (by finding biggest graph island)
    // subtract AIR voxels in the same logic

    // calculate +SOLID
    let mut sum = 0u32;
    for node in sys.nodes {
        let edges = sys.graph.neighbors(node).count() as u32;
        sum += 6 - edges; // 6 edges of the box
    }

    // calculate -AIR
    let air = invert_voxels(items);
    let air = System::new(&air);

    let outside_air = air.primary_island();
    for node in air.nodes {
        // skip voxels from big air
        if outside_air.contains(&node) {
            continue;
        }
        let edges = air.graph.neighbors(node).count() as u32;
        sum -= 6 - edges; // 6 edges of the box
    }
    sum
}

fn main() {
    let items = read_input();

    let result = part_one(&items);
    println!("Part one: {}", result);

    let result = part_two(&items);
    println!("Part two: {}", result);
}

#[cfg(test)]
mod tests {
    use petgraph::algo::kosaraju_scc;

    use crate::{
        get_bounding_box, invert_voxels, part_one, part_two, ParseVoxelError, System, Voxel,
    };

    fn get_volume(items: &Vec<Voxel>) -> i32 {
        let (min_x, max_x, min_y, max_y, min_z, max_z) = get_bounding_box(items);
        let s = 1;
        (max_x - min_x + s) * (max_y - min_y + s) * (max_z - min_z + s)
    }

    #[test]
    fn parse_voxel() {
        let result = "1,1,1".parse::<Voxel>();
        assert_eq!(result, Ok(Voxel::new(1, 1, 1)));

        let result = "-1,1,1".parse::<Voxel>();
        assert_eq!(result, Ok(Voxel::new(-1, 1, 1)));

        let result = "1-1,1".parse::<Voxel>();
        assert_eq!(result, Err(ParseVoxelError));

        let result = "".parse::<Voxel>();
        assert_eq!(result, Err(ParseVoxelError));
    }

    #[test]
    fn voxel_is_close() {
        let a = Voxel::new(2, 2, 2);
        let b = Voxel::new(1, 2, 2);
        assert_eq!(a.is_close(&b), true);

        let b = Voxel::new(3, 2, 2);
        assert_eq!(a.is_close(&b), true);

        let b = Voxel::new(2, 2, 4);
        assert_eq!(a.is_close(&b), false);
    }

    #[test]
    fn part_one_64() {
        let items = vec![
            Voxel::new(2, 2, 2),
            Voxel::new(1, 2, 2),
            Voxel::new(3, 2, 2),
            Voxel::new(2, 1, 2),
            Voxel::new(2, 3, 2),
            Voxel::new(2, 2, 1),
            Voxel::new(2, 2, 3),
            Voxel::new(2, 2, 4),
            Voxel::new(2, 2, 6),
            Voxel::new(1, 2, 5),
            Voxel::new(3, 2, 5),
            Voxel::new(2, 1, 5),
            Voxel::new(2, 3, 5),
        ];
        let result = part_one(&items);
        assert_eq!(result, 64);
    }

    #[test]
    fn part_two_58() {
        let items = vec![
            Voxel::new(2, 2, 2),
            Voxel::new(1, 2, 2),
            Voxel::new(3, 2, 2),
            Voxel::new(2, 1, 2),
            Voxel::new(2, 3, 2),
            Voxel::new(2, 2, 1),
            Voxel::new(2, 2, 3),
            Voxel::new(2, 2, 4),
            Voxel::new(2, 2, 6),
            Voxel::new(1, 2, 5),
            Voxel::new(3, 2, 5),
            Voxel::new(2, 1, 5),
            Voxel::new(2, 3, 5),
        ];
        let result = part_two(&items);
        assert_eq!(result, 58);
    }
    #[test]
    fn invert_voxels_simple() {
        let items = vec![Voxel::new(0, 0, 0), Voxel::new(1, 1, 1)];
        let inverted = invert_voxels(&items);
        assert_eq!(
            inverted,
            vec![
                Voxel::new(0, 0, 1),
                Voxel::new(0, 1, 0),
                Voxel::new(0, 1, 1),
                Voxel::new(1, 0, 0),
                Voxel::new(1, 0, 1),
                Voxel::new(1, 1, 0),
            ]
        );
    }

    #[test]
    fn invert_voxels_from_test() {
        let items = vec![
            Voxel::new(2, 2, 2),
            Voxel::new(1, 2, 2),
            Voxel::new(3, 2, 2),
            Voxel::new(2, 1, 2),
            Voxel::new(2, 3, 2),
            Voxel::new(2, 2, 1),
            Voxel::new(2, 2, 3),
            Voxel::new(2, 2, 4),
            Voxel::new(2, 2, 6),
            Voxel::new(1, 2, 5),
            Voxel::new(3, 2, 5),
            Voxel::new(2, 1, 5),
            Voxel::new(2, 3, 5),
        ];
        let inverted = invert_voxels(&items);
        let all = items.iter().all(|v| !inverted.contains(v));
        assert_eq!(all, true);
    }

    #[test]
    fn invert_voxels_count() {
        let items = vec![
            Voxel::new(2, 2, 2),
            Voxel::new(1, 2, 2),
            Voxel::new(3, 2, 2),
            Voxel::new(2, 1, 2),
            Voxel::new(2, 3, 2),
            Voxel::new(2, 2, 1),
            Voxel::new(2, 2, 3),
            Voxel::new(2, 2, 4),
            Voxel::new(2, 2, 6),
            Voxel::new(1, 2, 5),
            Voxel::new(3, 2, 5),
            Voxel::new(2, 1, 5),
            Voxel::new(2, 3, 5),
        ];
        let volume = get_volume(&items) as usize;
        assert_eq!(volume, 54);
        let inverted = invert_voxels(&items);
        assert_eq!(inverted.len(), volume - items.len());
    }

    #[test]
    fn air_islands() {
        let items = vec![
            Voxel::new(2, 2, 2),
            Voxel::new(1, 2, 2),
            Voxel::new(3, 2, 2),
            Voxel::new(2, 1, 2),
            Voxel::new(2, 3, 2),
            Voxel::new(2, 2, 1),
            Voxel::new(2, 2, 3),
            Voxel::new(2, 2, 4),
            Voxel::new(2, 2, 6),
            Voxel::new(1, 2, 5),
            Voxel::new(3, 2, 5),
            Voxel::new(2, 1, 5),
            Voxel::new(2, 3, 5),
        ];
        let inverted = invert_voxels(&items);
        let air = System::new(&inverted);
        let islands = kosaraju_scc(&air.graph);

        assert_eq!(islands.len(), 2);
        assert_eq!(islands[0].len(), 1);
        assert_eq!(islands[1].len(), 40);
    }

    #[test]
    fn primary_islands() {
        let items = vec![
            Voxel::new(2, 2, 2),
            Voxel::new(1, 2, 2),
            Voxel::new(3, 2, 2),
            Voxel::new(2, 1, 2),
            Voxel::new(2, 3, 2),
            Voxel::new(2, 2, 1),
            Voxel::new(2, 2, 3),
            Voxel::new(2, 2, 4),
            Voxel::new(2, 2, 6),
            Voxel::new(1, 2, 5),
            Voxel::new(3, 2, 5),
            Voxel::new(2, 1, 5),
            Voxel::new(2, 3, 5),
        ];
        let sys = System::new(&items);
        let result = sys.primary_island();
        assert_eq!(result.len(), 8);
    }
}
