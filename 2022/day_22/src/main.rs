use std::io;

#[derive(Debug)]
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
    // grid: [[Tile; 150]; 200],
    grid: Vec<Vec<Tile>>,
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

fn read_input() -> (Vec<Move>, Board) {
    let mut board = Board { grid: Vec::new() };

    for x in io::stdin().lines() {
        let line = x.unwrap();
        if line.is_empty() {
            break;
        }

        let y = board.grid.len();
        board.grid.push(Vec::new());

        for (_, t) in line.chars().enumerate() {
            let tile = match t {
                '.' => Tile::Open,
                '#' => Tile::Solid,
                ' ' => Tile::Void,
                _ => panic!("unknown tile"),
            };
            board.grid[y].push(tile);
        }
    }

    let path = io::stdin().lines().next().unwrap().unwrap();
    let moves = parse_path(path);

    (moves, board)
}

fn main() {
    let (path, board) = read_input();
    println!("{:?}", board);
    println!("{:?}", path);
}
