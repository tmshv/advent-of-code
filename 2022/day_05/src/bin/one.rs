extern crate regex;

use regex::Regex;
use std::{collections::HashMap, io};

#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn top(&self) -> Option<&T> {
        self.items.last()
    }
}

fn read_head() -> HashMap<u32, Stack<String>> {
    let mut stacks = HashMap::<u32, Stack<String>>::new();
    let mut head: Vec<String> = Vec::new();

    // read strin until empty row appear
    for x in io::stdin().lines() {
        let line = x.unwrap();
        if line.is_empty() {
            break;
        }
        head.push(line);
    }

    // take last row cause it has ids of stacks
    let ns = head.pop().unwrap();

    // reverse stack definition rows to fill stacks in right direction
    head.reverse();

    // fill stacks
    for (i, c) in ns.chars().enumerate() {
        if !c.is_numeric() {
            continue;
        }

        let n = c.to_digit(10).unwrap();
        let mut stack = Stack::new();
        for line in &head {
            let x = line.chars().nth(i).unwrap();
            if x == ' ' {
                continue;
            }

            let payload = String::from(x);
            stack.push(payload);
        }

        stacks.insert(n,  stack);
    }
    stacks
}

fn read_action(value: &String) -> (u32, u32, u32) {
    let rg = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let caps = rg.captures(value).unwrap();

    let amount = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let from = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let to = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();

    (amount, from, to)
}

fn get_mut_pair<'a, K, V>(hash: &'a mut HashMap<K, V>, a: &K, b: &K) -> (&'a mut V, &'a mut V)
where
    K: Eq + std::hash::Hash,
{
    unsafe {
        let a = hash.get_mut(a).unwrap() as *mut _;
        let b = hash.get_mut(b).unwrap() as *mut _;
        assert_ne!(a, b, "The two keys must not resolve to the same value");
        (&mut *a, &mut *b)
    }
}

fn main() {
    let head = &mut read_head();
    println!("{:?}", head);

    for x in io::stdin().lines() {
        let line = x.unwrap();
        let (amount, from, to) = read_action(&line);

        let (a, b) = get_mut_pair::<u32, Stack<String>>(head, &from, &to);

        for _ in 0..amount {
            let payload = a.pop().unwrap();
            b.push(payload);
        }
    }

    println!("{:?}", head);

    let mut tops: Vec<_> = head.keys().collect();
    tops.sort();
    let out: Vec<_> = tops.into_iter().map(|key| head.get(key).unwrap().top().unwrap().clone()).collect();
    println!("{}", out.join(""));
}
