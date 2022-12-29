use petgraph::{algo::dijkstra, graph::NodeIndex, Graph};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io, vec,
};

const MINUTES: i32 = 30;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    id: usize,
    name: String,
    rate: i32,
    tunnels: HashSet<String>,
}

#[derive(Debug, Clone)]
struct System {
    graph: Graph<i32, i32>,
    nodes: HashMap<String, NodeIndex>,
    valves: HashMap<NodeIndex, Valve>,
}

impl System {
    fn new(valves: &Vec<Valve>) -> System {
        let mut graph = Graph::<i32, i32>::default();
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
        let valves = HashMap::from_iter(valves.iter().map(|v| {
            let id = nodes.get(&v.name).unwrap();
            (*id, v.clone())
        }));

        Self {
            graph,
            nodes,
            valves,
        }
    }

    fn node_id(&self, name: &String) -> NodeIndex {
        *self.nodes.get(name).unwrap()
    }

    fn get_valve(&self, id: &NodeIndex) -> &Valve {
        self.valves.get(id).unwrap()
    }

    fn get_shortest_path(&self, start: NodeIndex, next: NodeIndex) -> i32 {
        let res = dijkstra(&self.graph, start, Some(next), |_| 1);
        let score = *res.get(&next).unwrap();
        score
    }

    fn solve_from(&self, start: &String, time_left: i32) -> i32 {
        let start_id = self.node_id(start);
        let available_nodes = HashSet::from_iter(
            self.valves
                .iter()
                .filter(|(_, valve)| valve.rate > 0)
                .map(|(id, _)| *id),
        );

        let (score, _) =
            self.best_flow(start_id, HashSet::new(), time_left as i32, &available_nodes);
        score
    }

    fn best_flow(
        &self,
        from_node: NodeIndex,
        visited_nodes: HashSet<NodeIndex>,
        time_left: i32,
        available_nodes: &HashSet<NodeIndex>,
    ) -> (i32, HashSet<NodeIndex>) {
        // Check if there's any time left:
        if time_left <= 0 {
            // No time left to release any pressure.
            return (0, visited_nodes);
        }

        // Check if we've already visited this node:
        if visited_nodes.contains(&from_node) {
            // We've already done the best we can from here.
            return (0, visited_nodes);
        }

        // It costs a minute to open this valve:
        let valve = self.get_valve(&from_node);
        // Spend a minute to open it if it has flow:
        let mut t = time_left;
        if valve.rate > 0 {
            t -= 1
        };

        // Find total pressure released from opening this node:
        let valve_flow = valve.rate * t;

        // Add this node to the visited set:
        let mut new_visited_nodes = visited_nodes.clone();
        new_visited_nodes.insert(from_node);

        // Look at the score for all neighbors, take the best:
        let mut max_sub_score = 0;
        let mut max_sub_visited = HashSet::new();

        // Look at every major node as a possible next step:
        for n in available_nodes {
            // Skip this node:
            if from_node == *n {
                continue;
            }

            let dist_n = self.get_shortest_path(from_node, *n);
            let time = t - dist_n;

            // Find the best we could do if we went to n next:
            let (score, path) =
                self.best_flow(*n, new_visited_nodes.clone(), time, available_nodes);

            // If this beats the previous best, use this instead:
            if score > max_sub_score {
                max_sub_score = score;
                max_sub_visited = path;
            }
        }

        max_sub_visited.insert(from_node);
        (valve_flow + max_sub_score, max_sub_visited)
    }
}

fn parse_row(row: String, id: usize) -> Valve {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    // Valve HH has flow rate=22; tunnel leads to valve GG
    let pattern =
        Regex::new(r"Valve ([\w]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let cap = pattern.captures(row.as_str()).unwrap();
    let name = cap.get(1).unwrap().as_str();
    let rate = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
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

fn main() {
    let items = read_input();
    let sys = System::new(&items);
    let score = sys.solve_from(&"AA".to_string(), MINUTES);
    println!("Result: {}", score);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
}
