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
const W: Vector = Vector(0, 0);
const STEPS: [&Vector; 5] = [&D, &R, &U, &L, &W];

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

    fn sub(&self, other: &Vector) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1)
    }

    fn as_index(&self) -> (usize, usize) {
        let x = if self.0 > 0 { self.0 as usize } else { 0 };
        let y = if self.1 > 0 { self.1 as usize } else { 0 };
        (x, y)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

#[derive(Debug, Clone)]
struct State {
    ts: usize,
    trace: Vec<Vector>,
}

impl State {
    fn apply(&self, pos: Vector) -> State {
        let mut trace = self.trace.clone();
        trace.push(pos);
        State {
            ts: self.next(),
            trace,
        }
    }

    fn position(&self) -> &Vector {
        self.trace.last().unwrap()
    }

    fn next(&self) -> usize {
        self.ts + 1
    }

    fn get_trace_value(&self) -> String {
        let s = self.trace.len();
        let head = self.trace.iter().take(s - 1);
        let tail = self.trace.iter().skip(1);
        head.zip(tail)
            .map(|(a, b)| match b.sub(a) {
                Vector(1, 0) => 'R',
                Vector(-1, 0) => 'L',
                Vector(0, 1) => 'D',
                Vector(0, -1) => 'U',
                Vector(0, 0) => 'W',
                _ => '?',
            })
            .into_iter()
            .collect()
    }

    fn evaluate(&self) -> bool {
        let s = self.trace.len();
        if s < 20 {
            return true;
        }

        let t = self.get_trace_value();
        let p = t.as_str();
        if &p[s - 4..s - 1] == &p[s - 8..s - 5] {
            return false;
        }

        // if &p[s - 6..s - 1] == &p[s - 12..s - 7] {
        //     return false;
        // }

        // last two steps are in loop
        let t1 = self.trace[s - 1];
        let t2 = self.trace[s - 2];
        let t3 = self.trace[s - 3];
        let t4 = self.trace[s - 4];
        if t1 == t3 && t2 == t4 {
            return false;
        }

        true
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
    period: usize,
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

    fn on_playground(&self, pos: &Vector) -> bool {
        if pos.is_zero() {
            return false;
        }
        let (x, y) = pos.as_index();
        let (xmin, xmax, ymin, ymax) = self.get_playground_bounds();
        return (x >= xmin && x <= xmax) && (y >= ymin && y <= ymax);
    }

    fn get_steps_at(&self, ts: usize, pos: &Vector) -> Vec<Vector> {
        let ts_module = ts % self.period;
        let mut result = Vec::new();
        for step in STEPS {
            let next = pos.add(step);
            if next.is_zero() {
                continue;
            }
            if next == self.finish {
                result.push(next);
                continue;
            }
            if self.on_playground(&next) {
                let (x, y) = next.as_index();
                if self.stat[y][x].contains(&ts_module) {
                    result.push(next);
                }
            }
        }
        result
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
        period: 0,
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
    valley.period = 700;
    // loop {
    for _ in 0..valley.period {
        // print_valley(&valley);
        valley.write_stat();
        valley.tick();
        // thread::sleep(Duration::from_millis(10));
        // clear();
        // if false {
        //     return 0;
        // }
    }
    print_valley(&valley);

    // for (y, row) in valley.stat.iter().enumerate() {
    //     for (x, tile) in row.iter().enumerate() {
    //         if tile.len() > 0 {
    //             let mut xs: Vec<&usize> = tile.iter().collect();
    //             xs.sort();
    //             println!("row={} col={} >>> {:?}", y, x, xs);
    //         }
    //     }
    // }

    let state = State {
        ts: 0,
        trace: vec![valley.start],
    };

    let mut min_trace = 30;
    let mut max_size = 0;
    let mut deq = VecDeque::<State>::with_capacity(1_000_000);
    deq.push_front(state);

    while deq.len() > 0 {
        if deq.len() > max_size {
            max_size = deq.len();
        }
        println!("size {} {}", deq.len(), max_size);

        let state = deq.pop_front().unwrap();
        let variants = valley.get_steps_at(state.next(), state.position());
        for v in variants {
            if v == valley.finish {
                let new_state = state.apply(v);
                println!(
                    "Finish! {:?} (ts={})",
                    new_state.get_trace_value(),
                    new_state.ts
                );
                min_trace = new_state.ts;
            }

            let new_state = state.apply(v);

            if !new_state.evaluate() {
                continue;
            }

            if new_state.trace.len() <= min_trace {
                deq.push_back(new_state);
            }
        }
    }
    min_trace
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
