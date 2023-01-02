use std::{
    fmt::Debug,
    io,
    vec,
};

enum Jet {
    Left,
    Right,
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

fn get_line() -> Vec<u8> {
    vec![0, 0, 0, 0, 0, 0, 0]
}

fn draw_shape(shape: &Shape, grid: &mut Vec<Vec<u8>>) {
    for (x, y) in shape.iter_pixels() {
        grid[y as usize][x as usize] = 1;
    }
}

fn get_most_top(grid: &Vec<Vec<u8>>) -> i32 {
    for (i, row) in grid.iter().rev().enumerate() {
        let y = grid.len() - i;
        for value in row {
            if *value == 1 {
                return y as i32;
            }
        }
    }
    0
}

fn display_grid(grid: &Vec<Vec<u8>>, shape: Option<&Shape>) {
    for (i, row) in grid.iter().rev().enumerate() {
        let y = grid.len() - i - 1;
        print!("{:0>4} |", y);
        for (x, value) in row.iter().enumerate() {
            let mut pixel = match value {
                0 => '.',
                1 => '#',
                _ => ' ',
            };
            match shape {
                None => {}
                Some(shape) => {
                    if shape.include(x as i32, y as i32) {
                        pixel = '@';
                    }
                }
            }
            print!("{}", pixel);
        }
        print!("|");
        println!();
    }
    println!("     +-------+");
}

fn main() {
    let shapes = get_shapes();
    let mut shape_cycle = shapes.iter().cycle();
    let jets = read_input();
    let mut jet_cycle = jets.iter().cycle();

    // 7 |-------|
    // 6 |-------|
    // 5 |-------|
    // 4 |-------|
    // 3 |-------|
    // 2 |-------|
    // 1 |-------|
    // 0 |-------|
    //   +-------+
    let mut grid: Vec<Vec<u8>> = vec![];

    let rocks = 2022;
    // let rocks = 5;
    for _ in 0..rocks {
        // 1. get next shape
        let mut rock = shape_cycle.next().unwrap().clone();

        // 3. fill grid with empty rows
        // take the height of rock + 3 rows for the bottom or last pixel
        let height = rock.height();
        let top = get_most_top(&grid);
        let lines = grid.len() as i32;
        let new_lines = (height + 3) - (lines - top);
        for _ in 0..new_lines {
            grid.push(get_line());
        }

        // start Y coordinate of rock
        let mut position = grid.len() as i32 - 1;
        // substract |new_lines| if no new lines has pushed to the grid (rock will fail not from the top)
        if new_lines < 0 {
            position -= new_lines.abs();
        }

        // 2. place this shape on the canvas
        rock.set_location(2, position);

        // println!("The N rock begins falling {:?}:", rock.location);
        // display_grid(&grid, Some(&rock));
        // println!("");

        // 3. drop it with jet stream until it will be at the bottom
        loop {
            let jet = jet_cycle.next().unwrap();
            match jet {
                Jet::Left => {
                    rock.move_left();
                    if rock.is_overlap(&grid) {
                        rock.move_right();
                    }
                }
                Jet::Right => {
                    rock.move_right();
                    if rock.is_overlap(&grid) {
                        rock.move_left();
                    }
                }
            }
            // let message = match jet {
            //     Jet::Left => {
            //         // "Jet of gas pushes rock left, but nothing happens:"
            //         "Jet of gas pushes rock left:"
            //     }
            //     Jet::Right => {
            //         "Jet of gas pushes rock right:"
            //         // "Jet of gas pushes rock right, but nothing happens:"
            //     }
            // };
            // println!("{}", message);
            // display_grid(&grid, Some(&rock));
            // println!("");

            rock.move_down();
            if rock.is_overlap(&grid) {
                rock.move_up();

                draw_shape(&rock, &mut grid);

                // println!("Rock falls 1 unit, causing it to come to rest:");
                // display_grid(&grid, None);
                // println!("");

                break;
            }
            // println!("Rock falls 1 unit:");
            // display_grid(&grid, Some(&rock));
            // println!("");
        }
    }

    // display
    println!("Final:");
    display_grid(&grid, None);

    let top = get_most_top(&grid);
    println!("Result: {}", top);
}
