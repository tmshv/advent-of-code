use std::io;

fn common_chars(a: &String, b: &String, c: &String) -> String {
    let mut common = String::new();

    for x in a.chars() {
        if b.contains(x) && c.contains(x) {
            common.push(x);
        }
    }

    common
}

const LOWERCASE_A: i32 = 'a' as i32;
const UPPERCASE_A: i32 = 'A' as i32;
const LOWERCASE_OFFSET: i32 = LOWERCASE_A - 1;
const UPPERCASE_OFFSET: i32 = UPPERCASE_A - 1 - 26;

fn sum_char_numbers(s: &String) -> i32 {
    let mut sum = 0 as i32;
    for c in s.chars() {
        if c >= 'a' && c <= 'z' {
            sum += c as i32 - LOWERCASE_OFFSET;
        } else if c >= 'A' && c <= 'Z' {
            sum += c as i32 - UPPERCASE_OFFSET;
        }
    }
    sum
}

fn main() {
    let mut intersection = String::new();
    let mut v = vec![String::new(), String::new(), String::new()];
    let mut count = 0;

    for line in io::stdin().lines() {
        if count > 0 && count % 3 == 0 {
            let common = common_chars(&v[0], &v[1], &v[2]);
            let x = common.chars().next().unwrap();
            println!("{} ({})", x, sum_char_numbers(&String::from(x)));
            intersection.push(x);
        }

        let i = count % 3;
        v[i] = line.unwrap();

        count += 1;
    }

    // yea I know this is looks ugly a bit (Im talking about duplication)
    let common = common_chars(&v[0], &v[1], &v[2]);
    let x = common.chars().next().unwrap();
    println!("{} ({})", x, sum_char_numbers(&String::from(x)));
    intersection.push(x);

    let total = sum_char_numbers(&intersection);
    println!("Total priority = {}", total);
}
