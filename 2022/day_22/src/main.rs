use std::io;

// X: 1 -> 50; 51 -> 100; 101 -> 150
// Y: 1 -> 50; 51 -> 100; 101 -> 150; 151 -> 200
// unique markers:
// |||
// X
// ><
// O
// []
// |
// --
#[allow(dead_code)]
const CUBE_TEST: [(Edge, Edge); 7] = [
    // 4 -> 6
    (
        Edge {
            size: 4,
            a: (12, 5),
            n: (0, 1), // down
        },
        Edge {
            size: 4,
            a: (13, 9),
            n: (1, 0), // right
        },
    ),
    // 5 -> 2
    (
        Edge {
            size: 4,
            a: (12, 12),
            n: (1, 0), // right
        },
        Edge {
            size: 4,
            a: (4, 8),
            n: (-1, 0), // left
        },
        // Edge {
        //     size: 4,
        //     a: (1, 8),
        //     n: (1, 0), // right
        // },
    ),
    // 2 -> 11
    (
        Edge {
            size: 4,
            a: (101, 1),
            // b: (101, 50),
            n: (-1, 0), // left
        },
        Edge {
            size: 4,
            a: (51, 50),
            // b: (100, 50),
            n: (0, -1), // up
        },
    ),
    // 8 -> 20
    (
        Edge {
            size: 4,
            a: (150, 51),
            // b: (150, 100),
            n: (1, 0), // right
        },
        Edge {
            size: 4,
            a: (100, 151),
            // b: (100, 200),
            n: (1, 0), // right
        },
    ),
    // 5 -> 16
    (
        Edge {
            size: 4,
            a: (101, 100),
            // b: (150, 100),
            n: (0, 1), // down
        },
        Edge {
            size: 4,
            a: (100, 101),
            // b: (100, 150),
            n: (1, 0), // right
        },
    ),
    // 10 -> 22
    (
        Edge {
            size: 4,
            a: (51, 51),
            // b: (51, 100),
            n: (-1, 0), // left
        },
        Edge {
            size: 4,
            a: (1, 151),
            // b: (1, 200),
            n: (-1, 0), // left
        },
    ),
    // 14 -> 23
    (
        Edge {
            size: 4,
            a: (51, 101),
            // b: (51, 150),
            n: (-1, 0), // left
        },
        Edge {
            size: 4,
            a: (1, 150),
            // b: (50, 150),
            n: (0, -1), // up
        },
    ),
];

#[allow(dead_code)]
const CUBE: [(Edge, Edge); 7] = [
    // 3 -> 21
    (
        Edge {
            size: 50,
            a: (101, 1),
            // b: (150, 1),
            n: (0, -1), // up
        },
        Edge {
            size: 50,
            a: (1, 200),
            // b: (50, 200),
            n: (0, 1), // down
        },
    ),
    // 4 -> 17
    (
        Edge {
            size: 50,
            a: (150, 1),
            // b: (150, 50),
            n: (1, 0), // right
        },
        Edge {
            size: 50,
            a: (51, 200),
            // b: (100, 200),
            n: (0, 1), // down
        },
    ),
    // 2 -> 11
    (
        Edge {
            size: 50,
            a: (101, 1),
            // b: (101, 50),
            n: (-1, 0), // left
        },
        Edge {
            size: 50,
            a: (51, 50),
            // b: (100, 50),
            n: (0, -1), // up
        },
    ),
    // 8 -> 20
    (
        Edge {
            size: 50,
            a: (150, 51),
            // b: (150, 100),
            n: (1, 0), // right
        },
        Edge {
            size: 50,
            a: (100, 151),
            // b: (100, 200),
            n: (1, 0), // right
        },
    ),
    // 5 -> 16
    (
        Edge {
            size: 50,
            a: (101, 100),
            // b: (150, 100),
            n: (0, 1), // down
        },
        Edge {
            size: 50,
            a: (100, 101),
            // b: (100, 150),
            n: (1, 0), // right
        },
    ),
    // 10 -> 22
    (
        Edge {
            size: 50,
            a: (51, 51),
            // b: (51, 100),
            n: (-1, 0), // left
        },
        Edge {
            size: 50,
            a: (1, 151),
            // b: (1, 200),
            n: (-1, 0), // left
        },
    ),
    // 14 -> 23
    (
        Edge {
            size: 50,
            a: (51, 101),
            // b: (51, 150),
            n: (-1, 0), // left
        },
        Edge {
            size: 50,
            a: (1, 150),
            // b: (50, 150),
            n: (0, -1), // up
        },
    ),
];

type Position = (usize, usize);
type Shift = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    Solid,
    Void,
}

#[derive(Debug)]
enum Move {
    Straight(usize),
    Left,
    Right,
}

#[derive(Debug)]
struct Board {
    grid: [[Tile; 151]; 201], // 150 + 1 columns; 200 + 1 rows; +1 cause 0 is Void
                              // to prevent negative indicies
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[Tile::Void; 151]; 201],
        }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get_start(&self) -> Position {
        for y in 0..200 {
            for x in 0..150 {
                let tile = self.grid[y][x];
                match tile {
                    Tile::Open => {
                        return (x, y);
                    }
                    _ => continue,
                }
            }
        }
        (0, 0)
    }

    fn tile_at(&self, position: Position) -> Tile {
        let (x, y) = position;
        if y == 0 || x == 0 {
            return Tile::Void;
        }
        if y > self.grid.len() - 1 || x > self.width() - 1 {
            return Tile::Void;
        }
        self.grid[y][x]
    }
}

trait Solver {
    fn run(&self, position: Position, shift: Shift) -> (Position, Tile);
    fn teleport(&self, position: Position, shift: Shift) -> (Position, Shift);
}

struct Flat<'a> {
    board: &'a Board,
}

impl<'a> Solver for Flat<'a> {
    fn run(&self, position: Position, shift: Shift) -> (Position, Tile) {
        let next_position = add(position, shift);
        let tile = self.board.tile_at(next_position);
        (next_position, tile)
    }

    fn teleport(&self, position: Position, shift: Shift) -> (Position, Shift) {
        let (sx, sy) = shift;

        // teleport to left
        if sx > 0 && sy == 0 {
            let y = position.1;
            for x in 0..position.0 {
                match self.board.tile_at((x, y)) {
                    Tile::Open => {
                        return ((x, y), shift);
                    }
                    Tile::Solid => {
                        return (position, shift);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        // teleport to right
        if sx < 0 && sy == 0 {
            let y = position.1;
            for x in (position.0..self.board.width()).rev() {
                match self.board.tile_at((x, y)) {
                    Tile::Open => {
                        return ((x, y), shift);
                    }
                    Tile::Solid => {
                        return (position, shift);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        // teleport to top
        if sx == 0 && sy > 0 {
            let x = position.0;
            for y in 0..position.1 {
                match self.board.tile_at((x, y)) {
                    Tile::Open => {
                        return ((x, y), shift);
                    }
                    Tile::Solid => {
                        return (position, shift);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        // teleport to bottom
        if sx == 0 && sy < 0 {
            let x = position.0;
            for y in (0..self.board.height()).rev() {
                match self.board.tile_at((x, y)) {
                    Tile::Open => {
                        return ((x, y), shift);
                    }
                    Tile::Solid => {
                        return (position, shift);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        (position, shift)
    }
}

struct Cube<'a> {
    board: &'a Board,
    cube: &'a [(Edge, Edge); 7],
}

impl<'a> Solver for Cube<'a> {
    fn run(&self, position: Position, shift: Shift) -> (Position, Tile) {
        let next_position = add(position, shift);
        let tile = self.board.tile_at(next_position);
        (next_position, tile)
    }

    fn teleport(&self, position: Position, shift: Shift) -> (Position, Shift) {
        let mut edge_from = self.cube[0].0;
        let mut edge_to = self.cube[0].1;
        for (a, b) in self.cube {
            if a.contains(position) {
                edge_from = *a;
                edge_to = b.revert();
                break;
            }
            if b.contains(position) {
                edge_from = *b;
                edge_to = a.revert();
                break;
            }
        }

        let relative = edge_from.get_relative(position);
        let pos = edge_to.from_relative(relative);
        let nor = edge_to.get_normal();

        // println!(
        //     "E {:?} -> {:?} = {:?} -> {:?} N:{:?}",
        //     edge_from.a, edge_to.a, position, pos, nor
        // );

        (pos, nor)

        // if position is on edge A
        // then teleport it to edge B
        // if edge_a.contains(position) {
        //     let relative = edge_a.get_relative(position);
        //     return (edge_b.from_relative(relative), edge_b.get_normal());
        // }

        // if position is on edge B
        // then teleport it to edge A
        // if edge_b.contains(position) {
        //     let relative = edge_b.get_relative(position);
        //     return (edge_a.from_relative(relative), edge_a.get_normal());
        // }

        // (self.teleport_from(position, shift), shift)
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    size: usize,
    // Edge is defined by coordinate A and shift N
    a: (usize, usize),
    n: (isize, isize),
}

impl Edge {
    fn end(&self) -> (usize, usize) {
        let size = (self.size - 1) as isize;
        (
            (self.a.0 as isize + self.n.0 * size) as usize,
            (self.a.1 as isize + self.n.1 * size) as usize,
        )
    }

    fn revert(&self) -> Edge {
        let a = self.end();
        let n = (self.n.0 * -1, self.n.1 * -1);
        Edge {
            size: self.size,
            a,
            n,
        }
    }

    fn is_vertical(&self) -> bool {
        // Y of A == Y of B
        // self.a.1 == self.b.1
        self.n.1 != 0
    }

    // normal of the edge is defined by counterclockwise 90 degrees rotation
    fn get_normal(&self) -> (isize, isize) {
        // (-self.n.1, self.n.0)
        (self.n.1, -self.n.0)
    }

    fn contains(&self, position: (usize, usize)) -> bool {
        let (px, py) = position;
        let (ax, ay) = self.a;
        // let (bx, by) = self.b;

        if self.is_vertical() {
            // X coordinate of edge and position should be equal
            // todo: check for range within
            return ax == px;
        } else {
            return ay == py;
        }
    }

    fn get_relative(&self, position: (usize, usize)) -> isize {
        let (px, py) = position;
        let (ax, ay) = self.a;
        if self.is_vertical() {
            py as isize - ay as isize
        } else {
            px as isize - ax as isize
        }
    }

    fn from_relative(&self, relative: isize) -> (usize, usize) {
        let (ax, ay) = self.a;
        let (nx, ny) = self.n;
        if self.is_vertical() {
            let x = ax;
            let y = ay as isize + relative * ny;
            (x, y as usize)
        } else {
            let x = ax as isize + relative * nx;
            let y = ay;
            (x as usize, y)
        }
    }
}

fn parse_path(path: String) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut buff = Vec::new();

    for c in path.chars() {
        match c {
            'L' => {
                let n = buff.iter().collect::<String>().parse::<usize>().unwrap();
                moves.push(Move::Straight(n));
                buff = Vec::new();
                moves.push(Move::Left);
            }
            'R' => {
                let n = buff.iter().collect::<String>().parse::<usize>().unwrap();
                moves.push(Move::Straight(n));
                buff = Vec::new();
                moves.push(Move::Right);
            }
            x => {
                buff.push(x);
            }
        }
    }

    let n = buff.iter().collect::<String>().parse::<usize>().unwrap();
    moves.push(Move::Straight(n));

    moves
}

fn read_input() -> (Board, Vec<Move>) {
    let mut board = Board::new();
    for (y, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        for (x, t) in line.chars().enumerate() {
            let tile = match t {
                '.' => Tile::Open,
                '#' => Tile::Solid,
                ' ' => Tile::Void,
                _ => panic!("unknown tile"),
            };
            board.grid[y + 1][x + 1] = tile; // +1 cause 0 is Void
        }
    }

    let path = io::stdin().lines().next().unwrap().unwrap();
    let moves = parse_path(path);

    (board, moves)
}

fn add(position: (usize, usize), shift: (isize, isize)) -> (usize, usize) {
    let (x, y) = position;
    let (sx, sy) = shift;
    ((x as isize + sx) as usize, (y as isize + sy) as usize)
}

fn print_path(
    board: &Board,
    path: &Vec<(Position, Shift)>,
    max_x: usize,
    max_y: usize,
) {
    for y in 1..max_y {
        for x in 1..max_x {
            let pos = (x, y);
            let trace = path.iter().rev().position(|(p, s)| *p == pos);
            let c = match trace {
                None => {
                    let tile = board.tile_at(pos);
                    match tile {
                        Tile::Void => ' ',
                        Tile::Open => '.',
                        Tile::Solid => '#',
                    }
                }
                Some(trace) => {
                    let shift = path[trace].1;
                    match shift {
                        // (-1, 0) => '<',
                        // (1, 0) => '>',
                        // (0, -1) => '^',
                        // (0, 1) => 'v',
                        _ => 'o',
                    }
                }
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn part_one(board: &Board, path: &Vec<Move>) -> usize {
    let start = board.get_start();
    let shift: Shift = (1, 0);

    let solver = Flat { board };

    let (result, _) = solve(&solver, start, shift, path);
    result
}

fn part_two(board: &Board, path: &Vec<Move>) -> usize {
    let start = board.get_start();
    let shift: Shift = (1, 0);

    let solver = Cube {
        board,
        cube: &CUBE_TEST,
    };

    let (result, log) = solve(&solver, start, shift, path);

    // debug
    print_path(board, &log, 17, 13);

    result
}

fn solve<S: Solver>(
    solver: &S,
    start_postion: Position,
    start_shift: Shift,
    path: &Vec<Move>,
) -> (usize, Vec<(Position, Shift)>) {
    // 0. take start
    let mut position = start_postion;
    let mut shift = start_shift;
    let mut log = vec![(position, shift)];

    // 1. iter over path
    for m in path {
        match m {
            // 2. apply Straight move step by step
            Move::Straight(steps) => {
                for _ in 0..*steps {
                    let (next_position, tile) = solver.run(position, shift);
                    match tile {
                        Tile::Open => {
                            // do a regular move
                            position = next_position;

                            // trace path
                            log.push((position, shift));
                        }
                        Tile::Solid => {
                            break; // it stuck in Solid
                                   // stop moving step by step
                        }
                        Tile::Void => {
                            // it going step in Void: teleport
                            let (next_position, next_shift) = solver.teleport(position, shift);

                            position = next_position;
                            shift = next_shift;

                            // trace path
                            log.push((position, shift));
                        }
                    }
                }
            }
            // 3. apply Rotation move
            // do it in complex numbers
            // see link below for how it works
            // https://www.youtube.com/watch?v=5PcpBw5Hbwo
            // (PS: positive direction of Y is down)
            Move::Left => {
                shift = (shift.1, -shift.0);
            }
            Move::Right => {
                shift = (-shift.1, shift.0);
            }
        }
    }

    // 4. calculate score based on final position and move
    let facing = match shift {
        (1, 0) => 0,
        (-1, 0) => 2,
        (0, -1) => 3,
        (0, 1) => 1,
        _ => 0,
    };
    let result = 1000 * position.1 + 4 * position.0 + facing;

    (result, log)
}

fn main() {
    let (board, path) = read_input();

    let result = part_one(&board, &path);
    println!("Part one: {}", result);

    let result = part_two(&board, &path);
    println!("Part two: {}", result);
}
