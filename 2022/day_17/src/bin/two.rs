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
    top: Option<i32>,
}

impl Grid {
    fn new(width: u8) -> Grid {
        Grid {
            grid: Vec::new(),
            width,
            top: None,
        }
    }

    fn height(&self) -> u32 {
        self.grid.len() as u32
    }

    fn draw_shape(&mut self, shape: &Shape) {
        // let mut top = self.top;
        for (x, y) in shape.iter_pixels() {
            // if y > self.top {
            //     self.top = y;
            // }
            self.grid[y as usize][x as usize] = 1;
        }
        // self.top = top;
        self.top = None;
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

    fn get_most_top(&mut self) -> i32 {
        match self.top {
            Some(top) => top,
            None => {
                let mut top = 0;
                'y: for (i, row) in self.grid.iter().rev().enumerate() {
                    let y = self.grid.len() - i;
                    for value in row {
                        if *value == 1 {
                            top = y as i32;
                            break 'y;
                        }
                    }
                }
                self.top = Some(top);
                top
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Shape {
    location: (i32, i32),
    data: Vec<(i32, i32)>,
    height: i32,
}

impl Shape {
    fn new(data: Vec<(i32, i32)>) -> Shape {
        let min_y = data.iter().map(|coord| coord.1).min().unwrap();
        let max_y = data.iter().map(|coord| coord.1).max().unwrap();
        let height = (max_y - min_y).abs() + 1;

        Shape {
            location: (0, 0),
            data,
            height,
        }
    }

    fn set_location(&mut self, x: i32, y: i32) {
        self.location = (x, y);
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
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;
        for (x, y) in self.iter_pixels() {
            if y < 0 || y >= height {
                return true;
            }
            if x < 0 || x >= width {
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
        Shape::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        // .#.
        // ###
        // .#.
        Shape::new(vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)]),
        // ..#
        // ..#
        // ###
        Shape::new(vec![(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)]),
        // #
        // #
        // #
        // #
        Shape::new(vec![(0, 0), (0, -1), (0, -2), (0, -3)]),
        // ##
        // ##
        Shape::new(vec![(0, 0), (0, -1), (1, 0), (1, -1)]),
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

fn solve(jets: Vec<Jet>, rocks: u64) -> u64 {
    let shapes = get_shapes();
    let mut shape_cycle = shapes.iter().cycle();
    let mut jet_cycle = jets.iter().cycle();
    let mut grid = Grid::new(7);
    for _ in 0..rocks {
        // 1. get next shape
        let mut rock = shape_cycle.next().unwrap().clone();

        // 3. fill grid with empty rows
        // take the height of rock + 3 rows for the bottom or last pixel
        let top = grid.get_most_top();
        let lines = grid.height() as i32;
        let new_lines = (rock.height + 3) - (lines - top);
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
    top as u64
}

fn main() {
    // let rocks = 2022;
    let rocks = 1_000_000;
    let jets = read_input();
    let top = solve(jets, rocks);
    println!("Result: {}", top);
}

#[cfg(test)]
mod tests {
    use crate::{solve, Jet};

    #[test]
    fn test_2022() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets = input
            .chars()
            .map(|value| match value {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => panic!("Wrong char"),
            })
            .collect::<Vec<Jet>>();
        let top = solve(jets, 2022);

        assert_eq!(top, 3068);
    }
}
