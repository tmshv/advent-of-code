use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::{Debug, Display},
    io,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Integer(u32),
    List(Vec<Item>),
}

struct Items(pub Vec<Item>);

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Integer(a), Item::Integer(b)) => a.cmp(b),
            (Item::Integer(value), _) => {
                let list = Item::List(vec![Item::Integer(*value)]);
                list.cmp(other)
            }
            (_, Item::Integer(value)) => {
                let list = Item::List(vec![Item::Integer(*value)]);
                self.cmp(&list)
            }
            (Item::List(left_items), Item::List(right_items)) => {
                let mut a = left_items.iter();
                let mut b = right_items.iter();
                loop {
                    match (a.next(), b.next()) {
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => continue,
                            other => return other,
                        },
                        // If the right list runs out of items first, the inputs are not in the right order
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Integer(value) => write!(f, "{}", value),
            Item::List(list) => {
                if list.len() == 0 {
                    write!(f, "[]")
                } else {
                    write!(f, "{:?}", list)
                }
            }
        }
    }
}

fn parse_row(row: &str) -> Item {
    let mut integer_buffer = vec![];
    let mut stack = vec![];

    for (_, c) in row.chars().enumerate() {
        match c {
            '[' => {
                stack.push(Item::List(Vec::new()));
            }
            ']' => {
                // insert integer
                let current_list = stack.last_mut().unwrap();
                match current_list {
                    Item::Integer(_) => {}
                    Item::List(list) => {
                        let val = String::from_iter(integer_buffer);
                        if val.len() > 0 {
                            let num = val.parse::<u32>().unwrap();
                            list.push(Item::Integer(num));
                        }
                    }
                }
                integer_buffer = vec![];

                // do routine with list
                let last = stack.pop().unwrap();

                if stack.len() == 0 {
                    return last;
                } else {
                    let current_list = stack.last_mut().unwrap();
                    match current_list {
                        Item::Integer(_) => {}
                        Item::List(list) => {
                            list.push(last);
                        }
                    }
                }
            }
            ',' => {
                // insert integer
                let current_list = stack.last_mut().unwrap();
                match current_list {
                    Item::Integer(_) => {}
                    Item::List(list) => {
                        let val = String::from_iter(integer_buffer);
                        if val.len() > 0 {
                            let num = val.parse::<u32>().unwrap();
                            list.push(Item::Integer(num));
                        }
                    }
                }
                integer_buffer = vec![];
            }
            c => {
                integer_buffer.push(c);
            }
        }
    }
    Item::List(vec![])
    // Item::List(result)
}

#[derive(Debug)]
struct Pair {
    left: Item,
    right: Item,
}

fn read_input() -> Vec<Pair> {
    let mut result = vec![];
    let mut buffer: Vec<String> = vec![];
    for line in io::stdin().lines() {
        match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => {
                if value.is_empty() {
                    let left = parse_row(buffer[0].as_str());
                    let right = parse_row(buffer[1].as_str());
                    result.push(Pair { left, right });
                    buffer.clear();
                } else {
                    buffer.push(value);
                }
            }
        }
    }
    let left = parse_row(buffer[0].as_str());
    let right = parse_row(buffer[1].as_str());
    result.push(Pair { left, right });
    result
}

fn main() {
    let pairs = read_input();
    let mut items = vec![];

    for pair in pairs {
        items.push(pair.left);
        items.push(pair.right);
    }

    let d1 = Item::List(vec![Item::List(vec![Item::Integer(2)])]);
    let d2 = Item::List(vec![Item::List(vec![Item::Integer(6)])]);

    let dividers = vec![d1.clone(), d2.clone()];

    items.push(d1);
    items.push(d2);

    items.sort();

    let mut signal = 1;
    for (i, item) in items.iter().enumerate() {
        if dividers.contains(item) {
            signal *= i + 1;
        }
        println!("{}", item);
    }
    println!("Result: {}", signal);
}

#[cfg(test)]
mod tests {
    use crate::{parse_row, Item};

    #[test]
    fn parse_empty_list() {
        let result = parse_row("[]");
        assert_eq!(result, Item::List(vec![]));
    }

    #[test]
    fn parse_simple_list() {
        let result = parse_row("[1,2,3,4,5]");
        assert_eq!(
            result,
            Item::List(vec![
                Item::Integer(1),
                Item::Integer(2),
                Item::Integer(3),
                Item::Integer(4),
                Item::Integer(5),
            ])
        );
    }

    #[test]
    fn parse_nested_list() {
        let result = parse_row("[1,[],[[2],[3,4]],5]");
        assert_eq!(
            result,
            Item::List(vec![
                Item::Integer(1),
                Item::List(vec![]),
                Item::List(vec![
                    Item::List(vec![Item::Integer(2),]),
                    Item::List(vec![Item::Integer(3), Item::Integer(4)]),
                ]),
                Item::Integer(5),
            ])
        );
    }
}
