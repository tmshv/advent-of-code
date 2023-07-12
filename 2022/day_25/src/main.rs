use std::io;

#[derive(Debug)]
struct Snafu(String);

impl From<String> for Snafu {
    fn from(value: String) -> Self {
        Snafu(value)
    }
}

impl From<&Snafu> for isize {
    fn from(snafu: &Snafu) -> Self {
        snafu
            .0
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let m = 5isize.pow(i as u32);
                match c {
                    '=' => -2 * m,
                    '-' => -1 * m,
                    '0' => 0 * m,
                    '1' => 1 * m,
                    '2' => 2 * m,
                    _ => panic!("UB"),
                }
            })
            .sum()
    }
}

fn read_input() -> Vec<Snafu> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(Snafu::from)
        .collect()
}

fn part_one(numbers: &Vec<Snafu>) -> String {
    let total: isize = numbers.iter().map(isize::from).sum();

    String::from("?")
}

fn main() {
    let numbers = read_input();

    println!("{:?}", numbers);

    let result = part_one(&numbers);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::Snafu;

    #[test]
    fn test_snafu_to_decimal() {
        let snafu = Snafu::from(String::from("1=-0-2"));
        assert_eq!(isize::from(&snafu), 1747);

        let snafu = Snafu::from(String::from("12111"));
        assert_eq!(isize::from(&snafu), 906);

        let snafu = Snafu::from(String::from("2=0="));
        assert_eq!(isize::from(&snafu), 198);

        let snafu = Snafu::from(String::from("21"));
        assert_eq!(isize::from(&snafu), 11);

        let snafu = Snafu::from(String::from("2=01"));
        assert_eq!(isize::from(&snafu), 201);

        let snafu = Snafu::from(String::from("111"));
        assert_eq!(isize::from(&snafu), 31);

        let snafu = Snafu::from(String::from("20012"));
        assert_eq!(isize::from(&snafu), 1257);

        let snafu = Snafu::from(String::from("112"));
        assert_eq!(isize::from(&snafu), 32);

        let snafu = Snafu::from(String::from("1=-1="));
        assert_eq!(isize::from(&snafu), 353);

        let snafu = Snafu::from(String::from("1-12"));
        assert_eq!(isize::from(&snafu), 107);

        let snafu = Snafu::from(String::from("12"));
        assert_eq!(isize::from(&snafu), 7);

        let snafu = Snafu::from(String::from("1="));
        assert_eq!(isize::from(&snafu), 3);

        let snafu = Snafu::from(String::from("122"));
        assert_eq!(isize::from(&snafu), 37);
    }
}
