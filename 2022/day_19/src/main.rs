use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    io,
    str::FromStr,
};

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    time: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

impl State {
    fn has_time(&self) -> bool {
        self.time > 0
    }

    fn enough_robots(&self, cost: (u8, u8, u8, u8)) -> bool {
        let (ore, clay, obsidian, geode) = cost;
        self.ore_robots >= ore + 1
            && self.clay_robots >= clay + 1
            && self.obsidian_robots >= obsidian + 1
            && self.geode_robots >= geode + 1
    }

    fn enough_resources(&self, cost: (u8, u8, u8, u8)) -> bool {
        let (ore, clay, obsidian, geode) = cost;
        self.ore >= ore && self.clay >= clay && self.obsidian >= obsidian && self.geode >= geode
    }

    fn create_robot(&mut self, robot: (u8, u8, u8, u8), cost: (u8, u8, u8, u8)) {
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

    fn tick(&mut self) {
        if self.time > 0 {
            self.ore += self.ore_robots;
            self.clay += self.clay_robots;
            self.obsidian += self.obsidian_robots;
            self.geode += self.geode_robots;
            self.time -= 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Blueprint {
    id: u8,

    ore_robot_cost: (u8, u8, u8, u8),
    clay_robot_cost: (u8, u8, u8, u8),
    obsidian_robot_cost: (u8, u8, u8, u8),
    geode_robot_cost: (u8, u8, u8, u8),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBlueprintError;

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern =
            Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.")
                .unwrap();
        let cap = pattern.captures(s);
        match cap {
            None => Err(ParseBlueprintError),
            Some(cap) => {
                let id = cap
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| ParseBlueprintError)?;
                let ore_cost = cap
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| ParseBlueprintError)?;
                let clay_cost = cap
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| ParseBlueprintError)?;
                let obsidian_cost_ore = cap
                    .get(4)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| ParseBlueprintError)?;
                let obsidian_cost_clay = cap
                    .get(5)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| ParseBlueprintError)?;
                let geode_cost_ore = cap
                    .get(6)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| ParseBlueprintError)?;
                let geode_cost_obsidian = cap
                    .get(7)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| ParseBlueprintError)?;

                Ok(Blueprint {
                    id,
                    ore_robot_cost: (ore_cost, 0, 0, 0),
                    clay_robot_cost: (clay_cost, 0, 0, 0),
                    obsidian_robot_cost: (obsidian_cost_ore, obsidian_cost_clay, 0, 0),
                    geode_robot_cost: (geode_cost_ore, 0, geode_cost_obsidian, 0),
                })
            }
        }
    }
}

impl Blueprint {
    fn evaluate(&self, state: State) -> u8 {
        let mut seen = HashSet::<State>::with_capacity(200_000_000);
        let mut deq = VecDeque::<State>::with_capacity(200_000_000);
        deq.push_front(state);

        // evaluate new states starting from current amount of geode earned
        let mut max_geodes = state.geode;
        let mut max_at_time = state.time;

        while deq.len() > 0 {
            if deq.len() % 1000000 == 0 {
                println!("Q{}", deq.len());
            }

            let state = deq.pop_front().unwrap();

            // state is already checked
            if seen.contains(&state) {
                continue;
            } else {
                seen.insert(state.clone());
            }

            // state is wasted
            if !state.has_time() {
                continue;
            }

            let geodes = state.geode + state.geode_robots;
            if geodes > max_geodes {
                max_geodes = geodes;
                max_at_time = state.time;
            }

            // skip state if it waste more time than best and earned less geodes
            if state.time > max_at_time && state.geode + state.geode_robots < max_geodes {
                continue;
            }

            // check unique branch where we buy geode robot
            if state.enough_resources(self.geode_robot_cost) {
                let mut next_state = state.clone();
                next_state.tick();
                next_state.create_robot((0, 0, 0, 1), self.geode_robot_cost);
                deq.push_back(next_state);

                // no need to check brances where other robots can be build at this step
                // nor earning resources
                continue;
            }

            // check branch where we buy obsidian robot
            if state.enough_resources(self.obsidian_robot_cost) {
                let mut next_state = state.clone();
                next_state.tick();
                next_state.create_robot((0, 0, 1, 0), self.obsidian_robot_cost);
                deq.push_back(next_state);
            } else {
            }

            // check branch where we buy clay robot
            if state.enough_resources(self.clay_robot_cost)
                && !state.enough_robots(self.clay_robot_cost)
            {
                let mut next_state = state.clone();
                next_state.tick();
                next_state.create_robot((0, 1, 0, 0), self.clay_robot_cost);
                deq.push_back(next_state);
            }

            // check branch where we buy ore robot
            if state.enough_resources(self.ore_robot_cost)
                && !state.enough_robots(self.ore_robot_cost)
            {
                let mut next_state = state.clone();
                next_state.tick();
                next_state.create_robot((1, 0, 0, 0), self.ore_robot_cost);
                deq.push_back(next_state);
            }

            // add current state too
            // as an option if strategy is to accumulate resources
            if state.time > 2 {
                let mut no_robot_state = state.clone();
                no_robot_state.tick();
                deq.push_back(no_robot_state);
            }
        }
        max_geodes
    }
}

fn read_input() -> Vec<Blueprint> {
    io::stdin()
        .lines()
        .map(|line| match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => value.as_str().parse::<Blueprint>().unwrap(),
        })
        .collect()
}

fn part_one(blueprints: &Vec<Blueprint>, state: State) -> u8 {
    blueprints
        .iter()
        .map(|blueprint| {
            let geodes_earned = blueprint.evaluate(state);
            blueprint.id * geodes_earned
        })
        .sum()
}

fn part_two(blueprints: &Vec<Blueprint>, state: State) -> u8 {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.evaluate(state))
        .product()
}

fn main() {
    // let blueprints = read_input();

    // let result = part_one(
    //     &blueprints,
    //     State {
    //         time: 24,
    //         ore: 0,
    //         clay: 0,
    //         obsidian: 0,
    //         geode: 0,
    //         ore_robots: 1,
    //         clay_robots: 0,
    //         obsidian_robots: 0,
    //         geode_robots: 0,
    //     },
    // );
    // println!("Part one: {}", result);

    let result = part_two(
        &vec![Blueprint {
            id: 1,
            ore_robot_cost: (4, 0, 0, 0),
            clay_robot_cost: (2, 0, 0, 0),
            obsidian_robot_cost: (3, 14, 0, 0),
            geode_robot_cost: (2, 0, 7, 0),
        }],
        State {
            time: 32,
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
    println!("Part two: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{Blueprint, State};

    #[test]
    fn blueprint_from_str() {
        let blueprint: Blueprint  = "Blueprint 18: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 12 obsidian.".parse().unwrap();
        assert_eq!(
            blueprint,
            Blueprint {
                id: 18,
                ore_robot_cost: (3, 0, 0, 0),
                clay_robot_cost: (4, 0, 0, 0),
                obsidian_robot_cost: (4, 18, 0, 0),
                geode_robot_cost: (4, 0, 12, 0),
            }
        );
    }

    // #[test]
    // fn blueprint1_9geode() {
    //     let blueprint = Blueprint {
    //         id: 1,
    //         ore_robot_cost: (4, 0, 0, 0),
    //         clay_robot_cost: (2, 0, 0, 0),
    //         obsidian_robot_cost: (3, 14, 0, 0),
    //         geode_robot_cost: (2, 0, 7, 0),
    //     };
    //     let result = blueprint.evaluate(State {
    //         time: 24,
    //         ore: 0,
    //         clay: 0,
    //         obsidian: 0,
    //         geode: 0,
    //         ore_robots: 1,
    //         clay_robots: 0,
    //         obsidian_robots: 0,
    //         geode_robots: 0,
    //     });
    //     assert_eq!(result, 9);
    // }

    #[test]
    fn blueprint1_56geode() {
        let blueprint = Blueprint {
            id: 1,
            ore_robot_cost: (4, 0, 0, 0),
            clay_robot_cost: (2, 0, 0, 0),
            obsidian_robot_cost: (3, 14, 0, 0),
            geode_robot_cost: (2, 0, 7, 0),
        };
        let result = blueprint.evaluate(State {
            time: 32,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        });
        assert_eq!(result, 56);
    }

    // #[test]
    // fn blueprint2_12geode() {
    //     let blueprint = Blueprint {
    //         id: 2,
    //         ore_robot_cost: (2, 0, 0, 0),
    //         clay_robot_cost: (3, 0, 0, 0),
    //         obsidian_robot_cost: (3, 8, 0, 0),
    //         geode_robot_cost: (3, 0, 12, 0),
    //     };
    //     let result = blueprint.evaluate(State {
    //         time: 24,
    //         ore: 0,
    //         clay: 0,
    //         obsidian: 0,
    //         geode: 0,
    //         ore_robots: 1,
    //         clay_robots: 0,
    //         obsidian_robots: 0,
    //         geode_robots: 0,
    //     });
    //     assert_eq!(result, 12);
    // }

    #[test]
    fn state_has_time() {
        let mut state = State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            time: 24,
        };
        assert_eq!(state.has_time(), true);
        let mut trues = [false; 24];
        for (i, _) in (0..24).enumerate() {
            trues[i] = state.has_time();
            state.tick();
        }
        assert_eq!(trues.iter().all(|x| *x), true);
        assert_eq!(state.has_time(), false);
    }

    #[test]
    fn state_create_robot() {
        let mut state = State {
            ore: 7,
            clay: 8,
            obsidian: 14,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            time: 11,
        };
        state.create_robot((0, 0, 0, 1), (2, 0, 7, 0));
        assert_eq!(
            state,
            State {
                ore: 5,
                clay: 8,
                obsidian: 7,
                geode: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 1,
                time: 11,
            }
        );
        state.create_robot((0, 0, 1, 0), (3, 4, 0, 0));
        assert_eq!(
            state,
            State {
                ore: 2,
                clay: 4,
                obsidian: 7,
                geode: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 1,
                geode_robots: 1,
                time: 11,
            }
        );
    }

    #[test]
    fn state_enough_resources() {
        let state = State {
            ore: 7,
            clay: 8,
            obsidian: 14,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            time: 11,
        };
        assert_eq!(state.enough_resources((2, 0, 7, 0)), true);
        assert_eq!(state.enough_resources((2, 9, 7, 0)), false);
    }

    #[test]
    fn state_tick() {
        let mut state = State {
            ore: 7,
            clay: 8,
            obsidian: 14,
            geode: 0,
            ore_robots: 2,
            clay_robots: 4,
            obsidian_robots: 3,
            geode_robots: 1,
            time: 10,
        };
        state.tick();
        assert_eq!(
            state,
            State {
                ore: 9,
                clay: 12,
                obsidian: 17,
                geode: 1,
                ore_robots: 2,
                clay_robots: 4,
                obsidian_robots: 3,
                geode_robots: 1,
                time: 9,
            }
        );
        state.tick();
        assert_eq!(
            state,
            State {
                ore: 11,
                clay: 16,
                obsidian: 20,
                geode: 2,
                ore_robots: 2,
                clay_robots: 4,
                obsidian_robots: 3,
                geode_robots: 1,
                time: 8,
            }
        );
    }
}
