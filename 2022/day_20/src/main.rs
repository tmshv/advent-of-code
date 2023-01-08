use std::io;

fn read_input() -> Vec<i64> {
    io::stdin()
        .lines()
        .map(|line| match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => value.as_str().parse::<i64>().unwrap(),
        })
        .collect()
}

fn mix(xs: &mut Vec<(usize, i64)>, order: &Vec<(usize, i64)>) {
    for x in order {
        let i = xs.iter().position(|y| x == y).unwrap();
        let im = (i as i64 + x.1).rem_euclid((xs.len() - 1) as i64) as usize;

        xs.remove(i);
        xs.insert(im, *x);
    }
}

fn part_one(input: &Vec<i64>) -> i64 {
    let xs = input
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x))
        .collect::<Vec<(usize, i64)>>();
    let mut mixed = xs.clone();
    mix(&mut mixed, &xs);

    let zero = mixed.iter().position(|(_, x)| *x == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| {
            let index = (zero + i).rem_euclid(mixed.len());
            mixed[index].1
        })
        .sum()
}

fn part_two(input: &Vec<i64>) -> i64 {
    let decryption_key = 811589153;
    let xs = input
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x * decryption_key))
        .collect::<Vec<(usize, i64)>>();
    let mut mixed = xs.clone();

    for _ in 0..10 {
        mix(&mut mixed, &xs);
    }

    let zero = mixed.iter().position(|(_, x)| *x == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| {
            let index = (zero + i).rem_euclid(mixed.len());
            mixed[index].1
        })
        .sum()
}
fn main() {
    let items = read_input();

    let result = part_one(&items);
    println!("Part one: {}", result);

    let result = part_two(&items);
    println!("Part two: {}", result);
}

#[cfg(test)]
mod tests {}
