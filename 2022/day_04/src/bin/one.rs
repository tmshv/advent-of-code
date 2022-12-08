use std::io;

struct Range {
    min: i32,
    max: i32,
}

fn parse_range(value: &str) -> Range {
    let xs: Vec<&str> = value.split('-').collect();
    let min = xs[0].parse::<i32>().unwrap();
    let max = xs[1].parse::<i32>().unwrap();

    Range { min, max }
}

fn is_contains(one: &Range, other: &Range) -> bool {
    one.min <= other.min && one.max >= other.max
}

fn main() {
    let mut count = 0;

    for x in io::stdin().lines() {
        let line = x.unwrap();
        let xs: Vec<&str> = line.split(',').collect();
        let r1 = parse_range(xs[0]);
        let r2 = parse_range(xs[1]);

        let contains = is_contains(&r1, &r2) || is_contains(&r2, &r1);

        if contains {
            count += 1;
        }

        println!("{:?} = {}", xs, contains);
    }

    println!("Contains pairs = {}", count);
}
