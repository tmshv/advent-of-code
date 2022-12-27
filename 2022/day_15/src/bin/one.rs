use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    io,
    iter::zip,
    ops::{Add, Sub},
    vec,
};

use petgraph::{
    algo::dijkstra,
    graph::{NodeIndex, UnGraph},
};
use regex::Regex;

#[derive(Debug, Clone)]
struct Landscape {
    graph: UnGraph<u32, u32>,
    ntov: HashMap<NodeIndex, Vector>,
    vton: HashMap<Vector, NodeIndex>,
}

impl Landscape {
    fn new(grid: &Vec<Vec<Vector>>) -> Landscape {
        let mut ntov = HashMap::new();
        let mut vton = HashMap::new();
        let mut graph = UnGraph::<u32, u32>::default();
        let left = Vector::new(-1, 0);
        let right = Vector::new(1, 0);
        let up = Vector::new(0, -1);
        let down = Vector::new(0, 1);
        for row in grid {
            for v in row {
                let node = graph.add_node(0);
                ntov.insert(node, *v);
                vton.insert(*v, node);
            }
        }
        for (k, v) in &ntov {
            let neighbors = vec![*v + left, *v + right, *v + up, *v + down];
            let cost = 1;
            for n in &neighbors {
                let node = vton.get(n);
                match node {
                    None => {}
                    Some(node) => {
                        graph.add_edge(*k, *node, cost);
                    }
                }
            }
        }
        Landscape { graph, ntov, vton }
    }

    fn mdist(&self, f: Vector, t: Vector) -> u32 {
        let a = *self.vton.get(&f).unwrap();
        let b = *self.vton.get(&t).unwrap();

        // let route = dijkstra(&self.graph, a.into(), b.into());
        let node_map = dijkstra(&self.graph, a.into(), b.into(), |_| 1);

        let path_length = node_map[&b];

        // println!("route: {:?}", node_map);
        path_length
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Cover,
    Far,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: i32,
    y: i32,
}

// impl Into<NodeIndex> for Vector {
//     fn into(self) -> NodeIndex {
//         NodeIndex::new(self)
//     }
// }

impl Vector {
    fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }
    fn zero() -> Vector {
        Vector { x: 0, y: 0 }
    }
    fn one() -> Vector {
        Vector { x: 1, y: 1 }
    }
    fn min() -> Vector {
        Vector {
            x: std::i32::MIN,
            y: std::i32::MIN,
        }
    }
    fn max() -> Vector {
        Vector {
            x: std::i32::MAX,
            y: std::i32::MAX,
        }
    }
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

fn parse_row(row: String) -> (Vector, Vector) {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15

    let pattern = Regex::new(
        r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)",
    )
    .unwrap();
    let cap = pattern.captures(row.as_str()).unwrap();
    let sensor = Vector {
        x: cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        y: cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
    };
    let beacon = Vector {
        x: cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        y: cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
    };
    (sensor, beacon)
}

fn read_input() -> (Vec<Vector>, Vec<Vector>) {
    let mut sensors = vec![];
    let mut beacons = vec![];
    for line in io::stdin().lines() {
        let (s, b) = match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => parse_row(value),
        };
        sensors.push(s);
        beacons.push(b);
    }
    (sensors, beacons)
}

// fn print_route(end: &Vector, land: &Landscape, route: &Vec<Vector>) {
//     let mut hash = HashMap::new();
//     let directions = route_to_directions(route);
//     for (i, dir) in directions.iter().enumerate() {
//         let loc = &route[i];
//         let c = match dir {
//             Direction::Cover => '*',
//             Direction::Left => '<',
//             Direction::Right => '>',
//             Direction::Up => '^',
//             Direction::Down => 'v',
//             Direction::Far => '@',
//         };
//         hash.insert(loc.as_tuple(), c);
//     }
//     let (w, h) = land.shape();
//     for y in 0..h {
//         for x in 0..w {
//             let cell = (x, y);
//             let d = hash.get(&cell);
//             let mut marker = match d {
//                 None => '.',
//                 Some(value) => *value,
//             };
//             if end.x == x && end.y == y {
//                 marker = 'E';
//             }
//             print!("{}", marker);
//         }
//         println!("");
//     }
// }

fn create_grid(tl: &Vector, br: &Vector) -> Vec<Vec<Vector>> {
    let mut items = vec![];
    for y in tl.y..=br.y {
        let mut row = vec![];
        for x in tl.x..=br.x {
            let v = Vector::new(x, y);
            row.push(v);
        }
        items.push(row);
    }
    items
}

fn get_bbox(items: &Vec<Vector>) -> (Vector, Vector) {
    let mut tl = Vector::max();
    let mut br = Vector::min();
    for v in items {
        if v.x < tl.x {
            tl.x = v.x;
        }
        if v.y < tl.y {
            tl.y = v.y;
        }

        if v.x > br.x {
            br.x = v.x;
        }
        if v.y > br.y {
            br.y = v.y;
        }
    }
    (tl, br)
}

fn manhattan(x: &Vector, y: &Vector) -> u32 {
    // Take the sum of the absolute values of the differences of the coordinates.
    // For example, if x=(a,b) and y=(c,d), the Manhattan distance between x and y is
    // |a-c| + |b-d|

    let (a, b) = x.as_tuple();
    let (c, d) = y.as_tuple();

    (a - c).abs() as u32 + (b - d).abs() as u32
}

fn main() {
    // let mut graph = UnGraph::<Vector, u32>::default();
    // let n1 = graph.add_node(Vector::zero());
    // let n2 = graph.add_node(Vector::one());
    // graph.add_edge(n1, n2, 123);

    // from_edges(&[(Vector::zero(), Vector::one())]);

    let (sensors, beacons) = read_input();

    let mut coords = vec![];
    for v in &sensors {
        coords.push(*v);
    }
    for v in &beacons {
        coords.push(*v);
    }
    let (tl, br) = get_bbox(&coords);
    println!("{:?}", br - tl);
    // return;
    let grid = create_grid(&tl, &br);
    // let landscape = Landscape::new(&grid);

    println!("grid items {:?}", grid.len());
    println!("{:?}, {:?}", tl, br);

    let mut coverage = HashMap::new();
    for (sensor, beacon) in zip(&sensors, &beacons) {
        // let target_dist = landscape.mdist(*sensor, *beacon);
        let target_dist = manhattan(sensor, beacon);
        for row in &grid {
            for v in row {
                // let dist = landscape.mdist(*sensor, *v);
                let dist = manhattan(sensor, v);
                let is_cover = dist <= target_dist;
                if is_cover {
                    coverage.insert(v, is_cover);
                }
            }
        }
    }

    // let target_dist = landscape.mdist(Vector::new(8, 7), Vector::new(2, 10));
    // println!("target dist is {:?}", target_dist);

    // let target_line = 10;
    let target_line = 200000;

    let mut count = 0;
    for row in &grid {
        for v in row {
            let is_cover = match coverage.get(v) {
                None => false,
                Some(_) => true,
            };
            if v.y == target_line && is_cover && !beacons.contains(v) {
                count += 1;
            }
        }
    }
    println!("Result: {}", count);

    // for row in &grid {
    //     for v in row {
    //         let is_cover = match coverage.get(v) {
    //             None => false,
    //             Some(_) => true,
    //         };
    //         if sensors.contains(v) {
    //             print!("S");
    //         } else if beacons.contains(v) {
    //             print!("B");
    //         } else if is_cover {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }
}

#[cfg(test)]
mod tests {
    use petgraph::algo::dijkstra;
    use petgraph::graph::{NodeIndex, UnGraph};

    use crate::{get_bbox, parse_row, Vector};

    #[test]
    fn petgraph_test() {
        // Create an undirected graph with `i32` nodes and edges with `()` associated data.
        let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

        // Find the shortest path from `1` to `4` using `1` as the cost for every edge.
        let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
        assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());
    }

    #[test]
    fn parse_row_from_example() {
        let result = parse_row(String::from(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        ));
        assert_eq!(result, (Vector::new(2, 18), Vector::new(-2, 15)));
    }

    // #[test]
    // fn create_grid_snapshot() {
    //     let (tl, br) = (Vector::new(0, 0), Vector::new(2, 3));
    //     let graph = create_grid(&tl, &br);

    //     assert_eq!(
    //         graph,
    //         Graph {
    //             nodes: HashMap::from([
    //                 (
    //                     Vector { x: 2, y: 0 },
    //                     Node {
    //                         id: Vector { x: 2, y: 0 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 0, y: 0 },
    //                     Node {
    //                         id: Vector { x: 0, y: 0 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 2, y: 1 },
    //                     Node {
    //                         id: Vector { x: 2, y: 1 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 2, y: 2 },
    //                     Node {
    //                         id: Vector { x: 2, y: 2 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 0, y: 3 },
    //                     Node {
    //                         id: Vector { x: 0, y: 3 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 0, y: 2 },
    //                     Node {
    //                         id: Vector { x: 0, y: 2 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 1, y: 0 },
    //                     Node {
    //                         id: Vector { x: 1, y: 0 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 1, y: 2 },
    //                     Node {
    //                         id: Vector { x: 1, y: 2 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 1, y: 3 },
    //                     Node {
    //                         id: Vector { x: 1, y: 3 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 1, y: 1 },
    //                     Node {
    //                         id: Vector { x: 1, y: 1 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 2, y: 3 },
    //                     Node {
    //                         id: Vector { x: 2, y: 3 },
    //                         payload: 0
    //                     }
    //                 ),
    //                 (
    //                     Vector { x: 0, y: 1 },
    //                     Node {
    //                         id: Vector { x: 0, y: 1 },
    //                         payload: 0
    //                     }
    //                 ),
    //             ]),
    //             edges: HashMap::from([
    //                 (
    //                     Vector { x: 1, y: 3 },
    //                     vec![
    //                         (Vector { x: 0, y: 3 }, 1),
    //                         (Vector { x: 2, y: 3 }, 1),
    //                         (Vector { x: 1, y: 2 }, 1),
    //                         (Vector { x: 1, y: 4 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 0, y: 3 },
    //                     vec![
    //                         (Vector { x: -1, y: 3 }, 1),
    //                         (Vector { x: 1, y: 3 }, 1),
    //                         (Vector { x: 0, y: 2 }, 1),
    //                         (Vector { x: 0, y: 4 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 2, y: 1 },
    //                     vec![
    //                         (Vector { x: 1, y: 1 }, 1),
    //                         (Vector { x: 3, y: 1 }, 1),
    //                         (Vector { x: 2, y: 0 }, 1),
    //                         (Vector { x: 2, y: 2 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 1, y: 1 },
    //                     vec![
    //                         (Vector { x: 0, y: 1 }, 1),
    //                         (Vector { x: 2, y: 1 }, 1),
    //                         (Vector { x: 1, y: 0 }, 1),
    //                         (Vector { x: 1, y: 2 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 1, y: 2 },
    //                     vec![
    //                         (Vector { x: 0, y: 2 }, 1),
    //                         (Vector { x: 2, y: 2 }, 1),
    //                         (Vector { x: 1, y: 1 }, 1),
    //                         (Vector { x: 1, y: 3 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 2, y: 2 },
    //                     vec![
    //                         (Vector { x: 1, y: 2 }, 1),
    //                         (Vector { x: 3, y: 2 }, 1),
    //                         (Vector { x: 2, y: 1 }, 1),
    //                         (Vector { x: 2, y: 3 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 0, y: 2 },
    //                     vec![
    //                         (Vector { x: -1, y: 2 }, 1),
    //                         (Vector { x: 1, y: 2 }, 1),
    //                         (Vector { x: 0, y: 1 }, 1),
    //                         (Vector { x: 0, y: 3 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 0, y: 1 },
    //                     vec![
    //                         (Vector { x: -1, y: 1 }, 1),
    //                         (Vector { x: 1, y: 1 }, 1),
    //                         (Vector { x: 0, y: 0 }, 1),
    //                         (Vector { x: 0, y: 2 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 1, y: 0 },
    //                     vec![
    //                         (Vector { x: 0, y: 0 }, 1),
    //                         (Vector { x: 2, y: 0 }, 1),
    //                         (Vector { x: 1, y: -1 }, 1),
    //                         (Vector { x: 1, y: 1 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 0, y: 0 },
    //                     vec![
    //                         (Vector { x: -1, y: 0 }, 1),
    //                         (Vector { x: 1, y: 0 }, 1),
    //                         (Vector { x: 0, y: -1 }, 1),
    //                         (Vector { x: 0, y: 1 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 2, y: 0 },
    //                     vec![
    //                         (Vector { x: 1, y: 0 }, 1),
    //                         (Vector { x: 3, y: 0 }, 1),
    //                         (Vector { x: 2, y: -1 }, 1),
    //                         (Vector { x: 2, y: 1 }, 1)
    //                     ]
    //                 ),
    //                 (
    //                     Vector { x: 2, y: 3 },
    //                     vec![
    //                         (Vector { x: 1, y: 3 }, 1),
    //                         (Vector { x: 3, y: 3 }, 1),
    //                         (Vector { x: 2, y: 2 }, 1),
    //                         (Vector { x: 2, y: 4 }, 1)
    //                     ]
    //                 ),
    //             ])
    //         }
    //     );
    // }

    #[test]
    fn get_bbox_from_example() {
        let items = vec![
            Vector { x: 2, y: 18 },
            Vector { x: 9, y: 16 },
            Vector { x: 13, y: 2 },
            Vector { x: 12, y: 14 },
            Vector { x: 10, y: 20 },
            Vector { x: 14, y: 17 },
            Vector { x: 8, y: 7 },
            Vector { x: 2, y: 0 },
            Vector { x: 0, y: 11 },
            Vector { x: 20, y: 14 },
            Vector { x: 17, y: 20 },
            Vector { x: 16, y: 7 },
            Vector { x: 14, y: 3 },
            Vector { x: 20, y: 1 },
        ];
        let result = get_bbox(&items);
        assert_eq!(result, (Vector { x: 0, y: 0 }, Vector { x: 20, y: 20 }));
    }
}
