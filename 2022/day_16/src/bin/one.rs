use regex::Regex;
use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    io,
    iter::zip,
    ops::{Add, Sub},
    vec,
};

#[derive(Debug, PartialEq, Eq)]
struct Valve {
    name: String,
    rate: u32,
    open: bool,
    tunnels: HashSet<String>,
}

fn parse_row(row: String) -> Valve {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    // Valve HH has flow rate=22; tunnel leads to valve GG
    let pattern =
        Regex::new(r"Valve ([\w]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let cap = pattern.captures(row.as_str()).unwrap();
    let name = cap.get(1).unwrap().as_str();
    let rate = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let t: Vec<&str> = cap.get(3).unwrap().as_str().split(", ").collect();
    let tunnels = HashSet::<String>::from_iter(t.iter().map(|x| String::from(*x)));

    Valve {
        name: String::from(name),
        rate,
        open: false,
        tunnels,
    }
}

fn read_input() -> Vec<Valve> {
    let mut items = vec![];
    for line in io::stdin().lines() {
        let s = match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => parse_row(value),
        };
        items.push(s);
    }
    items
}

fn main() {
    let valves = read_input();

    for v in &valves {
        println!("{:?}", v);
    }

    // let count = set.len();
    // println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs};

    use crate::{parse_row, Valve};

    #[test]
    fn parse_row_with_many_tunnels() {
        let result = parse_row(String::from(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
        ));
        assert_eq!(
            result,
            Valve {
                name: "AA".to_string(),
                rate: 0,
                open: false,
                tunnels: HashSet::from_iter(
                    ["DD".to_string(), "II".to_string(), "BB".to_string(),]
                ),
            }
        );
    }

    #[test]
    fn parse_row_with_single_tunnel() {
        let result = parse_row(String::from(
            "Valve HH has flow rate=22; tunnel leads to valve GG",
        ));
        assert_eq!(
            result,
            Valve {
                name: "HH".to_string(),
                rate: 22,
                open: false,
                tunnels: HashSet::from_iter(["GG".to_string()]),
            }
        );
    }

    #[test]
    fn solve_test() {
        let file_contents = fs::read_to_string("test.txt").unwrap();
    }
}
