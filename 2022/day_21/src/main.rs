use regex::Regex;
use std::{collections::HashMap, io, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Job {
    Number(i64),
    Operation((Op, String, String)),
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
                        let number = n.as_str().parse::<i64>().unwrap();
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

fn eval(tree: &HashMap<&str, Monkey>, key: &str) -> i64 {
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

// fn has_key(tree: &HashMap<&str, &Monkey>, start_key: &str, key: &str) -> bool {
//     if start_key == key {
//         return true;
//     }
//     let monkey = tree.get(&start_key).unwrap();
//     match &monkey.job {
//         Job::Number(n) => false,
//         Job::Operation(job) => {
//             let (op, key_a, key_b) = job;
//             let a = has_key(tree, key_a, key);
//             let b = has_key(tree, key_b, key);
//             a || b
//         }
//     }
// }

// fn transform_job(job: Job, key: String, op_key: String) -> Job {
//     match job {
//         Job::Number(_) => job,
//         Job::Operation(operation) => {
//             let (op, a, b) = operation;
//             if a == op_key {
//                 match op {
//                     Op::Add => {
//
//                     }
//                 }
//             }
//         }
//     }
// }

fn part_one(items: &Vec<Monkey>) -> i64 {
    // 1. items -> ast
    let tree: HashMap<&str, Monkey> = HashMap::from_iter(items.iter().map(|monkey| {
        let key = monkey.name.as_str();
        (key, monkey.clone())
    }));

    // 2. eval ast
    eval(&tree, "root")
}

fn part_two(items: &Vec<Monkey>) -> i64 {
    // 1. items -> ast
    let mut tree: HashMap<&str, Monkey> = HashMap::from_iter(items.iter().map(|monkey| {
        let key = monkey.name.as_str();
        (key, monkey.clone())
    }));

    // 3. eval A branch of ROOT
    let root = tree.get("root").unwrap();
    let root = Monkey {
        name: "root".to_string(),
        job: match &root.job {
            Job::Number(_) => panic!("root should be Op not Number"),
            Job::Operation(operation) => {
                let (_, a, b) = operation;
                Job::Operation((Op::Sub, a.to_string(), b.to_string()))
            }
        },
    };
    tree.insert("root", root);

    let mut answer = 0.0;
    let mut prev_answer = 1.0;
    let mut prev_err = 1.0;
    let learning_rate = 0.01;
    loop {
        // setup AST with current answer
        let humn = Monkey {
            name: "humn".to_string(),
            job: Job::Number(answer as i64),
        };
        tree.insert("humn", humn);

        // stop if ROOT = A - B = 0
        let result = eval(&tree, "root");
        if result == 0 {
            break;
        }

        // otherwise do GD step
        let err = result.abs() as f64;
        let err_diff = err - prev_err;
        let gradient = if err_diff == 0.0 {
            -1.0
        } else {
            (answer - prev_answer) / err_diff
        };
        answer -= learning_rate * err * gradient;

        // save of the previous attempt
        prev_answer = answer;
        prev_err = err;

        println!("{} -> {} ({})", answer, result, err);
    }

    answer as i64
}

// fn part_two(items: &Vec<Monkey>) -> i64 {
//     // 1. items -> ast
//     let mut tree: HashMap<&str, Monkey> = HashMap::from_iter(items.iter().map(|monkey| {
//         let key = monkey.name.as_str();
//         (key, monkey.clone())
//     }));
//
//     // // 2. set HUMN number 0
//     // let humn = Monkey {
//     //     name: "humn".to_string(),
//     //     // job: Job::Number(i32::MAX),
//     //     job: Job::Number(0),
//     // };
//     // // tree.insert("humn", &humn);
//
//     // 3. eval A branch of ROOT
//     // let root = tree.get("root").unwrap();
//     // let root_value = match &root.job {
//     //     Job::Number(_) => panic!("root should be Op not Number"),
//     //     Job::Operation(op) => {
//     //         let a_has_humn = has_key(&tree, &op.1, "humn");
//     //         if a_has_humn {
//     //             eval(&tree, &op.2)
//     //         } else {
//     //             eval(&tree, &op.1)
//     //         }
//     //     }
//     // };
//     // println!("result of new tree = {}", root_value);
//     //
//     // // 4. eval B brach of ROOT
//     // // 5. get |A - B|
//     // (a - b).abs()
//     answer
// }

fn main() {
    let items = read_input();

    let result = part_one(&items);
    println!("Part one: {}", result);

    let result = part_two(&items);
    println!("Part two: {}", result);
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
