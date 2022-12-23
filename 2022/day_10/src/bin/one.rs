use std::io;

#[derive(Debug)]
enum Cmd {
    Noop,
    Addx,
}

#[derive(Debug)]
struct Command {
    cmd: Cmd,
    value: Option<i32>,
}

fn read_input() -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];
    for x in io::stdin().lines() {
        match x {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => match value.as_str() {
                "noop" => {
                    commands.push(Command {
                        cmd: Cmd::Noop,
                        value: None,
                    });
                }
                other => {
                    let s: Vec<&str> = other.split(" ").collect();
                    let value = s[1].parse::<i32>().unwrap();
                    commands.push(Command {
                        cmd: Cmd::Addx,
                        value: Some(value),
                    });
                }
            },
        }
    }
    commands
}

fn main() {
    let commands = read_input();
    let mut register: i32 = 1;
    let mut cycles: Vec<i32> = vec![];
    cycles.push(register); // add 0 cycle value

    for command in &commands {
        match command.cmd {
            Cmd::Noop => {
                cycles.push(register);
            }
            Cmd::Addx => {
                let value = command.value.unwrap();

                cycles.push(register);
                cycles.push(register);

                register += value;
            }
        }
    }

    let interesting_signal_strengths: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let result: i32 = interesting_signal_strengths.iter().map(|cycle| *cycle as i32 * cycles[*cycle]).sum();

    println!("Result: {}", result);
}
