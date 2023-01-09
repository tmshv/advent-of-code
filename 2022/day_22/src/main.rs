use std::io;

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

    fn get_start(&self) -> (usize, usize) {
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

    fn tile_at(&self, position: (usize, usize)) -> Tile {
        let (x, y) = position;
        if y == 0 || x == 0 {
            return Tile::Void;
        }
        if y > self.grid.len() - 1 || x > self.grid[0].len() - 1 {
            return Tile::Void;
        }
        self.grid[y][x]
    }

    fn teleport_from(&self, position: (usize, usize), shift: (isize, isize)) -> (usize, usize) {
        let (sx, sy) = shift;

        // teleport to left
        if sx > 0 && sy == 0 {
            let y = position.1;
            for x in 0..position.0 {
                match self.tile_at((x, y)) {
                    Tile::Open => {
                        return (x, y);
                    }
                    Tile::Solid => {
                        return position;
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
            for x in (position.0..self.grid[0].len()).rev() {
                match self.tile_at((x, y)) {
                    Tile::Open => {
                        return (x, y);
                    }
                    Tile::Solid => {
                        return position;
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
                match self.tile_at((x, y)) {
                    Tile::Open => {
                        return (x, y);
                    }
                    Tile::Solid => {
                        return position;
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
            for y in (0..position.1).rev() {
                match self.tile_at((x, y)) {
                    Tile::Open => {
                        return (x, y);
                    }
                    Tile::Solid => {
                        return position;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        // println!("teleport from {:?}", position);

        position
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

fn part_one(board: &Board, path: &Vec<Move>) -> usize {
    // 0. take start
    let mut position = board.get_start();
    println!("starts from {:?}", position);

    let mut shift: (isize, isize) = (1, 0);
    let mut log = vec![(position, shift)];

    // 1. iter over path
    for m in path {
        match m {
            // 2. apply Straight move step by step
            Move::Straight(steps) => {
                for _ in 0..*steps {
                    let next_position = add(position, shift);
                    let tile = board.tile_at(next_position);
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
                            position = board.teleport_from(position, shift);

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

    // for y in 1..201 {
    //     for x in 1..151 {
    for y in 1..13 {
        for x in 1..17 {
            let pos = (x, y);
            let trace = log.iter().rev().position(|(p, s)| *p == pos);
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
                    let shift = log[trace].1;
                    match shift {
                        (-1, 0) => '<',
                        (1, 0) => '>',
                        (0, -1) => '^',
                        (0, 1) => 'v',
                        _ => '%',
                    }
                }
            };
            print!("{}", c);
        }
        println!("");
    }

    // 4. calculate score based on final position and move
    println!("end at {:?} ({:?})", position, shift);

    let facing = match shift {
        (1, 0) => 0,
        (-1, 0) => 2,
        (0, -1) => 3,
        (0, 1) => 1,
        _ => 0,
    };
    1000 * position.1 + 4 * position.0 + facing
}

fn main() {
    let (board, path) = read_input();

    let result = part_one(&board, &path);
    println!("Part one: {}", result);
}
