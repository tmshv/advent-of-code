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
    grid: [[u8; 7]; 60],
    top: i32,
    shift: u64,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            grid: [
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ],
            top: 0,
            shift: 0,
        }
    }

    fn shift(&mut self, steps: usize) {
        // increment shift
        self.shift += steps as u64;

        // 0. move bottom part of grid down one by one
        for i in 0..(self.grid.len() - steps) {
            self.grid[i] = self.grid[i + steps];
        }

        // 1. clear top of the grid
        let len = self.grid.len();
        let start = len - steps;
        let w = self.grid[0].len();
        for y in start..len {
            for x in 0..w {
                self.grid[y][x] = 0;
            }
        }

        // decreate top
        self.top -= steps as i32;
    }

    fn draw_shape(&mut self, shape: &Shape) {
        let mut top = 0;
        for (x, y) in shape.iter_pixels() {
            if y >= top {
                top = y + 1;
            }
            self.grid[y][x] = 1;
        }

        let top = top as i32;
        if self.top < top {
            self.top = top;
        };
    }

    fn contains(&self, shape: &Shape) -> bool {
        shape.iter_pixels().any(|(x, y)| self.grid[y][x] == 1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Shape {
    location: (i32, i32),
    data: [(i32, i32); 5],
    height: i32,
    right_border: i32,
}

impl Shape {
    fn new(data: [(i32, i32); 5]) -> Shape {
        let min_y = data.iter().map(|coord| coord.1).min().unwrap();
        let max_y = data.iter().map(|coord| coord.1).max().unwrap();
        let height = (max_y - min_y).abs() + 1;

        Shape {
            location: (0, 0),
            data,
            height,
            right_border: 1_000_000,
        }
    }

    fn set_right_border(&mut self, border: i32) {
        let width = 1 + self.data.iter().map(|coord| coord.0).max().unwrap();
        self.right_border = border - width;
    }

    fn set_location(&mut self, x: i32, y: i32) {
        self.location = (x, y);
    }

    fn move_down(&mut self) -> bool {
        if self.location.1 > 0 {
            self.location.1 -= 1;
            return true;
        }
        false
    }

    fn move_up(&mut self) {
        self.location.1 += 1;
    }

    fn move_left(&mut self) {
        if self.location.0 > 0 {
            self.location.0 -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.location.0 < self.right_border {
            self.location.0 += 1;
        }
    }

    fn iter_pixels(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let iter = self
            .data
            .iter()
            .map(|(x, y)| ((self.location.0 + x) as usize, (self.location.1 + y) as usize));
        iter
    }
}

fn get_shapes(right_border: i32) -> Vec<Shape> {
    let mut result = vec![
        // ####
        Shape::new([(0, 0), (1, 0), (2, 0), (3, 0), (0, 0)]),
        // .#.
        // ###
        // .#.
        Shape::new([(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)]),
        // ..#
        // ..#
        // ###
        Shape::new([(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)]),
        // #
        // #
        // #
        // #
        Shape::new([(0, 0), (0, -1), (0, -2), (0, -3), (0, 0)]),
        // ##
        // ##
        Shape::new([(0, 0), (0, -1), (1, 0), (1, -1), (0, 0)]),
    ];

    for shape in &mut result {
        shape.set_right_border(right_border);
    }

    result
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
    let shapes = get_shapes(7);
    let mut shape_cycle = shapes.iter().cycle();
    let mut jet_cycle = jets.iter().cycle();

    let mut grid = Grid::new();
    for _ in 0..rocks {
        // 1. get next shape
        let mut rock = shape_cycle.next().unwrap().clone();

        // 2. fill grid with empty rows
        // take the height of rock + 3 rows for the bottom or last pixel
        let position = grid.top + rock.height + 2; // 2 means 3 pixels higher
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

            let failing = rock.move_down();
            if !failing || grid.contains(&rock) {
                if failing {
                    rock.move_up();
                }

                grid.draw_shape(&rock);

                break;
            }
        }

        let top = grid.top;
        if top > 50 {
            grid.shift(10);
        }
    }

    grid.shift + grid.top as u64
}

fn main() {
    // let rocks = 2022;
    let rocks = 1_000_000;

    let jets = read_input();
    let top = solve(jets, rocks);

    // println!("Result: {} ({})", top, top == 3153);
    println!("Result: {} ({})", top, top == 1553686);
}

#[cfg(test)]
mod tests {
    use crate::{get_shapes, solve, Jet};

    // fn from_strs(rows: Vec<String>) -> Vec<Vec<u8>> {
    //     rows.iter()
    //         .rev()
    //         .map(|row| {
    //             row.chars()
    //                 .map(|c| match c {
    //                     '.' => 0,
    //                     '#' => 1,
    //                     _ => 0,
    //                 })
    //                 .collect()
    //         })
    //         .collect()
    // }

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

    // #[test]
    // fn shift10() {
    //     let mut grid = from_strs(vec![
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         "....#..".to_string(),
    //         "....#..".to_string(),
    //         "....##.".to_string(),
    //         "##..##.".to_string(),
    //         "######.".to_string(),
    //         ".###...".to_string(),
    //         "..#....".to_string(),
    //         ".####..".to_string(),
    //         "....##.".to_string(),
    //         "....##.".to_string(),
    //         "....#..".to_string(),
    //         "..#.#..".to_string(),
    //         "..#.#..".to_string(),
    //         "#####..".to_string(),
    //         "..###..".to_string(),
    //         "...#...".to_string(),
    //         "..####.".to_string(),
    //     ]);
    //     let result = from_strs(vec![
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         ".......".to_string(),
    //         "....#..".to_string(),
    //         "....#..".to_string(),
    //         "....##.".to_string(),
    //         "##..##.".to_string(),
    //         "######.".to_string(),
    //         ".###...".to_string(),
    //         "..#....".to_string(),
    //         // ".####..".to_string(),
    //         // "....##.".to_string(),
    //         // "....##.".to_string(),
    //         // "....#..".to_string(),
    //         // "..#.#..".to_string(),
    //         // "..#.#..".to_string(),
    //         // "#####..".to_string(),
    //         // "..###..".to_string(),
    //         // "...#...".to_string(),
    //         // "..####.".to_string(),
    //     ]);
    //     shift_vec(&mut grid, 10);
    //     assert_eq!(grid, result);
    // }

    #[test]
    fn get_shapes_height() {
        let heights: Vec<i32> = get_shapes(7).iter().map(|shape| shape.height).collect();
        assert_eq!(heights, vec![1, 3, 3, 4, 2]);
    }

    #[test]
    fn get_shapes_right_border() {
        let rb: Vec<i32> = get_shapes(7)
            .iter()
            .map(|shape| shape.right_border)
            .collect();
        assert_eq!(rb, vec![3, 4, 4, 6, 5]);
    }
}
