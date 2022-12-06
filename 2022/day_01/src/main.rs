use std::io;

fn main() -> io::Result<()> {
    let mut max = 0;
    let mut buffer = 0;

    for line in std::io::stdin().lines() {
        match line {
            Ok(result) => {
                if !result.is_empty() {
                    // collect buffer
                    let x = result.parse::<i32>().unwrap();
                    buffer += x;
                } else {
                    // we got new max value
                    if buffer > max {
                        max = buffer;
                    }

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

    println!("Max value is {}", max);

    Ok(())
}
