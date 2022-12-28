use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
    io, vec,
};

const M: u32 = 30;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Action {
    Start(String),
    Open(String),
    Move(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: HashSet<String>,
}

#[derive(Debug, Clone)]
struct Variant {
    sequence: Vec<Action>,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for action in &self.sequence {
            match action {
                Action::Start(name) => {
                    write!(f, "->{} ", name)?;
                }
                Action::Open(name) => {
                    write!(f, "^{} ", name)?;
                }
                Action::Move(name) => {
                    write!(f, ">{} ", name)?;
                }
            }
        }
        Ok(())
    }
}

impl Variant {
    fn entrypoint(name: String) -> Variant {
        Variant {
            sequence: vec![Action::Start(name)],
        }
    }

    fn cursor(&self) -> &String {
        let last = self.sequence.last().unwrap();
        match last {
            Action::Start(name) => name,
            Action::Open(name) => name,
            Action::Move(name) => name,
        }
    }

    fn extend(&self, action: Action) -> Variant {
        let mut clone = self.clone();
        clone.sequence.push(action);
        clone
    }

    fn is_full(&self) -> bool {
        self.sequence.len() as u32 == M
    }

    fn can_open(&self, valve: &Valve) -> bool {
        // cannot open if its rate is 0
        // cannot open if last action is Open

        if valve.rate == 0 {
            return false;
        }

        let last = self.sequence.last().unwrap();
        match last {
            Action::Open(_) => false,
            _ => true,
        }
    }

    fn can_move_to(&self, valve: &Valve) -> bool {
        // cannot move if last action is Move
        // but can if that valve is zero rate

        if valve.rate == 0 {
            return true;
        }

        let last = self.sequence.last().unwrap();
        match last {
            Action::Move(_) => false,
            _ => true,
        }
    }

    fn is_valve_opened(&self, name: &String) -> bool {
        for action in &self.sequence {
            match action {
                Action::Open(valve_name) => {
                    if valve_name == name {
                        return true;
                    }
                }
                _ => continue,
            }
        }
        false
    }

    fn released_pressure(&self, valves: &HashMap<String, &Valve>) -> u32 {
        let mut pressure = 0u32;
        let mut minute = 0u32;

        for (i, action) in self.sequence.iter().enumerate() {
            match action {
                Action::Move(_) => {
                    minute += 1;
                }
                Action::Open(name) => {
                    let valve = valves.get(name).unwrap();
                    let rate = valve.rate;
                    minute += 1;
                    let minutes_left = M - minute;
                    pressure += minutes_left * rate;
                }
                Action::Start(_) => continue,
            };
        }
        pressure
    }
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

fn get_variants(variant: Variant, valves: &HashMap<String, &Valve>) -> Vec<Variant> {
    // end recursion on:
    // 1. list of sequence is 30
    if variant.is_full() {
        // println!("full variant {:?}", variant);

        return vec![variant];
    }
    // 2. all valves are open
    let all_open = valves.iter().all(|(name, _)| variant.is_valve_opened(name));
    if all_open {
        println!("ALL ARE OPENED {:?}", variant);
        return vec![variant];
    }

    let current = variant.cursor();
    let valve = valves.get(current).unwrap();

    let mut result = vec![];

    // 1. open current valve (if we were moved)
    if variant.can_open(valve) {
        let branch1 = variant.extend(Action::Open(current.clone()));
        for variant in get_variants(branch1, valves) {
            result.push(variant);
        }
    }

    // 2. move to others
    for next in &valve.tunnels {
        let valve = valves.get(current).unwrap();
        if !variant.can_move_to(&valve) || variant.is_valve_opened(next) {
            continue;
        }
        let branch = variant.extend(Action::Move(next.clone()));
        for variant in get_variants(branch, valves) {
            result.push(variant);
        }
    }

    result
}

fn main() {
    let items = read_input();
    let valves = HashMap::from_iter(items.iter().map(|v| (v.name.clone(), v)));

    for v in &valves {
        println!("{:?}", v);
    }

    let entrypoint = Variant::entrypoint("AA".to_string());
    let variants = get_variants(entrypoint, &valves);

    for variant in &variants {
        let pressure = variant.released_pressure(&valves);
        println!("{} = {}", variant, pressure);
    }
    println!("");

    let best = variants.iter().max_by(|v1, v2| {
        v1.released_pressure(&valves)
            .cmp(&v2.released_pressure(&valves))
    });
    match best {
        None => {}
        Some(variant) => {
            let pressure = variant.released_pressure(&valves);
            println!("{}", variant);
            println!("Result: {}", pressure);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        fs,
    };

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
                tunnels: HashSet::from_iter(["GG".to_string()]),
            }
        );
    }

    #[test]
    fn test_variant_generation() {
        let mut valves = HashMap::new();
        let aa = Valve {
            name: "AA".to_string(),
            rate: 0,
            tunnels: HashSet::from_iter(["BB".to_string()]),
        };
        valves.insert("AA".to_string(), &aa);

        let bb = Valve {
            name: "AA".to_string(),
            rate: 0,
            tunnels: HashSet::new(),
            // tunnels: HashSet::from_iter(["AA".to_string()]),
        };
        valves.insert("BB".to_string(), &bb);
    }

    #[test]
    fn solve_test() {
        let file_contents = fs::read_to_string("test.txt").unwrap();
    }
}
