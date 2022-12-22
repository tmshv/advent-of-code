use std::{collections::HashSet, io};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: u32,
}

fn read_input() -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for x in io::stdin().lines() {
        match x {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => {
                let row: Vec<&str> = value.split(" ").collect();
                if row.len() == 2 {
                    let steps = row[1].parse::<u32>().unwrap();
                    let direction = match row[0] {
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        _ => {
                            panic!("Input data is wrong");
                        }
                    };
                    moves.push(Move { direction, steps });
                } else {
                    panic!("Input data is wrong");
                }
            }
        }
    }
    moves
}

fn norm(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}

fn is_cover(tail: &Vector, head: &Vector) -> bool {
    tail == head
}

fn is_touching(tail: &Vector, head: &Vector) -> bool {
    let x = (tail.x - head.x).abs();
    let y = (tail.y - head.y).abs();
    (x == 0 && y == 1) || (x == 1 && y == 0)
}

fn is_diagonally_touching(tail: &Vector, head: &Vector) -> bool {
    (tail.x - head.x).abs() == 1 && (tail.y - head.y).abs() == 1
}

fn move_tail(tail: &Vector, head: &Vector) -> Vector {
    if is_cover(tail, head) || is_touching(tail, head) || is_diagonally_touching(tail, head) {
        return tail.clone();
    }

    return Vector {
        x: tail.x + norm(head.x - tail.x),
        y: tail.y + norm(head.y - tail.y),
    };
}

fn main() {
    let moves = read_input();

    let mut tail = Vector { x: 0, y: 0 };
    let mut head = Vector { x: 0, y: 0 };

    let mut log: Vec<Vector> = vec![];
    log.push(tail.clone());

    for m in &moves {
        for _ in 0..m.steps {
            // move head according to direction
            match m.direction {
                Direction::Left => {
                    head.x -= 1;
                }
                Direction::Right => {
                    head.x += 1;
                }
                Direction::Up => {
                    head.y -= 1;
                }
                Direction::Down => {
                    head.y += 1;
                }
            }

            tail = move_tail(&tail, &head);
            log.push(tail.clone());
        }
    }

    println!("Head end: {:?}", head);
    println!("Tail end: {:?}", tail);

    let unique_cells: HashSet<Vector> = HashSet::from_iter(log.iter().cloned());
    println!("Unique tail positions: {:?}", unique_cells.len());
}
