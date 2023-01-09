use regex::Regex;
use std::{collections::HashMap, io, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug, PartialEq, Eq)]
enum Job {
    Number(usize),
    Operation((Op, String, String)),
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    name: String,
    job: Job,
}

#[derive(Debug)]
struct ParseMonkeyError;

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^(\w{4}): ((\d+)|(\w{4}) ([*-/+]) (\w{4}))$").unwrap();
        let cap = pattern.captures(s);
        match cap {
            None => Err(ParseMonkeyError),
            Some(cap) => {
                let name = cap.get(1).unwrap().as_str().to_string();
                let job = match cap.get(3) {
                    Some(n) => {
                        let number = n.as_str().parse::<usize>().unwrap();
                        Job::Number(number)
                    }
                    None => {
                        let a = cap.get(4).unwrap().as_str().to_string();
                        let b = cap.get(6).unwrap().as_str().to_string();
                        let op = match cap.get(5).unwrap().as_str() {
                            "+" => Op::Add,
                            "-" => Op::Sub,
                            "/" => Op::Div,
                            "*" => Op::Mul,
                            _ => panic!("op is undefined"),
                        };
                        Job::Operation((op, a, b))
                    }
                };
                Ok(Monkey { name, job })
            }
        }
    }
}

fn read_input() -> Vec<Monkey> {
    io::stdin()
        .lines()
        .map(|line| line.expect("monkey string").parse::<Monkey>().unwrap())
        .collect()
}

fn eval(tree: &HashMap<&str, &Monkey>, key: &str) -> usize {
    let monkey = tree.get(&key).unwrap();

    match &monkey.job {
        Job::Number(n) => *n,
        Job::Operation(job) => {
            let (op, key_a, key_b) = job;
            let a = eval(tree, key_a);
            let b = eval(tree, key_b);

            match op {
                Op::Add => a + b,
                Op::Sub => a - b,
                Op::Div => a / b,
                Op::Mul => a * b,
            }
        }
    }
}

fn part_one(items: &Vec<Monkey>) -> usize {
    // 1. items -> ast
    let tree: HashMap<&str, &Monkey> = HashMap::from_iter(items.iter().map(|monkey| {
        let key = monkey.name.as_str();
        (key, monkey)
    }));

    // 2. eval ast
    eval(&tree, "root")
}

fn main() {
    let items = read_input();

    let result = part_one(&items);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{Job, Monkey, Op};

    #[test]
    fn monkey_parse_from() {
        let m = "root: pppw + sjmn".parse::<Monkey>().unwrap();
        assert_eq!(
            m,
            Monkey {
                name: "root".to_string(),
                job: Job::Operation((Op::Add, "pppw".to_string(), "sjmn".to_string()))
            }
        );

        let m = "dbpl: 5".parse::<Monkey>().unwrap();
        assert_eq!(
            m,
            Monkey {
                name: "dbpl".to_string(),
                job: Job::Number(5),
            }
        );

        let m = "ptdq: humn - dvpt".parse::<Monkey>().unwrap();
        assert_eq!(
            m,
            Monkey {
                name: "ptdq".to_string(),
                job: Job::Operation((Op::Sub, "humn".to_string(), "dvpt".to_string()))
            }
        );

        let m = "sjmn: drzm * dbpl".parse::<Monkey>().unwrap();
        assert_eq!(
            m,
            Monkey {
                name: "sjmn".to_string(),
                job: Job::Operation((Op::Mul, "drzm".to_string(), "dbpl".to_string()))
            }
        );

        let m = "pppw: cczh / lfqf".parse::<Monkey>().unwrap();
        assert_eq!(
            m,
            Monkey {
                name: "pppw".to_string(),
                job: Job::Operation((Op::Div, "cczh".to_string(), "lfqf".to_string()))
            }
        );
    }
}
