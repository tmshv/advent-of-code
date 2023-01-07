use std::io;

fn read_input() -> Vec<i32> {
    io::stdin()
        .lines()
        .map(|line| match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => value.as_str().parse::<i32>().unwrap(),
        })
        .collect()
}

fn part_one(input: Vec<i32>) -> i32 {
    0
}

fn main() {
    let blueprints = read_input();

    let result = part_one(vec![1, 2, -3, 3, -2, 0, 4]);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {}
