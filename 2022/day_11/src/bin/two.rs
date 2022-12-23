extern crate evalexpr;

use evalexpr::{context_map, eval_with_context_mut, Context, Value};
use regex::Regex;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Item {
    level: i64,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<Item>,
    operation: String,
    test_value: i64,
    throw_true: usize,
    throw_false: usize,
    inspected_items: i64,
}

impl Monkey {
    fn inspect(&mut self, item: &Item) -> Item {
        self.inspected_items += 1;

        let mut context = context_map! {
            "old" => Value::Int(item.level as i64),
        }
        .unwrap(); // Do proper error handling here

        let result = eval_with_context_mut(&self.operation, &mut context);
        match result {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(_) => {
                let n = context.get_value("new").unwrap();
                let level = n.as_int().unwrap();
                Item { level }
            }
        }
    }

    fn test(&self, item: &Item) -> bool {
        item.level % self.test_value == 0
    }
}

fn parse_monkey(def: Vec<String>) -> Monkey {
    // Monkey 0:
    //   Starting items: 71, 86
    //   Operation: new = old * 13
    //   Test: divisible by 19
    //     If true: throw to monkey 6
    //     If false: throw to monkey 7
    let line_1 = Regex::new(r"Monkey (\d+):").unwrap();
    let id = line_1
        .captures(def[0].as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let line_2 = Regex::new(r"Starting items: ([\d\s,]+)").unwrap();
    let items: Vec<Item> = line_2
        .captures(def[1].as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|x| x.parse::<i64>().unwrap())
        .map(|level| Item { level })
        .collect();

    let line_3 = Regex::new(r"Operation: (.*)$").unwrap();
    let operation = line_3
        .captures(def[2].as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let line_4 = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let test_value = line_4
        .captures(def[3].as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap();

    let line_5 = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let throw_true = line_5
        .captures(def[4].as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let line_6 = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
    let throw_false = line_6
        .captures(def[5].as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    Monkey {
        id,
        items,
        operation: String::from(operation),
        test_value,
        throw_true,
        throw_false,
        inspected_items: 0,
    }
}

fn read_input() -> Vec<Monkey> {
    let mut buffer: Vec<String> = vec![];
    let mut monkeys: Vec<Monkey> = vec![];
    for x in io::stdin().lines() {
        match x {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => {
                if value.is_empty() {
                    let m = parse_monkey(buffer);
                    monkeys.push(m);
                    buffer = vec![];
                } else {
                    buffer.push(value);
                }
            }
        }
    }
    if buffer.len() == 6 {
        let m = parse_monkey(buffer);
        monkeys.push(m);
    }
    monkeys
}

fn run_round(monkeys: &mut Vec<Monkey>, lcd: i64) -> &mut Vec<Monkey> {
    for i in 0..monkeys.len() {
        let mut drops: Vec<(usize, Item)> = vec![];
        let monkey = &mut monkeys[i];
        while monkey.items.len() > 0 {
            let item = monkey.items.remove(0);

            // get an item with new worry level after monkey inspect it
            let mut new_item = monkey.inspect(&item);

            // reduce size of worry level
            new_item.level = new_item.level % lcd;

            // define to who monkey will throw it
            let next_id = if monkey.test(&new_item) {
                monkey.throw_true
            } else {
                monkey.throw_false
            };

            // monkey throw the item
            drops.push((next_id as usize, new_item));
        }

        // other monkey catches their drops
        for (id, item) in drops {
            monkeys[id].items.push(item);
        }
    }

    monkeys
}

fn main() {
    let mut monkeys = &mut read_input();
    let lcd: i64 = monkeys.iter().map(|m| m.test_value).product();

    for _ in 0..10000 {
        monkeys = run_round(monkeys, lcd);
    }

    monkeys.sort_by_key(|m| m.inspected_items * -1);
    let m1 = &monkeys[0];
    let m2 = &monkeys[1];
    let result = m1.inspected_items * m2.inspected_items;
    println!("Result: {}", result);
}
