use std::{
    collections::{HashMap, HashSet},
    io,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Vector(isize, isize);

impl Vector {
    fn add(&self, other: &Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1)
    }

    fn adjacents(&self) -> [Vector; 8] {
        let mut adjacents = ADJACENTS.clone();
        for i in 0..adjacents.len() {
            let a = adjacents[i];
            adjacents[i] = self.add(&a);
        }
        adjacents
    }
}

const N_MOVE: Vector = Vector(0, -1);
const S_MOVE: Vector = Vector(0, 1);
const W_MOVE: Vector = Vector(-1, 0);
const E_MOVE: Vector = Vector(1, 0);

const NE_MOVE: Vector = Vector(1, -1);
const NW_MOVE: Vector = Vector(-1, -1);
const SE_MOVE: Vector = Vector(1, 1);
const SW_MOVE: Vector = Vector(-1, 1);

const ADJACENTS: [Vector; 8] = [
    NE_MOVE, N_MOVE, NW_MOVE, W_MOVE, SW_MOVE, S_MOVE, SE_MOVE, E_MOVE,
];
const START_ORDER: [(Vector, (Vector, Vector, Vector)); 4] = [
    (N_MOVE, (NW_MOVE, N_MOVE, NE_MOVE)),
    (S_MOVE, (SW_MOVE, S_MOVE, SE_MOVE)),
    (W_MOVE, (NW_MOVE, W_MOVE, SW_MOVE)),
    (E_MOVE, (NE_MOVE, E_MOVE, SE_MOVE)),
];

enum Adjacent {
    All([Vector; 8]),
    Line((Vector, Vector, Vector)),
}

#[derive(Debug, Clone)]
struct Squad {
    elves: HashSet<Vector>,
    order: [(Vector, (Vector, Vector, Vector)); 4],
}

impl Squad {
    fn new() -> Squad {
        Squad {
            elves: HashSet::new(),
            order: START_ORDER.clone(),
        }
    }

    fn rotate_order(&mut self) {
        let first = self.order[0];
        for i in 1..self.order.len() {
            self.order[i - 1] = self.order[i];
        }
        let last = self.order.len() - 1;
        self.order[last] = first;
    }

    fn count_adjacents(&self, a: Adjacent) -> usize {
        // let mut elves: HashMap<Vector, &Elf> = HashMap::new();
        // for elf in &self.elves {
        //     elves.insert(elf.position, elf);
        // }

        match a {
            Adjacent::All(positions) => {
                let mut count = 0;
                for pos in positions {
                    if let Some(_) = self.elves.get(&pos) {
                        count += 1
                    }
                }
                count
            }
            Adjacent::Line((a, b, c)) => {
                let positions = [a, b, c];
                let mut count = 0;
                for pos in positions {
                    if let Some(_) = self.elves.get(&pos) {
                        count += 1
                    }
                }
                count
            }
        }
    }

    fn add_elf(&mut self, position: Vector) {
        self.elves.insert(position);
    }

    fn occupied(&self, position: Vector) -> bool {
        match self.elves.get(&position) {
            Some(_) => true,
            None => false,
        }
    }

    fn bounds(&self) -> (isize, isize, isize, isize) {
        let n = self.elves.iter().map(|elf| elf.1).min().unwrap();
        let s = self.elves.iter().map(|elf| elf.1).max().unwrap();
        let w = self.elves.iter().map(|elf| elf.0).min().unwrap();
        let e = self.elves.iter().map(|elf| elf.0).max().unwrap();
        (n, s, w, e)
    }
}

fn print_squad(squad: &Squad) {
    let (n, s, w, e) = squad.bounds();
    for y in n..=s {
        for x in w..=e {
            let position = Vector(x, y);
            let c = match squad.occupied(position) {
                true => '#',
                false => '.',
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn run_round(squad: &mut Squad) -> usize {
    let mut moves = 0;

    // Start of first part
    let mut proposes: HashMap<Vector, Vector> = HashMap::new();
    for elf in &squad.elves {
        let count = squad.count_adjacents(Adjacent::All(elf.adjacents()));

        // Do nothing with the elf if he is alone
        if count == 0 {
            continue;
        }

        // Find proposed move for the elf
        for (step, line) in squad.order {
            let line = (elf.add(&line.0), elf.add(&line.1), elf.add(&line.2));
            let count = squad.count_adjacents(Adjacent::Line(line));
            if count == 0 {
                let propose = elf.add(&step);
                proposes.insert(*elf, propose);
                break;
            }
        }
    }

    // Count how many elves are going to occupy propose
    let mut propose_counts: HashMap<Vector, usize> = HashMap::new();
    for (_, propose) in &proposes {
        if let Some(count) = propose_counts.get(&propose) {
            propose_counts.insert(*propose, count + 1);
        } else {
            propose_counts.insert(*propose, 1);
        }
    }

    // Second part
    let mut new_elves: HashSet<Vector> = HashSet::new();
    for elf in &mut squad.elves.iter() {
        if let Some(propose) = proposes.get(&elf) {
            if let Some(count) = propose_counts.get(&propose) {
                if *count == 1 {
                    new_elves.insert(*propose);
                    moves += 1;
                } else {
                    new_elves.insert(*elf);
                }
            }
        } else {
            new_elves.insert(*elf);
        }
    }

    squad.elves = new_elves;
    squad.rotate_order();

    moves
}

fn part_one(mut squad: Squad, debug: bool) -> usize {
    if debug {
        println!("== Initial State ==");
        print_squad(&squad);
        println!("");
    }

    for round in 1..=10 {
        run_round(&mut squad);

        if debug {
            println!("== End of Round {} ==", round);
            print_squad(&squad);
            println!("");
        }
    }

    let (n, s, w, e) = squad.bounds();
    let mut result = 0;
    for y in n..=s {
        for x in w..=e {
            let cell = Vector(x, y);
            if !squad.occupied(cell) {
                result += 1;
            }
        }
    }
    result
}

fn part_two(mut squad: Squad) -> usize {
    let mut rounds = 0;
    loop {
        let moves = run_round(&mut squad);
        rounds += 1;
        if moves == 0 {
            break;
        }
    }
    rounds
}

fn read_input() -> Squad {
    let mut squad = Squad::new();
    for (y, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        for (x, cell) in line.chars().enumerate() {
            match cell {
                '#' => {
                    let pos = Vector(x as isize, y as isize);
                    squad.add_elf(pos);
                }
                _ => (),
            };
        }
    }
    squad
}

fn main() {
    let squad = read_input();
    let result = part_one(squad.clone(), false);
    println!("Part one: {}", result);

    let result = part_two(squad.clone());
    println!("Part two: {}", result);
}
