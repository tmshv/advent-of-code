use std::{fmt::Debug, io, vec};

enum Jet {
    Left,
    Right,
}

// 7 |-------|
// 6 |-------|
// 5 |-------|
// 4 |-------|
// 3 |-------|
// 2 |-------|
// 1 |-------|
// 0 |-------|
//   +-------+
#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
    width: u8,
}

impl Grid {
    fn new(width: u8) -> Grid {
        Grid {
            grid: Vec::new(),
            width,
        }
    }

    fn height(&self) -> u32 {
        self.grid.len() as u32
    }

    fn draw_shape(&mut self, shape: &Shape) {
        for (x, y) in shape.iter_pixels() {
            self.grid[y as usize][x as usize] = 1;
        }
    }

    fn add_line(&mut self) {
        let line = self.get_line();
        self.grid.push(line);
    }

    fn get_line(&self) -> Vec<u8> {
        let mut line = vec![];
        for _ in 0..self.width {
            line.push(0);
        }
        line
    }

    fn get_most_top(&self) -> u32 {
        for (i, row) in self.grid.iter().rev().enumerate() {
            let y = self.grid.len() - i;
            for value in row {
                if *value == 1 {
                    return y as u32;
                }
            }
        }
        0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Shape {
    location: (i32, i32),
    data: Vec<(i32, i32)>,
}

impl Shape {
    fn set_location(&mut self, x: i32, y: i32) {
        self.location = (x, y);
    }

    fn include(&self, x: i32, y: i32) -> bool {
        for (px, py) in self.iter_pixels() {
            if px == x && py == y {
                return true;
            }
        }
        false
    }

    fn height(&self) -> i32 {
        let min_y = self.data.iter().map(|coord| coord.1).min().unwrap();
        let max_y = self.data.iter().map(|coord| coord.1).max().unwrap();
        (max_y - min_y).abs() + 1
    }

    fn move_down(&mut self) {
        self.location.1 -= 1;
    }

    fn move_up(&mut self) {
        self.location.1 += 1;
    }

    fn move_left(&mut self) {
        self.location.0 -= 1;
    }

    fn move_right(&mut self) {
        self.location.0 += 1;
    }

    fn iter_pixels(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let iter = self
            .data
            .iter()
            .map(|(x, y)| (self.location.0 + x, self.location.1 + y));
        iter
    }

    fn is_overlap(&self, grid: &Vec<Vec<u8>>) -> bool {
        for (x, y) in self.iter_pixels() {
            if y < 0 || y >= grid.len() as i32 {
                return true;
            }
            if x < 0 || x >= grid[0].len() as i32 {
                return true;
            }
            if grid[y as usize][x as usize] == 1 {
                return true;
            }
        }
        false
    }
}

fn get_shapes() -> Vec<Shape> {
    vec![
        // ####
        Shape {
            location: (0, 0),
            data: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        // .#.
        // ###
        // .#.
        Shape {
            location: (0, 0),
            data: vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)],
        },
        // ..#
        // ..#
        // ###
        Shape {
            location: (0, 0),
            data: vec![(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)],
        },
        // #
        // #
        // #
        // #
        Shape {
            location: (0, 0),
            data: vec![(0, 0), (0, -1), (0, -2), (0, -3)],
        },
        // ##
        // ##
        Shape {
            location: (0, 0),
            data: vec![(0, 0), (0, -1), (1, 0), (1, -1)],
        },
    ]
}

fn read_input() -> Vec<Jet> {
    let line = io::stdin().lines().next().unwrap();
    match line {
        Err(error) => {
            panic!("{}", error);
        }
        Ok(value) => value
            .chars()
            .map(|char| match char {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => panic!("Wrong char"),
            })
            .collect(),
    }
}

fn main() {
    let shapes = get_shapes();
    let mut shape_cycle = shapes.iter().cycle();
    let jets = read_input();
    let mut jet_cycle = jets.iter().cycle();
    let mut grid = Grid::new(7);
    let rocks = 2022;
    for _ in 0..rocks {
        // 1. get next shape
        let mut rock = shape_cycle.next().unwrap().clone();

        // 3. fill grid with empty rows
        // take the height of rock + 3 rows for the bottom or last pixel
        let height = rock.height();
        let top = grid.get_most_top() as i32;
        let lines = grid.height() as i32;
        let new_lines = (height + 3) - (lines - top);
        for _ in 0..new_lines {
            grid.add_line();
        }

        // start Y coordinate of rock
        let mut position = grid.height() as i32 - 1;
        // substract |new_lines| if no new lines has pushed to the grid (rock will fail not from the top)
        if new_lines < 0 {
            position -= new_lines.abs();
        }

        // 2. place this shape on the canvas
        rock.set_location(2, position);

        // 3. drop it with jet stream until it will be at the bottom
        loop {
            let jet = jet_cycle.next().unwrap();
            match jet {
                Jet::Left => {
                    rock.move_left();
                    if rock.is_overlap(&grid.grid) {
                        rock.move_right();
                    }
                }
                Jet::Right => {
                    rock.move_right();
                    if rock.is_overlap(&grid.grid) {
                        rock.move_left();
                    }
                }
            }

            rock.move_down();
            if rock.is_overlap(&grid.grid) {
                rock.move_up();

                grid.draw_shape(&rock);

                break;
            }
        }
    }

    let top = grid.get_most_top();
    println!("Result: {}", top);
}