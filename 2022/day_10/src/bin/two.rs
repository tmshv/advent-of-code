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

fn get_pixel(cursor: i32, sprite: i32) -> String {
    if cursor >= sprite - 1 && cursor <= sprite + 1 {
        return String::from("#");
    }
    String::from(".")
}

fn main() {
    let commands = read_input();
    let mut register: i32 = 1;
    let mut cycles: Vec<i32> = vec![];

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

    let mut i: usize = 0;
    for _ in 0..6 {
        for cursor in 0..40 {
            let sprite = cycles[i];
            let pixel = get_pixel(cursor, sprite);
            print!("{}", pixel);

            i += 1;
        }
        println!("");
    }
}
