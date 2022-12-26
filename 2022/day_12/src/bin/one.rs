use std::{collections::HashMap, fmt::Debug, hash::Hash, io};

const A: u32 = 'a' as u32;
const Z: u32 = 'z' as u32;

#[derive(Debug, Clone)]
struct Node<K, T> {
    id: K,
    payload: T,
}

impl<K, T> Node<K, T> {
    fn new(id: K, payload: T) -> Node<K, T> {
        Node { id, payload }
    }
}

#[derive(Debug)]
struct Graph<K, P>
where
    K: Eq + Hash + Copy + Debug,
{
    nodes: HashMap<K, Node<K, P>>,
    edges: HashMap<K, Vec<(K, i32)>>,
}

impl<K: Eq + Hash + Copy + Debug, P> Graph<K, P> {
    fn new() -> Graph<K, P> {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: K, payload: P) -> Option<&Node<K, P>> {
        if self.nodes.contains_key(&id) {
            return self.nodes.get(&id);
        }
        self.nodes.insert(id, Node::new(id, payload));
        self.nodes.get(&id)
    }

    fn get_node(&self, id: &K) -> Option<&Node<K, P>> {
        self.nodes.get(id)
    }

    fn add_edge(&mut self, a: K, b: K, weight: i32) {
        if !self.edges.contains_key(&a) {
            self.edges.insert(a, vec![]);
        }
        let v = self.edges.get_mut(&a).unwrap();
        v.push((b, weight));
    }

    fn dijekstra(&self, start: &K, end: &K) -> Option<Vec<K>> {
        let mut known = HashMap::new();
        let mut costs = HashMap::new();
        let mut paths = HashMap::new();

        // 1. mark all nodes as unvisited
        for (id, _) in &self.nodes {
            known.insert(id, false);
        }

        costs.insert(start, 0);

        loop {
            // 1. find chapest unknown node
            let mut unknown_nodes = vec![];
            for (id, is_known) in &known {
                if *is_known {
                    continue;
                }
                if costs.contains_key(id) {
                    let cost = costs.get(id).unwrap();
                    unknown_nodes.push((*id, *cost));
                }
            }
            if unknown_nodes.len() == 0 {
                break;
            }
            unknown_nodes.sort_by_key(|x| x.1);
            let (current, current_cost) = unknown_nodes[0];

            // 2. mark current node as known
            known.insert(current, true);

            // 3. inspect all unknown nodes of current node
            let edges = self.edges.get(current);
            match edges {
                None => {}
                Some(edges) => {
                    for (other, edge_cost) in edges {
                        // skip edge is node is known
                        if *known.get(other).unwrap() {
                            continue;
                        }

                        let cost = current_cost + *edge_cost;

                        if costs.contains_key(other) {
                            let old_cost = *costs.get(other).unwrap();
                            if old_cost > cost {
                                costs.insert(other, cost);
                                paths.insert(other, current);
                            }
                        } else {
                            costs.insert(other, cost);
                            paths.insert(other, current);
                        }
                    }
                }
            }
        }

        let mut cursor = end;
        let mut route = vec![*cursor];
        let mut total_cost = 0;
        loop {
            let parent = paths.get(cursor);
            match parent {
                None => {
                    break;
                }
                Some(parent) => {
                    let cost = costs.get(cursor).unwrap();
                    total_cost += cost;

                    route.push(**parent);
                    cursor = parent;
                }
            }
        }
        if cursor != start {
            return None;
        }
        route.reverse();
        println!("Toal cost = {}", total_cost);
        Some(route)
    }
}

#[derive(Debug, Clone)]
struct Landscape {
    grid: Vec<Vec<u32>>,
}

impl Landscape {
    fn new() -> Landscape {
        Landscape { grid: vec![] }
    }

    fn shape(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    // end is at the same elevation or +1 step or lower
    fn is_reachable(&self, start: &Location, end: &Location) -> bool {
        let slope = self.elevation_at_step(start, end);
        // elevation of destination cell can be much lower than current cell
        if slope < 0 {
            return true;
        }
        // or at most one higher than current
        return slope == 0 || slope == 1;
    }

    fn elevation_at(&self, loc: &Location) -> u32 {
        let s = &self.grid[loc.y][loc.x];
        *s
    }

    fn elevation_at_step(&self, start: &Location, end: &Location) -> i32 {
        let a = self.elevation_at(start) as i32;
        let b = self.elevation_at(end) as i32;
        b - a
    }

    fn left(&self, loc: &Location) -> Option<Location> {
        if loc.x == 0 {
            return None;
        }
        Some(Location {
            x: loc.x - 1,
            y: loc.y,
        })
    }

    fn right(&self, loc: &Location) -> Option<Location> {
        let border = self.grid[0].len() - 1;
        if loc.x == border {
            return None;
        }
        Some(Location {
            x: loc.x + 1,
            y: loc.y,
        })
    }

    fn up(&self, loc: &Location) -> Option<Location> {
        if loc.y == 0 {
            return None;
        }
        Some(Location {
            x: loc.x,
            y: loc.y - 1,
        })
    }

    fn down(&self, loc: &Location) -> Option<Location> {
        let border = self.grid.len() - 1;
        if loc.y == border {
            return None;
        }
        Some(Location {
            x: loc.x,
            y: loc.y + 1,
        })
    }

    fn get_adjacent(&self, loc: &Location) -> Vec<Location> {
        let steps = vec![
            self.left(loc),
            self.right(loc),
            self.up(loc),
            self.down(loc),
        ];
        let mut result = vec![];
        for step in steps {
            match step {
                None => {}
                Some(next) => {
                    // result.push(next);
                    if self.is_reachable(loc, &next) {
                        result.push(next);
                    }
                }
            }
        }
        result
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_direction_to(&self, other: &Location) -> Direction {
        if self.x == other.x && self.y == other.y {
            return Direction::Cover;
        }

        if self.x == other.x {
            return if self.y > other.y {
                Direction::Up
            } else {
                Direction::Down
            };
        }

        if self.y == other.y {
            return if self.x > other.x {
                Direction::Left
            } else {
                Direction::Right
            };
        }

        Direction::Far
    }
}

fn route_to_directions(route: &Vec<Location>) -> Vec<Direction> {
    let init = route.len() - 1;
    let mut result = vec![];
    for s in 0..init {
        let e = s + 1;
        let start = route[s];
        let end = route[e];
        let d = start.get_direction_to(&end);
        result.push(d);
    }
    result
}

fn read_input() -> (Location, Location, Landscape) {
    let mut start = Location { x: 0, y: 0 };
    let mut end = Location { x: 0, y: 0 };
    let mut env = Landscape::new();
    for (y, line) in io::stdin().lines().enumerate() {
        let mut row = vec![];
        match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => {
                for (x, char) in value.chars().enumerate() {
                    match char {
                        'S' => {
                            row.push(A - A);
                            start = Location { x, y };
                        }
                        'E' => {
                            row.push(Z - A);
                            end = Location { x, y };
                        }
                        c => {
                            row.push(c as u32 - A);
                        }
                    }
                }
            }
        }
        env.grid.push(row);
    }
    (start, end, env)
}

fn print_route(end: &Location, land: &Landscape, route: &Vec<Location>) {
    let mut hash = HashMap::new();
    let directions = route_to_directions(route);
    for (i, dir) in directions.iter().enumerate() {
        let loc = &route[i];
        let c = match dir {
            Direction::Cover => '*',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Far => '@',
        };
        hash.insert(loc.as_tuple(), c);
    }
    let (w, h) = land.shape();
    for y in 0..h {
        for x in 0..w {
            let cell = (x, y);
            let d = hash.get(&cell);
            let mut marker = match d {
                None => '.',
                Some(value) => *value,
            };
            if end.x == x && end.y == y {
                marker = 'E';
            }
            print!("{}", marker);
        }
        println!("");
    }
}

fn main() {
    let (start, end, landscape) = read_input();

    println!("start={:?}", start);
    println!("end={:?}", end);

    // Build graph
    let mut graph = Graph::<Location, u32>::new();
    let (w, h) = landscape.shape();
    let mut edges = vec![];
    for y in 0..h {
        for x in 0..w {
            let loc = Location { x, y };
            let elevation = landscape.elevation_at(&loc);
            graph.add_node(loc, elevation);
            let neighbors = landscape.get_adjacent(&loc);
            for loc_b in neighbors {
                edges.push((loc, loc_b));
            }
        }
    }
    for (a, b) in edges {
        let cost = landscape.elevation_at_step(&a, &b);
        graph.add_edge(a, b, cost);
    }

    let route = graph.dijekstra(&start, &end);
    match route {
        None => {
            println!("Unreachable!");
        }
        Some(route) => {
            println!("Route steps {}", route.len() - 1);
            print_route(&end, &landscape, &route);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn graph_path_finding_works() {
        let mut graph = Graph::<&str, u32>::new();
        graph.add_node("A", 0);
        graph.add_node("B", 0);
        graph.add_node("C", 0);
        graph.add_node("D", 0);
        graph.add_node("E", 0);
        graph.add_node("F", 0);
        graph.add_edge("A", "B", 2);
        graph.add_edge("A", "D", 8);
        graph.add_edge("B", "D", 5);
        graph.add_edge("B", "E", 6);
        graph.add_edge("D", "E", 3);
        graph.add_edge("D", "F", 2);
        graph.add_edge("E", "F", 1);
        graph.add_edge("E", "C", 9);
        graph.add_edge("F", "C", 3);
        let route = graph.dijekstra(&"A", &"C");
        assert_eq!(route, Some(vec!["A", "B", "D", "F", "C"]));
    }

    #[test]
    fn graph_path_finding_works_when_end_is_not_the_last_node() {
        let mut graph = Graph::<&str, u32>::new();
        graph.add_node("A", 0);
        graph.add_node("B", 0);
        graph.add_node("C", 0);
        graph.add_node("D", 0);
        graph.add_node("E", 0);
        graph.add_node("F", 0);
        graph.add_edge("A", "B", 2);
        graph.add_edge("A", "D", 8);
        graph.add_edge("B", "D", 5);
        graph.add_edge("B", "E", 6);
        graph.add_edge("D", "E", 3);
        graph.add_edge("D", "F", 2);
        graph.add_edge("E", "F", 1);
        graph.add_edge("E", "C", 9);
        graph.add_edge("F", "C", 3);
        let route = graph.dijekstra(&"A", &"E");
        assert_eq!(route, Some(vec!["A", "B", "E"]));
    }

    #[test]
    fn graph_path_finding_works_when_end_is_not_reachable() {
        let mut graph = Graph::<&str, u32>::new();
        graph.add_node("A", 0);
        graph.add_node("B", 0);
        graph.add_node("C", 0);
        graph.add_node("D", 0);
        graph.add_node("E", 0);
        graph.add_node("F", 0);
        graph.add_edge("A", "B", 2);
        graph.add_edge("A", "D", 8);
        graph.add_edge("B", "D", 5);
        graph.add_edge("B", "E", 6);
        graph.add_edge("D", "E", 3);
        graph.add_edge("D", "F", 2);
        graph.add_edge("E", "F", 1);
        let route = graph.dijekstra(&"A", &"C");
        assert_eq!(route, None);
    }
}
