use std::{
    collections::{HashMap, HashSet, VecDeque},
    io, thread,
    time::Duration,
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vector(isize, isize);

type Grid<T> = Vec<Vec<T>>;

const U: Vector = Vector(0, -1);
const D: Vector = Vector(0, 1);
const L: Vector = Vector(-1, 0);
const R: Vector = Vector(1, 0);

#[derive(Debug)]
enum Tile {
    Void,
    Wall,
    Obstacle(Blizzard),
    Mess(usize),
}

impl Vector {
    fn add(&self, other: &Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1)
    }

    fn as_index(&self) -> (usize, usize) {
        let x = if self.0 > 0 { self.0 as usize } else { 0 };
        let y = if self.1 > 0 { self.1 as usize } else { 0 };
        (x, y)
    }
}

#[derive(Debug, Clone)]
struct Blizzard {
    pos: Vector,
    dir: Vector,
}

#[derive(Debug)]
struct Valley {
    ts: usize,
    grid: Grid<Tile>,
    blizzards: Vec<Blizzard>,
    start: Vector,
    finish: Vector,
    stat: Grid<HashSet<usize>>,
}

impl Valley {
    fn tick(&mut self) {
        let wall_right = self.grid[0].len() as isize - 2;
        let wall_bottom = self.grid.len() as isize - 2;
        self.blizzards.iter_mut().for_each(|b| {
            let (x, y) = b.pos.add(&b.dir).as_index();
            let next = match self.grid[y][x] {
                Tile::Wall => {
                    match b.dir {
                        Vector(1, 0) => Vector(1, b.pos.1),
                        Vector(-1, 0) => Vector(wall_right, b.pos.1), // TODO 1 -> last
                        Vector(0, 1) => Vector(b.pos.0, 1),
                        Vector(0, -1) => Vector(b.pos.0, wall_bottom), // TODO 1 -> last
                        Vector(_, _) => panic!("UB"),
                    }
                }
                _ => b.pos.add(&b.dir),
            };
            b.pos = next;
        });

        // Increase frame
        self.ts += 1;
    }

    fn write_stat(&mut self) {
        let blizzards = self.get_blizzard_map();
        let (xmin, xmax, ymin, ymax) = self.get_playground_bounds();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let cell = Vector(x as isize, y as isize);
                match blizzards.get(&cell) {
                    None => {
                        self.stat[y][x].insert(self.ts);
                    }
                    Some(_) => (),
                }
            }
        }
    }

    fn get_blizzard_map(&self) -> HashMap<Vector, Tile> {
        self.blizzards.iter().fold(HashMap::new(), |mut acc, b| {
            match acc.get(&b.pos) {
                None => acc.insert(b.pos, Tile::Obstacle(b.clone())),
                Some(t) => {
                    let new_tile = match t {
                        Tile::Mess(n) => Tile::Mess(n + 1),
                        _ => Tile::Mess(2),
                    };
                    acc.insert(b.pos, new_tile)
                }
            };
            acc
        })
    }

    fn get_playground_bounds(&self) -> (usize, usize, usize, usize) {
        let xmin = 1;
        let xmax = self.grid[0].len() - 2;
        let ymin = 1;
        let ymax = self.grid.len() - 2;
        (xmin, xmax, ymin, ymax)
    }
}

fn read_input() -> Valley {
    let mut start = Vector(0, 0);
    let mut finish = Vector(0, 0);
    let mut blizzards = Vec::new();
    let mut grid = Vec::new();
    let mut stat = Vec::new();

    let lines: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();
    for (y, line) in lines.iter().enumerate() {
        let first_row = y == 0;
        let last_row = y == lines.len() - 1;
        let chars: Vec<char> = line.chars().collect();
        grid.push(Vec::new());
        stat.push(Vec::new());
        for (x, cell) in chars.iter().enumerate() {
            let pos = Vector(x as isize, y as isize);
            let is_wall = match cell {
                '#' => true,
                _ => {
                    let first_col = x == 0;
                    let last_col = x == chars.len() - 1;
                    first_row | last_row | first_col | last_col
                }
            };
            match cell {
                '.' => {
                    if is_wall {
                        if first_row {
                            start = pos.clone();
                        } else if last_row {
                            finish = pos.clone();
                        }
                    }
                }
                _ => (),
            };
            match cell {
                '>' => blizzards.push(Blizzard { pos, dir: R }),
                '<' => blizzards.push(Blizzard { pos, dir: L }),
                'v' => blizzards.push(Blizzard { pos, dir: D }),
                '^' => blizzards.push(Blizzard { pos, dir: U }),
                _ => (),
            };
            let tile = match cell {
                '#' => Tile::Wall,
                _ => Tile::Void,
            };
            grid[y].push(tile);
            stat[y].push(HashSet::new());
        }
    }
    Valley {
        ts: 0,
        grid,
        blizzards,
        start,
        finish,
        stat,
    }
}

fn print_valley(valley: &Valley) {
    let blizzards = valley.get_blizzard_map();
    for (y, row) in valley.grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let mut c = match tile {
                Tile::Void => '.',
                Tile::Wall => '#',
                _ => ' ',
            };
            let cell = Vector(x as isize, y as isize);
            if let Some(t) = blizzards.get(&cell) {
                c = match t {
                    Tile::Obstacle(b) => match b.dir {
                        Vector(1, 0) => '>',
                        Vector(-1, 0) => '<',
                        Vector(0, 1) => 'v',
                        Vector(0, -1) => '^',
                        _ => 'x',
                    },
                    Tile::Mess(m) => match m {
                        2 => '2',
                        3 => '3',
                        4 => '4',
                        5 => '5',
                        6 => '6',
                        7 => '7',
                        8 => '8',
                        9 => '9',
                        _ => '%',
                    },
                    _ => ' ',
                }
            }
            print!("{}", c);
        }
        println!("");
    }
}

fn part_one(mut valley: Valley) -> usize {
    // loop {
    for _ in 0..12 {
        print_valley(&valley);
        valley.write_stat();
        valley.tick();
        thread::sleep(Duration::from_millis(100));
        clear();
        if false {
            return 0;
        }
    }
    print_valley(&valley);

    for (y, row) in valley.stat.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile.len() > 0 {
                let mut xs: Vec<&usize> = tile.iter().collect();
                xs.sort();
                println!("row={} col={} >>> {:?}", y, x, xs);
            }
        }
    }

    //     let mut seen = HashSet::<State>::with_capacity(10_000_000);
    //     let mut deq = VecDeque::<State>::with_capacity(10_000_000);
    //     deq.push_front(state);
    //
    //     // evaluate new states starting from current amount of geode earned
    //     let mut max_geodes = state.geode;
    //     let mut max_at_time = state.time;
    //
    //     while deq.len() > 0 {
    //         let state = deq.pop_front().unwrap();
    //
    //         // state is already checked
    //         if seen.contains(&state) {
    //             continue;
    //         } else {
    //             seen.insert(state.clone());
    //         }
    //
    //         // state is wasted
    //         if !state.has_time() {
    //             continue;
    //         }
    //
    //         let geodes = state.geode + state.geode_robots;
    //         if geodes > max_geodes {
    //             max_geodes = geodes;
    //             max_at_time = state.time;
    //         }
    //
    //         // skip state if it waste more time than best and earned less geodes
    //         if state.time > max_at_time && state.geode + state.geode_robots < max_geodes {
    //             continue;
    //         }
    //
    //         // check unique branch where we buy geode robot
    //         if state.enough_resources(self.geode_robot_cost) {
    //             let mut next_state = state.clone();
    //             next_state.tick();
    //             next_state.create_robot((0, 0, 0, 1), self.geode_robot_cost);
    //             deq.push_back(next_state);
    //
    //             // no need to check brances where other robots can be build at this step
    //             // nor earning resources
    //             continue;
    //         }
    //
    //         // check branch where we buy obsidian robot
    //         if state.enough_resources(self.obsidian_robot_cost) {
    //             let mut next_state = state.clone();
    //             next_state.tick();
    //             next_state.create_robot((0, 0, 1, 0), self.obsidian_robot_cost);
    //             deq.push_back(next_state);
    //         }
    //
    //         // check branch where we buy clay robot
    //         if state.enough_resources(self.clay_robot_cost)
    //             && !state.enough_robots(self.clay_robot_cost)
    //         {
    //             let mut next_state = state.clone();
    //             next_state.tick();
    //             next_state.create_robot((0, 1, 0, 0), self.clay_robot_cost);
    //             deq.push_back(next_state);
    //         }
    //     }
    //     max_geodes

    0
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let valley = read_input();
    let result = part_one(valley);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {}
