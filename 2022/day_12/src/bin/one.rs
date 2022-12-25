use std::{
    collections::HashMap,
    io,
};

const A: u32 = 'a' as u32;
const Z: u32 = 'z' as u32;

#[derive(Debug, Clone)]
struct Cell {
    elevation: u32,
    // loc: Location,
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

    fn is_reachable(&self, start: &Location, end: &Location) -> bool {
        let s = &self.grid[start.y][start.x];
        let e = &self.grid[end.y][end.x];
        // let slope = (e.elevation as i32 - s.elevation as i32).abs();
        let slope = e.elevation as i32 - s.elevation as i32;
        return slope == 0 || slope == 1;
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

    fn next_steps(&self, loc: &Location) -> Vec<Location> {
        let mut steps = vec![];
        let left = self.left(loc);
        match left {
            None => {}
            Some(next) => {
                if self.is_unvisited(&next) && self.is_reachable(loc, &next) {
                    steps.push(next);
                }
            }
        }
        let right = self.right(loc);
        match right {
            None => {}
            Some(next) => {
                if self.is_unvisited(&next) && self.is_reachable(loc, &next) {
                    steps.push(next);
                }
            }
        }

        let up = self.up(loc);
        match up {
            None => {}
            Some(next) => {
                if self.is_unvisited(&next) && self.is_reachable(loc, &next) {
                    steps.push(next);
                }
            }
        }

        let down = self.down(loc);
        match down {
            None => {}
            Some(next) => {
                if self.is_unvisited(&next) && self.is_reachable(loc, &next) {
                    steps.push(next);
                }
            }
        }
        steps
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

#[derive(Debug, Clone, Copy)]
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
}

impl Route {
    fn new(start: Location) -> Route {
        Route { steps: vec![start] }
    }

    fn current(&self) -> Option<&Location> {
        self.steps.last()
    }

    fn next(&self, step: Location) -> Route {
        let mut new_route = self.clone();
        new_route.steps.push(step);
        new_route
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

fn find_route(route: Route, end: Location, env: &mut Env) -> Vec<Route> {
    let loc = route.current().unwrap();
    if end.equal(loc) {
        println!("Found! with {} steps", route.steps.len() - 1);
        return vec![route];
    }
    let mut result = vec![];
    let next_steps = env.next_steps(&loc);
    for next in next_steps {
        let mut new_env = env.clone();
        new_env.visit(&next);
        let new_route = route.next(next);
        for r in find_route(new_route, end, &mut new_env) {
            result.push(r);
        }
    }
    result
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
    let x = read_input();
    let start = x.0;
    let end = x.1;
    let mut env = x.2;

    println!("start={:?}", start);
    println!("end={:?}", end);

    let mut routes = find_route(Route::new(start), end, &mut env);

    println!("Found {} routes", routes.len());

    routes.sort_by_key(|route| route.steps.len());

    let route = &routes[0];
    println!("Route steps {}", route.steps.len() - 1);
    print_route(&end, &env, route);

    // for (r, route) in routes.iter().enumerate() {
    //     println!("Route {} (len={})", r, route.steps.len() - 1);
    //     print_route(&end, &env, route);
    // }
}
