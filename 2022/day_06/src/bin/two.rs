use std::{collections::HashSet, io};

const SIZE_OF_SIGNAL_START: usize = 14;

fn is_buffer_unique(items: &Vec<char>) -> bool {
    let size = items.len();
    let mut s = HashSet::<&char>::new();
    for c in items {
        s.insert(&c);
    }

    s.len() == size
}

fn read_signal(value: String) -> u32 {
    let mut s = Vec::<char>::new();
    for (i, c) in value.chars().enumerate() {
        s.push(c);

        if s.len() > SIZE_OF_SIGNAL_START {
            s.remove(0);
        }

        if s.len() == SIZE_OF_SIGNAL_START && is_buffer_unique(&s) {
            return i as u32 + 1;
        }
    }

    return 0;
}

fn main() {
    for x in io::stdin().lines() {
        let line = x.unwrap();
        let scanned = read_signal(line);
        println!("Scanned {} characters", scanned);
    }
}
