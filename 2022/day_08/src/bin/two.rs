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

fn look_left(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = grid[y][x];
    let row = &grid[y];
    let mut count = 0;
    for i in (0..x).rev() {
        count += 1;
        if row[i] >= value {
            break;
        }
    }
    count
}

fn look_right(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = grid[y][x];
    let row = &grid[y];
    if x >= row.len() - 1 {
        return 0;
    }
    let mut count = 0;
    for i in (x + 1)..row.len() {
        count += 1;
        if row[i] >= value {
            break;
        }
    }
    count
}

fn look_up(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = grid[y][x];
    let mut count = 0;
    for i in (0..y).rev() {
        count += 1;
        if grid[i][x] >= value {
            break;
        }
    }
    count
}

fn look_down(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = grid[y][x];
    if y >= grid.len() - 1 {
        return 0;
    }
    let mut count = 0;
    for i in (y + 1)..grid.len() {
        count += 1;
        if grid[i][x] >= value {
            break;
        }
    }
    count
}

fn main() {
    let grid = read_input();
    let mut max_value = 0u32;

    for row in &grid {
        for value in row {
            print!("{}", value);
        }
        println!("");
    }
    println!("");

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let l = look_left(&grid, x, y);
            let r = look_right(&grid, x, y);
            let u = look_up(&grid, x, y);
            let d = look_down(&grid, x, y);
            let score = l * r * u * d;
            println!("x={} y={} score={}", x, y, score);

            if score > max_value {
                max_value = score;
            }
        }
    }

    println!("Result {:?}", max_value);
}
