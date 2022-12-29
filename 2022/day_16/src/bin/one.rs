use petgraph::{algo::astar, graph::NodeIndex, prelude::UnGraph, Graph};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
    io, vec,
};

const MINUTES: u32 = 11;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Action {
    Start(String),
    Open(String),
    Move(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    id: usize,
    name: String,
    rate: u32,
    tunnels: HashSet<String>,
}

#[derive(Debug, Clone)]
struct System {
    graph: Graph<u32, u32>,
    nodes: HashMap<String, NodeIndex>,
    valves: HashMap<String, Valve>,
    open: HashSet<String>,
}

impl System {
    fn new(valves: &Vec<Valve>) -> System {
        let mut graph = Graph::<u32, u32>::default();
        let nodes: HashMap<String, NodeIndex> = HashMap::from_iter(
            valves
                .iter()
                .map(|v| (v.name.clone(), graph.add_node(v.rate))),
        );
        for v in valves {
            let a = nodes.get(&v.name).unwrap();
            for tunnel in &v.tunnels {
                let b = nodes.get(tunnel).unwrap();
                graph.add_edge(*a, *b, 1);
            }
        }
        Self {
            graph,
            nodes,
            valves: HashMap::from_iter(valves.iter().map(|v| (v.name.clone(), v.clone()))),
            open: HashSet::new(),
        }
    }

    fn get_next(&self, start: String, minutes_left: u32) {
        let start_id = self.node_id(&start);
        for (next, valve) in &self.valves {
            // skip if it is start
            if *next == start {
                continue;
            }
            // skip if it is open
            if self.open.contains(next) {
                continue;
            }
            // skip evaluating node with 0 rate
            if valve.rate == 0 {
                continue;
            }

            let next_id = self.node_id(next);
            // let res = dijkstra(&self.graph, start_id, Some(next_id), |_| 1);
            let path = astar(
                &self.graph,
                start_id,         // start
                |n| n == next_id, // is_goal
                |e| *e.weight(),  // edge_cost
                |_| 0,            // estimate_cost
            );

            match path {
                Some((travel_cost, path)) => {
                    if minutes_left < travel_cost {
                        println!("have no time to react valve by this path");
                        continue;
                    }

                    let time = minutes_left - travel_cost - 1; // -1 cause it will be wasted for opening
                    let pressure = time * valve.rate;

                    println!(
                        "{} -> {} travel={}, time={} * rate={} = pressure={}",
                        start, next, travel_cost, time, valve.rate, pressure
                    );
                }
                None => println!("There was no path"),
            }
        }
    }

    fn node_id(&self, name: &String) -> NodeIndex {
        *self.nodes.get(name).unwrap()
    }
}

#[derive(Debug, Clone)]
struct Variant {
    sequence: Vec<Action>,
    opened: HashSet<usize>,
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
            opened: HashSet::new(),
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
        self.sequence.len() as u32 == MINUTES + 1
    }

    fn open(&mut self, valve: &Valve) {
        self.opened.insert(valve.id);
    }

    fn can_open(&self, valve: &Valve) -> bool {
        // cannot open if its rate is 0
        // cannot open if last action is Open

        if valve.rate == 0 {
            return false;
        }

        !self.opened.contains(&valve.id)

        // let last = self.sequence.last().unwrap();
        // match last {
        //     Action::Open(_) => false,
        //     _ => true,
        // }
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

    fn released_pressure(&self, valves: &HashMap<String, &Valve>) -> i32 {
        let mut pressure = 0i32;
        // let mut minute = 0u32;

        for (i, action) in self.sequence.iter().enumerate() {
            match action {
                Action::Move(_) => {
                    // minute += 1;
                }
                Action::Open(name) => {
                    let minutes_left = MINUTES as i32 - i as i32; 
                    if minutes_left < 0 {
                        return 0;
                    }

                    let valve = valves.get(name).unwrap();
                    let rate = valve.rate;
                    // minute += 1;
                    // let minutes_left = M - minute;
                    pressure += minutes_left * rate as i32;
                }
                Action::Start(_) => continue,
            };
        }
        pressure
    }
}

fn parse_row(row: String, id: usize) -> Valve {
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
        id,
        name: String::from(name),
        rate,
        tunnels,
    }
}

fn read_input() -> Vec<Valve> {
    let mut items = vec![];
    let mut i = 0;
    for line in io::stdin().lines() {
        let s = match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => parse_row(value, i),
        };
        items.push(s);
        i += 1;
    }
    items
}

fn get_variants(
    variant: Variant,
    valves: &HashMap<String, &Valve>,
    time_left: u32,
) -> Vec<Variant> {
    // end recursion if list of sequence is 30 + 1
    if time_left == 0 {
        return vec![variant];
    }

    let current = variant.cursor();
    let valve = valves.get(current).unwrap();

    let mut result = vec![];

    // 1. open current valve branch
    if variant.can_open(valve) {
        let mut branch = variant.extend(Action::Open(current.clone()));
        branch.open(valve);
        for variant in get_variants(branch, valves, time_left - 1) {
            result.push(variant);
        }
    }

    // 2. move to others branch
    for next in &valve.tunnels {
        // let valve = valves.get(current).unwrap();
        // if !variant.can_move_to(&valve) || variant.is_valve_opened(next) {
        // if !variant.can_move_to(&valve) {
        //     continue;
        // }
        let branch = variant.extend(Action::Move(next.clone()));
        for variant in get_variants(branch, valves, time_left - 1) {
            result.push(variant);
        }
    }

    result
}

fn solve(valve: u32, valves_opened: u32, time_remaining: u32) -> u32 {
    // auto key = U*R.size()*31*2 + p1*31*2 + time*2 + other_players;
    let other_players = 0;
    let key = valve * 20 * 31 * 2 + valve * 31 * 2 + time_remaining * 2 + other_players;
    0
}

fn main() {
    let items = read_input();
    let valves = HashMap::from_iter(items.iter().map(|v| (v.name.clone(), v)));

    // let sys = System::new(&valves);
    // println!("{:?}", sys.get_next("AA".to_string(), M));

    let entrypoint = Variant::entrypoint("AA".to_string());
    let variants = get_variants(entrypoint, &valves, MINUTES);

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
        let result = parse_row(
            String::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            0,
        );
        assert_eq!(
            result,
            Valve {
                id: 0,
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
        let result = parse_row(
            String::from("Valve HH has flow rate=22; tunnel leads to valve GG"),
            0,
        );
        assert_eq!(
            result,
            Valve {
                id: 0,
                name: "HH".to_string(),
                rate: 22,
                tunnels: HashSet::from_iter(["GG".to_string()]),
            }
        );
    }

    #[test]
    fn solve_test() {
        let file_contents = fs::read_to_string("test.txt").unwrap();
    }
}
