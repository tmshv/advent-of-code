use std::{fmt::Debug, io, vec};

use moveslice::Moveslice;

#[derive(Debug)]
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
    grid: [u8; 60],
    high_index: usize,
    shift: u64,
}

impl Grid {
    fn new(grid: Option<[u8; 60]>) -> Grid {
        let grid = match grid {
            None => [
                0b1111111, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
                0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
                0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
                0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
                0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
                0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
                0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
                0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
                0b0000000, 0b0000000, 0b0000000, 0b0000000,
            ],
            Some(grid) => grid,
        };
        Grid {
            grid,
            high_index: 0,
            shift: 0,
        }
    }

    fn shift(&mut self) {
        // increment shift
        self.shift += 10;

        // 0. move bottom part of grid down one by one
        self.grid.moveslice(10.., 0);

        // 1. clear top of the grid
        self.grid[50..].fill(0);

        // decreate top
        // if self.high_index >= 10 {
        self.high_index -= 10;
        // }
    }

    fn draw_shape(&mut self, shape: &Shape) {
        for (y, row) in shape.iter_rows() {
            self.grid[y] = row | self.grid[y];
        }
        let h = shape.height - 1;
        let top = shape.location.1 + h;
        if top > self.high_index {
            self.high_index = top;
        }
    }

    fn contains(&self, shape: &Shape) -> bool {
        for (y, row) in shape.iter_rows() {
            if row & self.grid[y] > 0 {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Shape {
    location: (usize, usize),
    data: [u8; 4],
    width: usize,
    height: usize,
    right_border: usize,
}

impl Shape {
    fn new(width: usize, data: [u8; 4]) -> Shape {
        let height = data.iter().filter(|row| **row != 0).count();

        Shape {
            location: (0, 0),
            width,
            height,
            data,
            right_border: 1_000_000,
        }
    }

    fn set_right_border(&mut self, border: usize) {
        self.right_border = border - self.width;
    }

    fn set_location(&mut self, x: usize, y: usize) {
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

    fn move_left(&mut self) -> bool {
        if self.location.0 > 0 {
            self.location.0 -= 1;
            return true;
        }
        false
    }

    fn move_right(&mut self) -> bool {
        if self.location.0 < self.right_border {
            self.location.0 += 1;
            return true;
        }
        false
    }

    fn iter_rows(&self) -> impl Iterator<Item = (usize, u8)> + '_ {
        self.data[0..self.height]
            .iter()
            .enumerate()
            .map(|(i, row)| (self.location.1 + i, row >> self.location.0))
    }
}

fn get_shapes(right_border: usize) -> Vec<Shape> {
    let mut result = vec![
        // ####
        Shape::new(
            4,
            [
                0b1111000, // ####...
                0b0000000, // unused
                0b0000000, // unused
                0b0000000, // unused
            ],
        ),
        // .#.
        // ###
        // .#.
        Shape::new(
            3,
            [
                0b0100000, // .#.
                0b1110000, // ###
                0b0100000, // .#.
                0b0000000, // unused
            ],
        ),
        // ..#
        // ..#
        // ###
        Shape::new(
            3,
            [
                0b1110000, // ..#
                0b0010000, // ..#
                0b0010000, // ###
                0b0000000, // unused
                // 0b0010000, // ..#
                // 0b0010000, // ..#
                // 0b1110000, // ###
                // 0b0000000, // unused
            ],
        ),
        // #
        // #
        // #
        // #
        Shape::new(
            1,
            [
                0b1000000, // #
                0b1000000, // #
                0b1000000, // #
                0b1000000, // #
            ],
        ),
        // ##
        // ##
        Shape::new(
            2,
            [
                0b1100000, // ##
                0b1100000, // ##
                0b0000000, // unused
                0b0000000, // unused
            ],
        ),
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

fn solve(jets: Vec<Jet>, rocks: u64) -> Grid {
    let shapes = get_shapes(7);
    let mut shape_cycle = shapes.iter().cycle();
    let mut jet_cycle = jets.iter().cycle();

    let mut grid = Grid::new(None);
    for i in 0..rocks {
        // if i % 1_000_000_000 == 0 {
        // println!("{}", i);
        // }

        // 1. get next shape
        let mut rock = shape_cycle.next().unwrap().clone();

        // 2. fill grid with empty rows
        // take the height of rock + 3 rows for the bottom or last pixel
        let position = grid.high_index + 1 + 3;
        rock.set_location(2, position);

        println!("rock {} will start from {:?}", i, rock.location);

        // 3. drop it with jet stream until it will be at the bottom
        loop {
            let jet = jet_cycle.next().unwrap();
            match jet {
                Jet::Left => {
                    if rock.move_left() {
                        println!("move {:?}", jet);
                    } else {
                        // rock.move_right();
                        println!("move {:?} but nothing has happened", jet);
                    }
                    if grid.contains(&rock) {
                        rock.move_right();
                    }
                }
                Jet::Right => {
                    if rock.move_right() {
                        println!("move {:?}", jet);
                    } else {
                        // rock.move_left();
                        println!("move {:?} but nothing has happened", jet);
                    }
                    if grid.contains(&rock) {
                        rock.move_left();
                    }
                }
            }

            rock.move_down();
            if grid.contains(&rock) {
                println!("move down causing it to ground");
                rock.move_up();

                grid.draw_shape(&rock);
                break;
            } else {
                println!("move down");
            }
        }

        println!("top: {}", grid.high_index);

        if grid.high_index > 50 {
            grid.shift();
        }
    }
    grid
}

fn part_two(grid: &Grid) -> u64 {
    grid.shift + grid.high_index as u64
}

fn main() {
    // let rocks = 2022;
    // let rocks = 1_000_000;
    // let rocks = 1_000_000_000;
    // let rocks = 1_000_000_000_000;
    let rocks = 10;

    let jets = read_input();
    let grid = solve(jets, rocks);
    let top = part_two(&grid);

    // println!("Result: {} ({})", top, top == 3153);
    // println!("Result: {} ({})", top, top == 1553686);
    // println!("Result: {} ({})", top, top == 1553665705);
    println!("Result: {}", top);

    for i in grid.grid.iter().rev() {
        if *i == 0 {
            continue;
        }
        println!("{:#09b}", i);
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_shapes, part_two, solve, Grid, Jet, Shape};

    #[test]
    fn test_1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets = input
            .chars()
            .map(|value| match value {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => panic!("Wrong char"),
            })
            .collect::<Vec<Jet>>();
        println!("{:?}", jets);
        let grid = solve(jets, 1);
        let top = part_two(&grid);
        assert_eq!(top, 1);
    }

    #[test]
    fn test_2() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets = input
            .chars()
            .map(|value| match value {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => panic!("Wrong char"),
            })
            .collect::<Vec<Jet>>();
        let grid = solve(jets, 2);
        let top = part_two(&grid);
        assert_eq!(top, 4);
        let mut result: [u8; 60] = [
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, // 19
            0b0000000, // 18
            0b0000000, // 17
            0b0000000, // 16
            0b0000000, // 15
            0b0000000, // 14
            0b0000000, // 13
            0b0000000, // 12
            0b0000000, // 11
            0b0000000, // 10
            0b0000000, // 09
            0b0000000, // 08
            0b0000000, // 07
            0b0000000, // 06
            0b0000000, // 05
            0b0001000, // 04
            0b0011100, // 03
            0b0001000, // 02
            0b0011110, // 01
            0b1111111, // 00
        ];
        result.reverse();
        assert_eq!(grid.grid, result);
    }

    #[test]
    fn test_10() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets = input
            .chars()
            .map(|value| match value {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => panic!("Wrong char"),
            })
            .collect::<Vec<Jet>>();
        let grid = solve(jets, 10);
        let top = part_two(&grid);
        assert_eq!(top, 17);
        let mut result: [u8; 60] = [
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, // 19
            0b0000000, // 18
            0b0000100, // 17
            0b0000100, // 16
            0b0000110, // 15
            0b1100110, // 14
            0b1111110, // 13
            0b0111000, // 12
            0b0010000, // 11
            0b0111100, // 10 
            0b0000110, // 09
            0b0000110, // 08
            0b0000100, // 07
            0b0010100, // 06
            0b0010100, // 05
            0b1111100, // 04
            0b0011100, // 03
            0b0001000, // 02
            0b0011110, // 01
            0b1111111, // 00
        ];
        result.reverse();
        assert_eq!(grid.grid, result);
    }

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
        let grid = solve(jets, 2022);
        let top = part_two(&grid);
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
        let grid = solve(jets, 1_000_000);
        let top = part_two(&grid);
        assert_eq!(top, 1514288);
    }

    #[test]
    fn shift10() {
        let mut matrix: [u8; 60] = [
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0011100, // 19
            0b0001000, // 18
            0b0001000, // 17
            0b0001111, // 16
            0b0010001, // 15
            0b0000111, // 14
            0b0111100, // 13
            0b0100011, // 12
            0b1110011, // 11
            0b0100110, // 10
            0b0000110, // 09
            0b0010110, // 08
            0b0001110, // 07
            0b0011111, // 06
            0b0010000, // 05
            0b1110000, // 04
            0b0111100, // 03
            0b0100000, // 02
            0b1110000, // 01
            0b0111110, // 00
        ];
        matrix.reverse();
        let mut grid = Grid::new(Some(matrix));
        grid.high_index = 1_000_000;
        assert_eq!(grid.shift, 0);
        grid.shift();

        let mut result: [u8; 60] = [
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, // ...
            0b0011100, // 19
            0b0001000, // 18
            0b0001000, // 17
            0b0001111, // 16
            0b0010001, // 15
            0b0000111, // 14
            0b0111100, // 13
            0b0100011, // 12
            0b1110011, // 11
            0b0100110, // 10
        ];
        result.reverse();
        assert_eq!(grid.grid, result);
        assert_eq!(grid.shift, 10);
    }

    #[test]
    fn get_shapes_widths() {
        let widths: Vec<usize> = get_shapes(7).iter().map(|shape| shape.width).collect();
        assert_eq!(widths, vec![4, 3, 3, 1, 2]);
    }

    #[test]
    fn get_shapes_height() {
        let heights: Vec<usize> = get_shapes(7).iter().map(|shape| shape.height).collect();
        assert_eq!(heights, vec![1, 3, 3, 4, 2]);
    }

    #[test]
    fn get_shapes_right_border() {
        let rb: Vec<usize> = get_shapes(7)
            .iter()
            .map(|shape| shape.right_border)
            .collect();
        assert_eq!(rb, vec![3, 4, 4, 6, 5]);
    }

    #[test]
    fn binary_ops() {
        assert_eq!(0b1111000 >> 0, 0b1111000);
        assert_eq!(0b0001111 << 2, 0b0111100);
        assert_eq!(0b0001111 >> 2, 0b0000011);
        assert_eq!(0b0001111 & 0b0011000, 0b0001000);
    }

    #[test]
    fn shape_move_right() {
        let mut shape = Shape::new(
            4,
            [
                0b1111000, // ####...
                0b0000000, // unused
                0b0000000, // unused
                0b0000000, // unused
            ],
        );
        shape.set_right_border(7);
        shape.set_location(0, 3);
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(3, 0b1111000),]
        );

        shape.move_right();
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(3, 0b0111100),]
        );

        shape.move_right();
        shape.move_right();
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(3, 0b0001111),]
        );

        shape.move_right();
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(3, 0b0001111),]
        );

        let mut shape = Shape::new(
            3,
            [
                0b0100000, // .#.
                0b1110000, // ###
                0b0100000, // .#.
                0b0000000, // unused
            ],
        );
        shape.set_right_border(7);
        shape.set_location(0, 9);
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(09, 0b0100000), (10, 0b1110000), (11, 0b0100000),]
        );

        shape.move_right();
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(09, 0b0010000), (10, 0b0111000), (11, 0b0010000),]
        );

        shape.move_right();
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(09, 0b0001000), (10, 0b0011100), (11, 0b0001000),]
        );

        shape.move_right();
        shape.move_right();
        shape.move_right();
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(09, 0b0000010), (10, 0b0000111), (11, 0b0000010),]
        );
    }

    #[test]
    fn shape_apply_location() {
        let mut shape = Shape::new(4, [0b1111000, 0b0000000, 0b0000000, 0b0000000]);
        shape.set_location(1, 0);
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(0, 0b0111100),]
        );
        shape.set_location(3, 8);
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(8, 0b0001111),]
        );

        let mut shape = Shape::new(
            3,
            [
                0b0100000, // .#.
                0b1110000, // ###
                0b0100000, // .#.
                0b0000000, // unused
            ],
        );
        shape.set_location(2, 5);
        assert_eq!(
            shape.iter_rows().collect::<Vec<(usize, u8)>>(),
            vec![(5, 0b0001000), (6, 0b0011100), (7, 0b0001000),]
        );
    }

    #[test]
    fn grid_draw() {
        let mut grid = Grid::new(Some([
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000,
        ]));
        let mut shape = Shape::new(
            4,
            [
                0b1111000, // ####...
                0b0000000, // unused
                0b0000000, // unused
                0b0000000, // unused
            ],
        );
        shape.set_location(1, 3);
        grid.draw_shape(&shape);

        shape.set_location(3, 6);
        grid.draw_shape(&shape);

        let mut shape = Shape::new(
            3,
            [
                0b0100000, // .#.
                0b1110000, // ###
                0b0100000, // .#.
                0b0000000, // unused
            ],
        );
        shape.set_location(4, 1);
        grid.draw_shape(&shape);

        let mut result: [u8; 60] = [
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, // ...
            0b0000000, // 9
            0b0000000, // 8
            0b0000000, // 7
            0b0001111, // 6
            0b0000000, // 5
            0b0000000, // 4
            0b0111110, // 3
            0b0000111, // 2
            0b0000010, // 1
            0b0000000, // 0
        ];
        result.reverse();
        assert_eq!(grid.grid, result);
    }

    #[test]
    fn grid_contains() {
        let mut matrix: [u8; 60] = [
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, // ...
            0b0011000, // 9
            0b0011000, // 8
            0b0111100, // 7
            0b0010000, // 6
            0b0010000, // 5
            0b1110001, // 4
            0b0000011, // 3
            0b0000111, // 2
            0b0001111, // 1
            0b1111111, // 0
        ];
        matrix.reverse();
        let grid = Grid::new(Some(matrix));
        let mut shape = Shape::new(
            4,
            [
                0b1111000, // ####...
                0b0000000, // unused
                0b0000000, // unused
                0b0000000, // unused
            ],
        );
        shape.set_location(0, 0);
        assert_eq!(grid.contains(&shape), true);

        shape.set_location(2, 1);
        assert_eq!(grid.contains(&shape), true);

        shape.set_location(1, 2);
        assert_eq!(grid.contains(&shape), true);

        shape.set_location(1, 3);
        assert_eq!(grid.contains(&shape), false);

        shape.set_location(3, 5);
        assert_eq!(grid.contains(&shape), false);

        let mut shape = Shape::new(
            3,
            [
                0b0100000, // .#.
                0b1110000, // ###
                0b0100000, // .#.
                0b0000000, // unused
            ],
        );
        shape.set_location(4, 5);
        assert_eq!(grid.contains(&shape), false);

        shape.set_location(2, 7);
        assert_eq!(grid.contains(&shape), true);
    }

    #[test]
    fn grid_draw_top() {
        let mut grid = Grid::new(Some([
            0b1111111, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000,
            0b0000000, 0b0000000, 0b0000000, 0b0000000,
        ]));
        assert_eq!(grid.high_index, 0);

        let mut shape = Shape::new(
            3,
            [
                0b0100000, // .#.
                0b1110000, // ###
                0b0100000, // .#.
                0b0000000, // unused
            ],
        );

        shape.set_location(0, 15);
        grid.draw_shape(&shape);
        assert_eq!(grid.high_index, 17);

        shape.set_location(0, 5);
        grid.draw_shape(&shape);
        assert_eq!(grid.high_index, 17);
    }
}
