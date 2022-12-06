use std::io;

fn split_at_center(s: &String) -> (String, String) {
    let len = s.len();
    let mid = len / 2;

    let a = s[..mid].to_string();
    let b = s[mid..].to_string();

    (a, b)
}

fn common_chars(s1: String, s2: String) -> String {
    let mut common = String::new();

    for c in s1.chars() {
        if s2.contains(c) {
            common.push(c);
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
    for line in io::stdin().lines() {
        let items = line.unwrap();
        let (a, b) = split_at_center(&items);
        let common = common_chars(a, b);
        let x = common.chars().next().unwrap();

        println!("{} priority={}", x, sum_char_numbers(&String::from(x)));
        intersection.push(x);
    }
    let total = sum_char_numbers(&intersection);
    println!("Total priority = {}", total);
}
