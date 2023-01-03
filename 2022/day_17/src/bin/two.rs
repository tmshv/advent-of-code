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
    width: i32,
    height: i32,
    top: Option<i32>,
    shift: u64,
}

impl Grid {
    fn new(width: i32, fill: usize) -> Grid {
        let mut item = Grid {
            grid: Vec::with_capacity(fill),
            width,
            height: 0,
            top: None,
            shift: 0,
        };
        for _ in 0..fill {
            item.add_line();
        }
        item
    }

    fn shift(&mut self, steps: usize) {
        self.shift += steps as u64;
        shift_vec(&mut self.grid, steps);
        self.top = match self.top {
            None => None,
            Some(top) => Some(top - steps as i32),
        }
    }

    fn draw_shape(&mut self, shape: &Shape) {
        // let mut top = match self.top {
        //     None => 0,
        //     Some(top) => top,
        // };
        for (x, y) in shape.iter_pixels() {
            // if y > top {
            //     top = y;
            // }
            self.grid[y as usize][x as usize] = 1;
        }
        // self.top = Some(top);
        self.top = None;
    }

    fn add_line(&mut self) {
        let line = self.get_line();
        self.grid.push(line);
        self.height += 1;
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
            }
        }
    }

    fn contains(&self, shape: &Shape) -> bool {
        // let height = grid.len() as i32;
        // let width = grid[0].len() as i32;
        for (x, y) in shape.iter_pixels() {
            if y < 0 || y >= self.height {
                return true;
            }
            if x < 0 || x >= self.width {
                return true;
            }
            if self.grid[y as usize][x as usize] == 1 {
                return true;
            }
        }
        false
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

    let mut grid = Grid::new(7, 100);
    for _ in 0..rocks {
        // 1. get next shape
        let mut rock = shape_cycle.next().unwrap().clone();

        // 2. fill grid with empty rows
        // take the height of rock + 3 rows for the bottom or last pixel
        let top = grid.get_most_top();
        let position = top + rock.height + 2; // 2 means 3 pixels higher
        rock.set_location(2, position);

        // 3. drop it with jet stream until it will be at the bottom
        loop {
            let jet = jet_cycle.next().unwrap();
            match jet {
                Jet::Left => {
                    rock.move_left();
                    if grid.contains(&rock) {
                        rock.move_right();
                    }
                }
                Jet::Right => {
                    rock.move_right();
                    if grid.contains(&rock) {
                        rock.move_left();
                    }
                }
            }

            rock.move_down();
            if grid.contains(&rock) {
                rock.move_up();

                grid.draw_shape(&rock);

                break;
            }
        }

        if top > 90 {
            grid.shift(10);
        }
    }

    let top = grid.get_most_top();
    // top as u64
    grid.shift + top as u64
}

fn main() {
    // let rocks = 2022;
    let rocks = 1_000_000;

    let jets = read_input();
    let top = solve(jets, rocks);

    // println!("Result: {} ({})", top, top == 3153);
    println!("Result: {} ({})", top, top == 1553686);
}

fn shift_vec(grid: &mut Vec<Vec<u8>>, steps: usize) {
    // move bottom part of grid down one by one
    for i in 0..(grid.len() - steps) {
        grid[i] = grid[i + steps].clone();
    }

    // clear top of the grid
    let len = grid.len();
    let start = len - steps;
    let w = grid[0].len();
    for y in start..len {
        for x in 0..w {
            grid[y][x] = 0;
        }
    }
}

fn from_strs(rows: Vec<String>) -> Vec<Vec<u8>> {
    rows.iter()
        .rev()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{from_strs, get_shapes, shift_vec, solve, Jet};

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

    #[test]
    fn test_1000000() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets = input
            .chars()
            .map(|value| match value {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => panic!("Wrong char"),
            })
            .collect::<Vec<Jet>>();
        let top = solve(jets, 1_000_000);

        assert_eq!(top, 1514288);
    }

    #[test]
    fn shift10() {
        let mut grid = from_strs(vec![
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            "....#..".to_string(),
            "....#..".to_string(),
            "....##.".to_string(),
            "##..##.".to_string(),
            "######.".to_string(),
            ".###...".to_string(),
            "..#....".to_string(),
            ".####..".to_string(),
            "....##.".to_string(),
            "....##.".to_string(),
            "....#..".to_string(),
            "..#.#..".to_string(),
            "..#.#..".to_string(),
            "#####..".to_string(),
            "..###..".to_string(),
            "...#...".to_string(),
            "..####.".to_string(),
        ]);
        let result = from_strs(vec![
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            ".......".to_string(),
            "....#..".to_string(),
            "....#..".to_string(),
            "....##.".to_string(),
            "##..##.".to_string(),
            "######.".to_string(),
            ".###...".to_string(),
            "..#....".to_string(),
            // ".####..".to_string(),
            // "....##.".to_string(),
            // "....##.".to_string(),
            // "....#..".to_string(),
            // "..#.#..".to_string(),
            // "..#.#..".to_string(),
            // "#####..".to_string(),
            // "..###..".to_string(),
            // "...#...".to_string(),
            // "..####.".to_string(),
        ]);
        shift_vec(&mut grid, 10);
        assert_eq!(grid, result);
    }

    #[test]
    fn get_shapes_height() {
        let heights: Vec<i32> = get_shapes().iter().map(|shape| shape.height).collect();
        assert_eq!(heights, vec![1, 3, 3, 4, 2]);
    }
}
