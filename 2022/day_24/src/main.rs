use std::{
    collections::{BinaryHeap, HashMap, HashSet},
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
const STEPS: [&Vector; 5] = [&R, &D, &U, &L, &W];

fn gcd(a: usize, b: usize) -> usize {
    let mut min = std::cmp::min(a, b);
    let mut max = std::cmp::max(a, b);
    loop {
        let remainder = max % min;
        if remainder == 0 {
            return min;
        }
        max = min;
        min = remainder;
    }
}

fn lcm(first: usize, second: usize) -> usize {
    let x = gcd(first, second);
    first * second / x
}

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

    fn in_bounds(&self, bounds: &(isize, isize, isize, isize)) -> bool {
        let (xmin, xmax, ymin, ymax) = bounds;
        let x = &self.0;
        let y = &self.1;
        (x >= xmin && x <= xmax) && (y >= ymin && y <= ymax)
    }

    fn get_move(&self, other: &Vector) -> char {
        match other.sub(self) {
            Vector(1, 0) => 'R',
            Vector(-1, 0) => 'L',
            Vector(0, 1) => 'D',
            Vector(0, -1) => 'U',
            Vector(0, 0) => 'W',
            _ => '?',
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Squad {
    ts: usize,
    trace: Vec<Vector>,
}

impl Ord for Squad {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ts.cmp(&other.ts).reverse()
    }
}

impl PartialOrd for Squad {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Squad {
    fn apply(&self, pos: Vector) -> Squad {
        let mut trace = self.trace.clone();
        trace.push(pos);
        Squad {
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
            .map(|(a, b)| a.get_move(b))
            .into_iter()
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Blizzard {
    pos: Vector,
    dir: Vector,
}

impl Blizzard {
    fn next(&self) -> Vector {
        self.pos.add(&self.dir)
    }

    fn step(&mut self) {
        self.pos = self.next();
    }

    fn teleport(&mut self, bounds: &(isize, isize, isize, isize)) {
        let (xmin, xmax, ymin, ymax) = bounds;
        self.pos = match self.dir {
            Vector(1, 0) => Vector(*xmin, self.pos.1),
            Vector(-1, 0) => Vector(*xmax, self.pos.1),
            Vector(0, 1) => Vector(self.pos.0, *ymin),
            Vector(0, -1) => Vector(self.pos.0, *ymax),
            Vector(_, _) => panic!("Unreachable"),
        }
    }
}

#[derive(Debug)]
struct Valley {
    ts: usize,
    grid: Grid<Tile>,
    blizzards: Vec<Blizzard>,
    start: Vector,
    finish: Vector,
    stat: HashMap<usize, HashSet<Vector>>,
}

impl Valley {
    fn period(&self) -> usize {
        let (_, xmax, _, ymax) = self.get_playground_bounds();
        lcm(xmax, ymax)
    }

    fn tick(&mut self) {
        let (xmin, xmax, ymin, ymax) = self.get_playground_bounds();
        let playground = (xmin as isize, xmax as isize, ymin as isize, ymax as isize);

        self.blizzards.iter_mut().for_each(|blizzard| {
            if blizzard.next().in_bounds(&playground) {
                blizzard.step();
            } else {
                blizzard.teleport(&playground);
            };
        });

        // Increase frame
        self.ts += 1;
    }

    fn save_blizzard_positions(&mut self) {
        let blizzards: HashSet<Vector> =
            self.blizzards.iter().map(|blizzard| blizzard.pos).collect();
        self.stat.insert(self.ts, blizzards);
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

    fn is_wall(&self, pos: &Vector) -> bool {
        if &self.start == pos || &self.finish == pos {
            return false;
        }

        let (xmin, xmax, ymin, ymax) = self.get_playground_bounds();
        let (x, y) = pos.as_index();
        x < xmin || x > xmax || y < ymin || y > ymax
    }

    fn is_blizzard(&self, pos: &Vector, ts: usize) -> bool {
        let ts_index = ts % self.period();
        match self.stat.get(&ts_index) {
            Some(blizzards) => blizzards.contains(pos),
            None => {
                println!("is_blizzard {:?} {}", pos, ts);
                panic!("Unreachable");
            }
        }
    }
}

fn read_input() -> Valley {
    let mut start = Vector(0, 0);
    let mut finish = Vector(0, 0);
    let mut blizzards = Vec::new();
    let mut grid = Vec::new();

    let lines: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();
    for (y, line) in lines.iter().enumerate() {
        let first_row = y == 0;
        let last_row = y == lines.len() - 1;
        let chars: Vec<char> = line.chars().collect();
        grid.push(Vec::new());
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
        }
    }
    Valley {
        ts: 0,
        grid,
        blizzards,
        start,
        finish,
        stat: HashMap::new(),
    }
}

fn print_valley(valley: &Valley, e: Option<Vector>) {
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
            if let Some(e) = e {
                if e == cell {
                    c = 'E';
                }
            }
            print!("{}", c);
        }
        println!("");
    }
}

fn solve(valley: &Valley, start: Vector, finish: Vector, ts: usize) -> Option<Squad> {
    let squad = Squad {
        ts,
        trace: vec![start],
    };

    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();

    seen.insert((*squad.position(), squad.ts % valley.period()));
    queue.push(squad);

    while let Some(squad) = queue.pop() {
        let pos = squad.position();

        // Done
        if pos == &finish {
            return Some(squad);
        }

        let new_ts = squad.next();
        STEPS
            .iter()
            .map(|step| pos.add(step))
            .filter(|pos| !valley.is_wall(pos))
            .filter(|pos| !valley.is_blizzard(pos, new_ts))
            .for_each(|pos| {
                let new_squad = squad.apply(pos);
                if seen.insert((*new_squad.position(), new_squad.ts % valley.period())) {
                    queue.push(new_squad);
                }
            });
    }
    None
}

fn part_one(valley: &Valley) -> usize {
    let start = valley.start;
    let finish = valley.finish;
    if let Some(squad) = solve(valley, start, finish, 0) {
        return squad.ts;
    }

    0
}

fn simulate(valley: &mut Valley, squad: &Squad) {
    for (i, s) in squad.trace.iter().enumerate() {
        println!("{:?}", &squad.get_trace_value());
        print_valley(&valley, Some(*s));
        valley.tick();
        thread::sleep(Duration::from_millis(100));
        clear();
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let mut valley = read_input();
    valley.save_blizzard_positions();
    for _ in 0..valley.period() {
        valley.tick();
        valley.save_blizzard_positions();
    }

    let result = part_one(&valley);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{gcd, lcm};

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(25, 15), 5);
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(100, 35), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(25, 15), 75);
        assert_eq!(lcm(6, 4), 12);
        assert_eq!(lcm(100, 35), 700);
    }
}
