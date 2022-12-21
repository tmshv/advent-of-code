use std::io;

fn read_input() -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = vec![];
    for x in io::stdin().lines() {
        match x {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => {
                let row: Vec<u32> = value.chars().map(|c| c.to_digit(10).unwrap()).collect();
                grid.push(row);
            }
        }
    }
    grid
}

fn is_visible_left(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let value = grid[y][x];
    let row = &grid[y];
    for i in 0..x {
        let cell = row[i];
        if cell >= value {
            return false;
        }
    }
    true
}

fn is_visible_right(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let value = grid[y][x];
    let row = &grid[y];
    if x >= row.len() {
        return true;
    }
    for i in ((x + 1)..(row.len())).rev() {
        let cell = row[i];
        if cell >= value {
            return false;
        }
    }
    true
}

fn is_visible_top(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let value = grid[y][x];
    for i in 0..y {
        let cell = grid[i][x];
        if cell >= value {
            return false;
        }
    }
    true
}

fn is_visible_bottom(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let value = grid[y][x];
    if y >= grid.len() {
        return true;
    }
    for i in ((y + 1)..grid.len()).rev() {
        let cell = grid[i][x];
        if cell >= value {
            return false;
        }
    }
    true
}

fn main() {
    let grid = read_input();
    let mut count = 0u32;

    // for row in &grid {
    //     for value in row {
    //         print!("{}", value);
    //     }
    //     println!("");
    // }
    // println!("");

    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if false
                || is_visible_left(&grid, x, y)
                || is_visible_right(&grid, x, y)
                || is_visible_top(&grid, x, y)
                || is_visible_bottom(&grid, x, y)
            {
                print!("{}", value);

                count += 1;
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");

    println!("Result {:?}", count);
}
