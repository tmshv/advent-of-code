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
    grid: [[Tile; 150]; 200],
}

impl Board {
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
    let mut board = Board { grid: [[Tile::Void; 150]; 200] };

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
            board.grid[y][x] = tile;
        }
    }

    let path = io::stdin().lines().next().unwrap().unwrap();
    let moves = parse_path(path);

    (board, moves)
}

fn part_one(board: &Board, path: &Vec<Move>) -> usize {
    let start = board.get_start();
    println!("starts from {:?}", start);
    0
}

fn main() {
    let (board, path) = read_input();

    let result = part_one(&board, &path);
    println!("Part one: {}", result);
}

