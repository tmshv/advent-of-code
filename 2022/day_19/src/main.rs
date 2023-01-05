use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::Debug,
    io,
    ops::{Add, Sub},
    str::FromStr,
};

const LAST_MINUTE: u16 = 24;

#[derive(Debug, Copy, Clone)]
struct State {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
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
    fn evaluate(&self, state: State, minute: u16) -> u16 {
        // end of recursion
        // at the last minute just return total earned geode
        if minute == LAST_MINUTE {
            // if state.geode > 5 || state.geode_robots > 2 {
            //     println!("stop of good variant at M{} {:?}", minute, state);
            // }
            return state.geode + state.geode_robots;
        }

        // see what robots can be factored according to resources
        // with amount of resources in the state at the begining of the minute
        let mut next_states = Vec::new();
        for (robot, can_build) in self.robots_to_build(&state).iter().enumerate() {
            if *can_build {
                let mut next_state = state.clone();
                let (robots, resources) = match robot {
                    0 => ((1, 0, 0, 0), self.ore_robot_cost),
                    1 => ((0, 1, 0, 0), self.clay_robot_cost),
                    2 => ((0, 0, 1, 0), self.obsidian_robot_cost),
                    3 => ((0, 0, 0, 1), self.geode_robot_cost),
                    _ => {
                        panic!("unreachable");
                    }
                };

                // + robots
                next_state.ore_robots += robots.0;
                next_state.clay_robots += robots.1;
                next_state.obsidian_robots += robots.2;
                next_state.geode_robots += robots.3;

                // - resources
                next_state.ore -= resources.0;
                next_state.clay -= resources.1;
                next_state.obsidian -= resources.2;
                next_state.geode -= resources.3;

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
        }

        // evaluate new states starting from current amount of geode earned
        let mut max_geodes = state.geode;
        // println!("M{} {:?}", minute, state);
        for s in next_states {
            let geodes = self.evaluate(s, minute + 1);
            if geodes > max_geodes {
                // println!("improve at M{} {:?}", minute, state);
                max_geodes = geodes;
            }
        }
        max_geodes
    }

    fn robots_to_build(&self, state: &State) -> [bool; 4] {
        [
            self.enough_for_ore_robot(state),
            self.enough_for_clay_robot(state),
            self.enough_for_obsidian_robot(state),
            self.enough_for_geode_robot(state),
        ]
    }

    fn enough_for_ore_robot(&self, state: &State) -> bool {
        let (ore, clay, obsidian, geode) = self.ore_robot_cost;
        state.ore >= ore && state.clay >= clay && state.obsidian >= obsidian && state.geode >= geode
    }

    fn enough_for_clay_robot(&self, state: &State) -> bool {
        let (ore, clay, obsidian, geode) = self.clay_robot_cost;
        state.ore >= ore && state.clay >= clay && state.obsidian >= obsidian && state.geode >= geode
    }

    fn enough_for_obsidian_robot(&self, state: &State) -> bool {
        let (ore, clay, obsidian, geode) = self.obsidian_robot_cost;
        state.ore >= ore && state.clay >= clay && state.obsidian >= obsidian && state.geode >= geode
    }

    fn enough_for_geode_robot(&self, state: &State) -> bool {
        let (ore, clay, obsidian, geode) = self.geode_robot_cost;
        state.ore >= ore && state.clay >= clay && state.obsidian >= obsidian && state.geode >= geode
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
        let geodes_earned = blueprint.evaluate(state, 1);
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
            geode_robot_cost: (7, 0, 2, 0),
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
            geode_robot_cost: (7, 0, 2, 0),
        };
        let result = blueprint.evaluate(
            State {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            },
            1,
        );
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
        let result = blueprint.evaluate(
            State {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            },
            1,
        );
        assert_eq!(result, 12);
    }
}
