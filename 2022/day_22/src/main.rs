use std::{
    cmp::{max, min},
    io,
};

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
    // A -> B
    (
        Edge {
            size: 4,
            a: (12, 5),
            n: (0, 1), // down
            clockwise: false,
        },
        Edge {
            size: 4,
            a: (16, 9),
            n: (-1, 0), // left
            clockwise: false,
        },
    ),
    // C -> D
    (
        Edge {
            size: 4,
            a: (13, 12),
            n: (1, 0), // right
            clockwise: false,
        },
        Edge {
            size: 4,
            a: (1, 5),
            n: (0, 1), // down
            clockwise: false,
        },
    ),
    // E -> F
    (
        Edge {
            size: 4,
            a: (1, 8),
            n: (1, 0), // right
            clockwise: false,
        },
        Edge {
            size: 4,
            a: (12, 12),
            n: (-1, 0), // left
            clockwise: false,
        },
    ),
    // G -> H
    (
        Edge {
            size: 4,
            a: (5, 5),
            n: (1, 0), // right
            clockwise: false,
        },
        Edge {
            size: 4,
            a: (9, 1),
            n: (0, 1), // down
            clockwise: false,
        },
    ),
    // ? -> ?
    (
        Edge {
            size: 1,
            a: (0, 0),
            n: (0, 0),
            clockwise: false,
        },
        Edge {
            size: 1,
            a: (0, 0),
            n: (0, 0),
            clockwise: false,
        },
    ),
    // ? -> ?
    (
        Edge {
            size: 1,
            a: (0, 0),
            n: (0, 0),
            clockwise: false,
        },
        Edge {
            size: 1,
            a: (0, 0),
            n: (0, 0),
            clockwise: false,
        },
    ),
    // ? -> ?
    (
        Edge {
            size: 1,
            a: (0, 0),
            n: (0, 0),
            clockwise: false,
        },
        Edge {
            size: 1,
            a: (0, 0),
            n: (0, 0),
            clockwise: false,
        },
    ),
];

#[allow(dead_code)]
const CUBE: [(Edge, Edge); 7] = [
    // A
    (
        Edge {
            size: 50,
            a: (100, 100),
            n: (0, -1), // up
            clockwise: false,
        },
        Edge {
            size: 50,
            a: (150, 50),
            n: (-1, 0), // left
            clockwise: true,
        },
    ),
    // B
    (
        Edge {
            size: 50,
            a: (51, 51),
            n: (0, 1), // down
            clockwise: false,
        },
        Edge {
            size: 50,
            a: (1, 101),
            n: (1, 0), // right
            clockwise: true,
        },
    ),
    // C
    (
        Edge {
            size: 50,
            a: (100, 150),
            n: (-1, 0), // left
            clockwise: true,
        },
        Edge {
            size: 50,
            a: (50, 200),
            n: (0, -1), // up
            clockwise: false,
        },
    ),
    // D
    (
        Edge {
            size: 50,
            a: (51, 1),
            n: (0, 1), // down
            clockwise: false,
        },
        Edge {
            size: 50,
            a: (1, 150),
            n: (0, -1), // up
            clockwise: true,
        },
    ),
    // E
    (
        Edge {
            size: 50,
            a: (100, 1),
            n: (-1, 0), // left
            clockwise: false,
        },
        Edge {
            size: 50,
            a: (1, 200),
            n: (0, -1), // up
            clockwise: true,
        },
    ),
    // F
    (
        Edge {
            size: 50,
            a: (150, 1),
            n: (-1, 0), // left
            clockwise: false,
        },
        Edge {
            size: 50,
            a: (50, 200),
            n: (-1, 0), // left
            clockwise: true,
        },
    ),
    // G
    (
        Edge {
            size: 50,
            a: (150, 1),
            n: (0, 1), // down
            clockwise: true,
        },
        Edge {
            size: 50,
            a: (100, 150),
            n: (0, -1), // up
            clockwise: false,
        },
    ),
];

type Point = (usize, usize);
type Shift = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    Solid,
    Void,
}

#[derive(Debug, Clone, Copy)]
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

    fn get_start(&self) -> Point {
        for y in 0..self.height() {
            for x in 0..self.width() {
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

    fn tile_at(&self, position: Point) -> Tile {
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
    fn step(&self, position: Point, shift: Shift) -> (Point, Tile);
    fn teleport(&self, position: Point, shift: Shift) -> Option<(Point, Shift)>;
}

struct Flat<'a> {
    board: &'a Board,
}

impl<'a> Solver for Flat<'a> {
    fn step(&self, position: Point, shift: Shift) -> (Point, Tile) {
        let next_position = add(position, shift);
        let tile = self.board.tile_at(next_position);
        (next_position, tile)
    }

    fn teleport(&self, position: Point, shift: Shift) -> Option<(Point, Shift)> {
        let (sx, sy) = shift;

        // teleport to left
        if sx > 0 && sy == 0 {
            let y = position.1;
            for x in 0..position.0 {
                match self.board.tile_at((x, y)) {
                    Tile::Open => {
                        return Some(((x, y), shift));
                    }
                    Tile::Solid => {
                        return Some((position, shift));
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
                        return Some(((x, y), shift));
                    }
                    Tile::Solid => {
                        return Some((position, shift));
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
                        return Some(((x, y), shift));
                    }
                    Tile::Solid => {
                        return Some((position, shift));
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
                        return Some(((x, y), shift));
                    }
                    Tile::Solid => {
                        return Some((position, shift));
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        Some((position, shift))
    }
}

struct Cube<'a> {
    board: &'a Board,
    cube: &'a [(Edge, Edge); 7],
}

impl<'a> Solver for Cube<'a> {
    fn step(&self, position: Point, shift: Shift) -> (Point, Tile) {
        let next_position = add(position, shift);
        let tile = self.board.tile_at(next_position);
        (next_position, tile)
    }

    fn teleport(&self, position: Point, shift: Shift) -> Option<(Point, Shift)> {
        let mut edge_from = self.cube[0].0;
        let mut edge_to = self.cube[0].1;
        for (a, b) in self.cube {
            // point is on the edge and this edge is orthogonal to move
            if a.contains(position) && a.scalar(shift) == 0 {
                edge_from = *a;
                edge_to = *b;
                break;
            }

            // point is on the edge and this edge is orthogonal to move
            if b.contains(position) && b.scalar(shift) == 0 {
                edge_from = *b;
                edge_to = *a;
                break;
            }
        }

        let relative = edge_from.get_relative(position);
        let next_position = edge_to.from_relative(relative);
        let next_shift = edge_to.get_teleportation_shift();

        // Check if teleportation is blocked by obstacle
        match self.board.tile_at(next_position) {
            Tile::Solid => None,
            _ => Some((next_position, next_shift)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    // Edge is defined by start coordinate A, direction N and size
    size: usize,
    a: Point,
    n: Shift,
    clockwise: bool,
}

impl Edge {
    fn end(&self) -> Point {
        let size = (self.size - 1) as isize;
        (
            (self.a.0 as isize + self.n.0 * size) as usize,
            (self.a.1 as isize + self.n.1 * size) as usize,
        )
    }

    fn is_vertical(&self) -> bool {
        // Y of A == Y of B
        // self.a.1 == self.b.1
        self.n.1 != 0
    }

    // normal of the edge is defined by counterclockwise 90 degrees rotation
    // or by clockwise 90 degrees rotation if edge is LEFT oriented
    fn get_teleportation_shift(&self) -> Shift {
        if self.clockwise {
            (-self.n.1, self.n.0)
        } else {
            (self.n.1, -self.n.0)
        }
    }

    fn scalar(&self, shift: Shift) -> isize {
        self.n.0 * shift.0 + self.n.1 * shift.1
    }

    // check position is within Edge
    fn contains(&self, position: Point) -> bool {
        let (x, y) = position;
        let (a, b) = self.get_corners();

        let x_min = min(a.0, b.0);
        let x_max = max(a.0, b.0);
        let y_min = min(a.1, b.1);
        let y_max = max(a.1, b.1);

        let h = x >= x_min && x <= x_max;
        let v = y >= y_min && y <= y_max;
        h && v
    }

    fn get_relative(&self, position: Point) -> isize {
        let (px, py) = position;
        let (ax, ay) = self.a;
        let px = px as isize;
        let py = py as isize;
        let ax = ax as isize;
        let ay = ay as isize;
        if self.is_vertical() {
            (py - ay).abs()
        } else {
            (px - ax).abs()
        }
    }

    fn from_relative(&self, relative: isize) -> Point {
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

    fn get_corners(&self) -> (Point, Point) {
        (self.a, self.end())
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

fn add(position: Point, shift: Shift) -> Point {
    let (x, y) = position;
    let (sx, sy) = shift;
    ((x as isize + sx) as usize, (y as isize + sy) as usize)
}

fn print_path(board: &Board, path: &Vec<(Point, Shift)>, max_x: usize, max_y: usize) {
    for y in 1..max_y {
        for x in 1..max_x {
            let pos = (x, y);
            let trace = path.iter().rev().find(|(p, _)| *p == pos);
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
                    let (_, shift) = trace;
                    match shift {
                        (-1, 0) => '<',
                        (1, 0) => '>',
                        (0, -1) => '^',
                        (0, 1) => 'v',
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

    let solver = Cube { board, cube: &CUBE };
    // let solver = Cube { board, cube: &CUBE_TEST };

    let (result, log) = solve(&solver, start, shift, path);

    // debug
    // print_path(board, &log, 17, 13);
    // print_path(board, &log, 151, 201);

    // for p in log.iter() {
    //     let (pos, shift) = p;
    //     let (x, y) = pos;
    //     println!("{} {} ({:?})", x, y, shift);
    // }

    result
}

fn solve<S: Solver>(
    solver: &S,
    start_postion: Point,
    start_shift: Shift,
    moves: &Vec<Move>,
) -> (usize, Vec<(Point, Shift)>) {
    // 0. take start
    let mut position = start_postion;
    let mut shift = start_shift;
    let mut log = vec![(position, shift)];

    // 1. iter over path
    for m in moves {
        match m {
            // 2. apply Straight move step by step
            Move::Straight(steps) => {
                for _ in 0..*steps {
                    let (next_position, tile) = solver.step(position, shift);
                    match tile {
                        Tile::Open => {
                            // Do a regular move
                            position = next_position;

                            // trace path
                            log.push((position, shift));
                        }
                        Tile::Solid => {
                            // It looking at obstacle
                            // Stop moving
                            break;
                        }
                        Tile::Void => {
                            // it going step in Void: teleporting
                            if let Some(t) = solver.teleport(position, shift) {
                                let (next_position, next_shift) = t;

                                position = next_position;
                                shift = next_shift;

                                // trace path
                                log.push((position, shift));
                            } else {
                                // Teleporting to obstacle. Ignore
                                break;
                            }
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
                // trace path
                log.push((position, shift));
            }
            Move::Right => {
                shift = (-shift.1, shift.0);
                // trace path
                log.push((position, shift));
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

#[cfg(test)]
mod test {
    use crate::Edge;

    #[test]
    fn edge_get_relative() {
        let e = Edge {
            size: 50,
            a: (100, 100),
            n: (0, -1), // up
            clockwise: false,
        };
        // same point as edge A
        assert_eq!(e.get_relative((100, 100)), 0);

        // end of edge
        assert_eq!(e.get_relative((100, 51)), 49);

        // just at point on the edge
        assert_eq!(e.get_relative((100, 95)), 5);
    }

    #[test]
    fn edge_to_relative() {
        let e = Edge {
            size: 50,
            a: (100, 100),
            n: (0, -1), // up
            clockwise: false,
        };

        assert_eq!(e.from_relative(0), (100, 100));
        assert_eq!(e.from_relative(20), (100, 80));
        assert_eq!(e.from_relative(49), (100, 51));
    }
}
