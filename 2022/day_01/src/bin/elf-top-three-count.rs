use std::io;
use std::vec::Vec;

fn main() -> io::Result<()> {
    let mut elfes: Vec<i32> = Vec::new();
    let mut buffer = 0;

    for line in std::io::stdin().lines() {
        match line {
            Ok(result) => {
                if !result.is_empty() {
                    // collect buffer
                    let x = result.parse::<i32>().unwrap();
                    buffer += x;
                } else {
                    elfes.push(buffer);

                    // reset buffer
                    buffer = 0;
                }
            }
            Err(error) => {
                println!("Something went wrong");
                panic!("{}", error);
            }
        }
    }

    if buffer > 0 {
        elfes.push(buffer);
    }

    elfes.sort_by(|a, b| b.cmp(a));
    let total = elfes[0] + elfes[1] + elfes[2];
    println!("Toal value is {}", total);

    Ok(())
}
