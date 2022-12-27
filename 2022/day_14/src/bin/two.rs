use std::{
    fmt::Debug,
    hash::Hash,
    io,
    ops::{Add, Sub},
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

struct Cave {
    start: Vector,
    grid: Vec<Vec<Tile>>,
}

impl Cave {
    fn new(width: usize, height: usize) -> Cave {
        let mut grid = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Tile::Air);
            }
            grid.push(row);
        }

        Cave {
            grid,
            start: Vector::zero(),
        }
    }

    fn shape(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn emit(&self) -> Vector {
        self.start.clone()
    }

    fn is_out(&self, tail: &Vector) -> bool {
        if tail.x < 0 {
            return true;
        }

        let last_x = self.grid[0].len() as isize;
        if tail.x > last_x - 1 {
            return true;
        }

        if tail.y < 0 {
            return true;
        }

        let last_y = self.grid.len() as isize;
        tail.y > last_y - 1
    }

    fn is_air(&self, tail: &Vector) -> bool {
        let x = tail.x as usize;
        let y = tail.y as usize;
        let t = &self.grid[y][x];
        match t {
            Tile::Air => true,
            _ => false,
        }
    }

    fn mark_tail(&mut self, tail: &Vector, t: Tile) {
        let x = tail.x as usize;
        let y = tail.y as usize;
        self.grid[y][x] = t;
    }

    fn draw(&mut self, from: &Vector, to: &Vector) {
        let mut cursor = from.clone();
        let mut step = *to - *from;
        step.norm();
        let length = 1 + from.dist_to(to);
        for _ in 0..length {
            let x = cursor.x as usize;
            let y = cursor.y as usize;
            self.grid[y][x] = Tile::Rock;

            cursor = cursor + step;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn zero() -> Self {
        Vector { x: 0, y: 0 }
    }

    fn max() -> Self {
        Vector {
            x: std::isize::MAX,
            y: std::isize::MAX,
        }
    }

    fn min() -> Self {
        Vector {
            x: std::isize::MIN,
            y: std::isize::MIN,
        }
    }

    fn dist_to(&self, other: &Self) -> u32 {
        if self.x == other.x {
            (self.y as i32 - other.y as i32).abs() as u32
        } else {
            (self.x as i32 - other.x as i32).abs() as u32
        }
    }

    fn norm(&mut self) {
        if self.x > 0 {
            self.x = 1;
        } else if self.x < 0 {
            self.x = -1;
        }
        if self.y > 0 {
            self.y = 1;
        } else if self.y < 0 {
            self.y = -1;
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_row(row: &str) -> Vec<Vector> {
    let parts: Vec<&str> = row.split(" -> ").collect();
    let mut coords = vec![];
    for value in parts {
        let xy: Vec<&str> = value.split(',').collect();
        let x = xy[0].parse::<isize>().unwrap();
        let y = xy[1].parse::<isize>().unwrap();
        coords.push(Vector { x, y });
    }
    coords
}

fn read_input() -> Cave {
    let mut paths = vec![];
    for line in io::stdin().lines() {
        match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => {
                let path = parse_row(value.as_str());
                paths.push(path);
            }
        }
    }

    let mut tl = Vector::max();
    let mut br = Vector::min();

    let start = Vector { x: 500, y: 0 };

    let mut coords = vec![start.clone()];
    for path in &paths {
        for coord in path {
            coords.push(coord.clone());
        }
    }

    for coord in &coords {
        if coord.x < tl.x {
            tl.x = coord.x;
        }
        if coord.y < tl.y {
            tl.y = coord.y;
        }

        if coord.x > br.x {
            br.x = coord.x;
        }
        if coord.y > br.y {
            br.y = coord.y;
        }
    }

    let cave_bounding_box = br - tl;
    let height = cave_bounding_box.y + 1 + 2;
    let canvas_dimension = Vector {
        x: height * 3,
        y: height,
    };
    let new_start = Vector {
        x: (canvas_dimension.x - 1) / 2,
        y: 0,
    };
    let shift = new_start - (start - tl);

    let mut cave = Cave::new(canvas_dimension.x as usize, canvas_dimension.y as usize);
    cave.start = start - tl + shift;
    for path in &paths {
        let end = path.len() - 1;
        for i in 0..end {
            let a = path[i] - tl + shift;
            let b = path[i + 1] - tl + shift;
            cave.draw(&a, &b);
        }
    }

    cave.draw(
        &Vector {
            x: 0,
            y: canvas_dimension.y - 1,
        },
        &Vector {
            x: canvas_dimension.x - 1,
            y: canvas_dimension.y - 1,
        },
    );

    cave
}

fn print_cave(cave: &Cave) {
    for (y, row) in cave.grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let cell = Vector {
                x: x as isize,
                y: y as isize,
            };
            let mut marker = match tile {
                Tile::Air => '.',
                Tile::Rock => '#',
                Tile::Sand => 'o',
            };
            if cell == cave.start {
                marker = '+';
            }
            print!("{}", marker);
        }
        println!("");
    }
}

fn main() {
    let mut cave = read_input();
    println!("Shape: {:?}", cave.shape());

    let down = Vector { x: 0, y: 1 };
    let left = Vector { x: -1, y: 1 };
    let right = Vector { x: 1, y: 1 };

    let mut count = 0u32;
    'sand: loop {
        let mut steps_made = 0u32;
        let mut sand = cave.emit();
        loop {
            let mut next_sand = sand + down;
            if cave.is_air(&next_sand) {
                sand = sand + down;
                steps_made += 1;
                continue;
            }

            next_sand = sand + left;
            if cave.is_air(&next_sand) {
                sand = next_sand;
                steps_made += 1;
                continue;
            }

            next_sand = sand + right;
            if cave.is_air(&next_sand) {
                sand = next_sand;
                steps_made += 1;
                continue;
            }

            count += 1;

            // exit condition
            next_sand = sand + down;
            if steps_made == 0 && !cave.is_air(&next_sand) {
                break 'sand;
            }

            cave.mark_tail(&sand, Tile::Sand);
            break;
        }
    }

    print_cave(&cave);
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use crate::{parse_row, Vector};

    #[test]
    fn vector_norm_1() {
        let mut v = Vector { x: 10, y: 0 };
        v.norm();
        assert_eq!(v, Vector { x: 1, y: 0 });
    }

    #[test]
    fn vector_norm_2() {
        let mut v = Vector { x: -10, y: 0 };
        v.norm();
        assert_eq!(v, Vector { x: -1, y: 0 });
    }

    #[test]
    fn vector_norm_3() {
        let mut v = Vector { x: 0, y: 10 };
        v.norm();
        assert_eq!(v, Vector { x: 0, y: 1 });
    }

    #[test]
    fn vector_norm_4() {
        let mut v = Vector { x: 0, y: -10 };
        v.norm();
        assert_eq!(v, Vector { x: 0, y: -1 });
    }

    #[test]
    fn vector_norm_5() {
        let mut v = Vector { x: 10, y: -10 };
        v.norm();
        assert_eq!(v, Vector { x: 1, y: -1 });
    }

    #[test]
    fn vector_norm_6() {
        let mut v = Vector { x: -10, y: 10 };
        v.norm();
        assert_eq!(v, Vector { x: -1, y: 1 });
    }

    #[test]
    fn parse_row_of_four_coords() {
        let result = parse_row("503,4 -> 502,4 -> 502,9 -> 494,9");
        assert_eq!(
            result,
            vec![
                Vector { x: 503, y: 4 },
                Vector { x: 502, y: 4 },
                Vector { x: 502, y: 9 },
                Vector { x: 494, y: 9 },
            ]
        );
    }
}
