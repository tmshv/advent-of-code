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

fn mix(xs: &mut Vec<(usize, i32)>, order: &Vec<(usize, i32)>) {
    for x in order {
        let i = xs.iter().position(|y| x == y).unwrap();
        let im = (i as i32 + x.1).rem_euclid((xs.len() - 1) as i32) as usize;

        xs.remove(i);
        xs.insert(im, *x);
    }
}

fn part_one(input: Vec<i32>) -> i32 {
    let xs = input
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x))
        .collect::<Vec<(usize, i32)>>();
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

fn main() {
    let items = read_input();

    let result = part_one(items);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {
}
