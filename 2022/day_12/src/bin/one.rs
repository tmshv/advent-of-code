use std::{collections::HashMap, hash::Hash, io};

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
    K: Eq + Hash,
{
    // nodes: Vec<Node<T>>,
    nodes: HashMap<K, Node<K, P>>,
    edges: HashMap<K, Vec<(K, u32)>>,
}

impl<K: Eq + Hash + Copy, P> Graph<K, P> {
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

    fn add_edge(&mut self, a: K, b: K, weight: u32) {
        if !self.edges.contains_key(&a) {
            self.edges.insert(a, vec![]);
        }
        let v = self.edges.get_mut(&a).unwrap();
        v.push((b, weight));
    }

    fn dijekstra(&self, start: &K, end: &K) -> Option<Route> {
        let mut visited = HashMap::new();
        let mut costs = HashMap::new();
        let mut parent = HashMap::new();

        // let mut visits = HashMap::new();
        // let mut result = Route::new(start);
        // let mut variants = vec![result.clone()];
        // env.visit(&start);

        let mut min = 1_000_000_000u32;

        let mut current = start;
        costs.insert(current, 0u32);

        loop {
            let mut next = current;
            let edges = self.edges.get(current).unwrap();
            for (other, cost) in edges {
                parent.insert(other, current);
                costs.insert(other, *cost);

                if *cost < min {
                    min = *cost;
                    next = other;
                }
            }

            visited.insert(current, true);

            current = next;
        }
        // while variants.len() > 0 {
        //     // let cs: Vec<u32> = variants.iter().map(|r| r.cost()).collect();
        //     // println!("current costs {:?}", cs);
        //     println!("current buf {:?}", variants.len());
        //     let route = variants.pop().unwrap();
        //     // let route = variants.remove(0);
        //     // if route.cost < result.cost {
        //     // println!("found new best with cost{:?}", variants.len());
        //     // result = Some(route.clone());
        //     result = route.clone();
        //     // println!("found new best with cost {:?}", result.costs);
        //     // }
        //     if route.is_reached(&end) {
        //         println!("Reach!");
        //         break;
        //     } else {
        //         let current_position = route.current().unwrap();
        //         let next_steps = env.next_possible_steps(current_position);
        //         for (cost, step) in next_steps {
        //             let mut new_route = route.clone();
        //             new_route.move_to_with_cost(cost, step);
        //             variants.push(new_route);
        //         }
        //         env.visit(current_position);
        //     }
        //     variants.sort_by_key(|route| 100000000 - route.cost());
        // }
        // Some(result)
        None
    }
}

#[derive(Debug, Clone)]
struct Cell {
    elevation: u32,
    visited: bool,
}

impl Cell {
    fn new(elevation: u32) -> Cell {
        Cell {
            elevation,
            visited: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Env {
    grid: Vec<Vec<Cell>>,
}

impl Env {
    fn new() -> Env {
        Env { grid: vec![] }
    }

    fn shape(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn is_unvisited(&self, loc: &Location) -> bool {
        let cell = &self.grid[loc.y][loc.x];
        !cell.visited
    }

    // end is at the same elevation or +1 step (-1 is not available)
    fn is_reachable(&self, start: &Location, end: &Location) -> bool {
        let slope = self.elevation_at_step(start, end);
        // let slope = (e.elevation as i32 - s.elevation as i32).abs();
        return slope == 0 || slope == 1;
    }

    fn elevation_at(&self, loc: &Location) -> u32 {
        let s = &self.grid[loc.y][loc.x];
        s.elevation
    }

    fn elevation_at_step(&self, start: &Location, end: &Location) -> i32 {
        let s = &self.grid[start.y][start.x];
        let e = &self.grid[end.y][end.x];
        e.elevation as i32 - s.elevation as i32
    }

    fn visit(&mut self, loc: &Location) {
        self.grid[loc.y][loc.x].visited = true
    }

    fn left(&self, loc: &Location) -> Option<Location> {
        if loc.x == 0 {
            return None;
        }
        Some(Location {
            x: loc.x - 1,
            y: loc.y,
            // direction: Direction::Left,
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
            // direction: Direction::Right,
        })
    }

    fn up(&self, loc: &Location) -> Option<Location> {
        if loc.y == 0 {
            return None;
        }
        Some(Location {
            x: loc.x,
            y: loc.y - 1,
            // direction: Direction::Up,
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
            // direction: Direction::Down,
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
                    if self.is_reachable(loc, &next) {
                        result.push(next);
                    }
                }
            }
        }
        result
    }

    fn next_possible_steps(&self, loc: &Location) -> Vec<(u32, Location)> {
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
                    if self.is_unvisited(&next) && self.is_reachable(loc, &next) {
                        let mut cost = self.elevation_at_step(loc, &next);
                        cost += 1;
                        result.push((cost as u32, next));
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
    // direction: Direction,
}

impl Location {
    fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn equal(&self, other: &Location) -> bool {
        self.x == other.x && self.y == other.y
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

#[derive(Debug, Clone)]
struct Route {
    steps: Vec<Location>,
    costs: Vec<u32>,
}

impl Route {
    fn new(start: Location) -> Route {
        Route {
            steps: vec![start],
            costs: vec![0],
        }
    }

    fn cost(&self) -> u32 {
        self.costs.iter().sum()
    }

    fn move_to_with_cost(&mut self, cost: u32, loc: Location) {
        self.steps.push(loc);
        self.costs.push(cost);
    }

    fn is_reached(&self, loc: &Location) -> bool {
        let last = self.current().unwrap();
        last.equal(loc)
    }

    fn current(&self) -> Option<&Location> {
        self.steps.last()
    }

    fn to_directions(&self) -> Vec<Direction> {
        let init = self.steps.len() - 1;
        let mut result = vec![];
        for s in 0..init {
            let e = s + 1;
            let start = self.steps[s];
            let end = self.steps[e];
            let d = start.get_direction_to(&end);
            result.push(d);
        }
        result
    }
}

fn read_input() -> (Location, Location, Env) {
    let mut start = Location { x: 0, y: 0 };
    let mut end = Location { x: 0, y: 0 };
    let mut env = Env::new();
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
                            row.push(Cell::new(A - A));
                            start = Location { x, y };
                        }
                        'E' => {
                            row.push(Cell::new(Z - A));
                            end = Location { x, y };
                        }
                        c => {
                            row.push(Cell::new(c as u32 - A));
                        }
                    }
                }
            }
        }
        env.grid.push(row);
    }
    (start, end, env)
}

fn print_route(end: &Location, env: &Env, route: &Route) {
    let mut hash = HashMap::new();
    let directions = route.to_directions();
    for (i, dir) in directions.iter().enumerate() {
        let loc = &route.steps[i];
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
    let (w, h) = env.shape();
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
    let (start, end, mut env) = read_input();

    println!("start={:?}", start);
    println!("end={:?}", end);

    // let route = dijekstra(start, end, &mut env);
    // match route {
    //     None => {
    //         println!("Unreachable!");
    //     }
    //     Some(route) => {
    //         println!(
    //             "Route steps {} with cost {}",
    //             route.steps.len() - 1,
    //             route.cost()
    //         );
    //         print_route(&end, &env, &route);
    //     }
    // }

    // println!("Visit map:");
    // let (w, h) = env.shape();
    // for y in 0..h {
    //     for x in 0..w {
    //         let elevation = env.elevation_at_step(&start, &end)
    //         let mut marker = if env.is_unvisited(&Location { x, y }) {
    //             '.'
    //         } else {
    //             '#'
    //         };
    //         if end.x == x && end.y == y {
    //             marker = 'E';
    //         }
    //         print!("{}", marker);
    //     }
    //     println!("");
    // }

    let mut graph = Graph::<Location, u32>::new();
    let (w, h) = env.shape();
    let mut edges = vec![];
    for y in 0..h {
        for x in 0..w {
            let loc = Location { x, y };
            // let elevation = env.elevation_at(&loc);
            let siblings = env.get_adjacent(&loc);

            for s in siblings {
                edges.push((loc, s));
            }

            // let node = graph.add_node(loc, elevation);
            // match node {
            //     None => {
            //         panic!("Failed to add node");
            //     }
            //     Some(node) => {
            //         // let node = graph.add_node(loc, elevation);
            //     }
            // }
        }
    }

    for y in 0..h {
        for x in 0..w {
            let loc = Location { x, y };
            let elevation = env.elevation_at(&loc);
            graph.add_node(loc, elevation);
            let siblings = env.get_adjacent(&loc);
            for loc_b in siblings {
                // let a = graph.get_node(&loc_a);
                // let b = graph.get_node(&loc_b);
                // match a.zip(b) {
                //     Some((a, b)) => {
                edges.push((loc, loc_b));
                //     }
                //     None => {
                //         panic!("Fail");
                //     }
                // }
            }
        }
    }

    for (a, b) in edges {
        let elevation = env.elevation_at(&a);
        // let node_a = graph.add_node(a, elevation);
        // let node_b = graph.add_node(b, elevation);
        graph.add_edge(a, b, elevation);
        // match node_a {
        //     None => {
        //         panic!("Failed to add node");
        //     }
        //     Some(node) => {
        //         // let node = graph.add_node(loc, elevation);
        //     }
        // }
    }

    println!("{:?}", graph);
}
