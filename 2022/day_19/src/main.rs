use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    io,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    time: u16,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
}

impl State {
    fn has_time(&self) -> bool {
        self.time > 0
    }

    fn enough_resources(&self, cost: (u16, u16, u16, u16)) -> bool {
        let (ore, clay, obsidian, geode) = cost;
        self.ore >= ore && self.clay >= clay && self.obsidian >= obsidian && self.geode >= geode
    }

    fn create_robot(&mut self, robot: (u16, u16, u16, u16), cost: (u16, u16, u16, u16)) {
        // + robots
        self.ore_robots += robot.0;
        self.clay_robots += robot.1;
        self.obsidian_robots += robot.2;
        self.geode_robots += robot.3;

        // - resources
        self.ore -= cost.0;
        self.clay -= cost.1;
        self.obsidian -= cost.2;
        self.geode -= cost.3;
    }
}

// #[derive(Debug, PartialEq, Eq)]
// struct ParseVoxelError;

// impl FromStr for Voxel {
//     type Err = ParseVoxelError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let parts = s.split(',').collect::<Vec<&str>>();
//         if parts.len() != 3 {
//             return Err(ParseVoxelError);
//         }
//         let x = parts[0].parse::<i32>().map_err(|_| ParseVoxelError)?;
//         let y = parts[1].parse::<i32>().map_err(|_| ParseVoxelError)?;
//         let z = parts[2].parse::<i32>().map_err(|_| ParseVoxelError)?;
//         Ok(Voxel::new(x, y, z))
//     }
// }

#[derive(Debug, Clone)]
struct Blueprint {
    id: u16,

    ore_robot_cost: (u16, u16, u16, u16),
    clay_robot_cost: (u16, u16, u16, u16),
    obsidian_robot_cost: (u16, u16, u16, u16),
    geode_robot_cost: (u16, u16, u16, u16),
}

impl Blueprint {
    fn evaluate(&self, state: State) -> u16 {
        let mut deq = VecDeque::from([state]);
        let mut seen = HashSet::<State>::new();

        // evaluate new states starting from current amount of geode earned
        let mut max_geodes = state.geode;
        while deq.len() > 0 {
            let state = deq.pop_front().unwrap();

            if seen.contains(&state) {
                continue;
            } else {
                seen.insert(state.clone());
            }

            // end of recursion
            // at the last minute just return total earned geode
            if !state.has_time() {
                // if state.geode > 5 || state.geode_robots > 2 {
                //     println!("stop of good variant at M{} {:?}", minute, state);
                // }
                // return state.geode;
                continue;
            }

            if state.geode + state.geode_robots > max_geodes {
                max_geodes = state.geode + state.geode_robots;
            }

            // see what robots can be factored according to resources
            // with amount of resources in the state at the begining of the minute
            let mut next_states = Vec::new();
            for (robot, can_build) in self.robots_to_build(&state).iter().enumerate() {
                if *can_build {
                    let mut next_state = state.clone();
                    let (robot, cost) = match robot {
                        0 => ((1, 0, 0, 0), self.ore_robot_cost),
                        1 => ((0, 1, 0, 0), self.clay_robot_cost),
                        2 => ((0, 0, 1, 0), self.obsidian_robot_cost),
                        3 => ((0, 0, 0, 1), self.geode_robot_cost),
                        _ => {
                            panic!("unreachable");
                        }
                    };
                    next_state.create_robot(robot, cost);
                    next_states.push(next_state);
                }
            }

            // add current state too
            // as an option if strategy is to accumulate resources
            next_states.push(state.clone());

            // end of minute
            // add resource earned by robots
            // using amount of robots from the state at the begining of the minute
            for s in &mut next_states {
                s.ore += state.ore_robots;
                s.clay += state.clay_robots;
                s.obsidian += state.obsidian_robots;
                s.geode += state.geode_robots;
                s.time -= 1;

                deq.push_back(*s);
            }
        }
        max_geodes
    }

    fn robots_to_build(&self, state: &State) -> [bool; 4] {
        [
            state.enough_resources(self.ore_robot_cost),
            state.enough_resources(self.clay_robot_cost),
            state.enough_resources(self.obsidian_robot_cost),
            state.enough_resources(self.geode_robot_cost),
        ]
    }
}

// fn read_input() -> Vec<Voxel> {
//     io::stdin()
//         .lines()
//         .map(|line| match line {
//             Err(error) => {
//                 panic!("{}", error);
//             }
//             Ok(value) => value.as_str().parse::<Voxel>().unwrap(),
//         })
//         .collect()
// }

fn part_one(blueprints: &Vec<Blueprint>, state: State) -> u16 {
    let mut result = 0;
    for blueprint in blueprints {
        let geodes_earned = blueprint.evaluate(state);
        let level = blueprint.id * geodes_earned;
        result += level;
    }
    result
}

fn main() {
    let blueprints = vec![
        Blueprint {
            id: 1,
            ore_robot_cost: (4, 0, 0, 0),
            clay_robot_cost: (2, 0, 0, 0),
            obsidian_robot_cost: (3, 14, 0, 0),
            geode_robot_cost: (2, 0, 7, 0),
        },
        // Blueprint {
        //     id: 2,
        //     ore_robot_cost: (2, 0, 0, 0),
        //     clay_robot_cost: (3, 0, 0, 0),
        //     obsidian_robot_cost: (3, 8, 0, 0),
        //     geode_robot_cost: (3, 0, 12, 0),
        // },
    ];

    let result = part_one(
        &blueprints,
        State {
            time: 24,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        },
    );
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{Blueprint, State};

    #[test]
    fn blueprint1_9geode() {
        let blueprint = Blueprint {
            id: 1,
            ore_robot_cost: (4, 0, 0, 0),
            clay_robot_cost: (2, 0, 0, 0),
            obsidian_robot_cost: (3, 14, 0, 0),
            geode_robot_cost: (2, 0, 7, 0),
        };
        let result = blueprint.evaluate(State {
            time: 24,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        });
        assert_eq!(result, 9);
    }
    #[test]
    fn blueprint2_12geode() {
        let blueprint = Blueprint {
            id: 2,
            ore_robot_cost: (2, 0, 0, 0),
            clay_robot_cost: (3, 0, 0, 0),
            obsidian_robot_cost: (3, 8, 0, 0),
            geode_robot_cost: (3, 0, 12, 0),
        };
        let result = blueprint.evaluate(State {
            time: 24,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        });
        assert_eq!(result, 12);
    }
}
