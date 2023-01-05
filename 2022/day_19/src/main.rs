use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::Debug,
    io,
    ops::{Add, Sub},
    str::FromStr,
};

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

#[derive(Debug, Clone)]
struct Blueprint {
    id: u16,

    ore_robot_cost: (u16, u16, u16, u16),
    clay_robot_cost: (u16, u16, u16, u16),
    obsidian_robot_cost: (u16, u16, u16, u16),
    geode_robot_cost: (u16, u16, u16, u16),
}

impl Blueprint {
    fn evalulate(&self, state: &State, minute: u16) -> u16 {
        0
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
    0
}

fn main() {
    let blueprints = vec![Blueprint {
        id: 2,
        ore_robot_cost: (2, 0, 0, 0),
        clay_robot_cost: (3, 0, 0, 0),
        obsidian_robot_cost: (3, 8, 0, 0),
        geode_robot_cost: (3, 0, 12, 0),
    }];

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
}
