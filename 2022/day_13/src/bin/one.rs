use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::Debug,
    io,
};

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Integer(u32),
    List(Vec<Item>),
}

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

impl Pair {
    fn compare(&self) -> bool {
        self.left <= self.right
    }
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
    result
}

fn main() {
    let pairs = read_input();
    let mut count = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.compare() {
            println!("Good {}", i + 1);
            count += i + 1;
        }
    }

    println!("{:?}", count);
}

#[cfg(test)]
mod tests {
    use crate::{parse_row, Item, Pair};

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

    #[test]
    fn pair_compare_equal_integer_list_of_same_size() {
        let pair = Pair {
            left: Item::List(vec![Item::Integer(1), Item::Integer(2), Item::Integer(3)]),
            right: Item::List(vec![Item::Integer(1), Item::Integer(2), Item::Integer(3)]),
        };
        let result = pair.compare();
        assert_eq!(result, true);
    }

    #[test]
    fn pair_compare_right_ordered_integer_list_of_same_size() {
        let pair = Pair {
            left: Item::List(vec![
                Item::Integer(1),
                Item::Integer(1),
                Item::Integer(3),
                Item::Integer(1),
                Item::Integer(1),
            ]),
            right: Item::List(vec![
                Item::Integer(1),
                Item::Integer(1),
                Item::Integer(5),
                Item::Integer(1),
                Item::Integer(1),
            ]),
        };
        let result = pair.compare();
        assert_eq!(result, true);
    }

    #[test]
    fn pair_compare_wrong_ordered_integer_list_of_same_size() {
        let pair = Pair {
            left: Item::List(vec![
                Item::Integer(1),
                Item::Integer(8),
                Item::Integer(3),
                Item::Integer(1),
                Item::Integer(1),
            ]),
            right: Item::List(vec![
                Item::Integer(1),
                Item::Integer(1),
                Item::Integer(5),
                Item::Integer(1),
                Item::Integer(1),
            ]),
        };
        let result = pair.compare();
        assert_eq!(result, false);
    }

    #[test]
    fn pair_compare_right_complex() {
        // [[4,4],4,4]
        // [[4,4],4,4,4]
        let pair = Pair {
            left: Item::List(vec![
                Item::List(vec![Item::Integer(4), Item::Integer(4)]),
                Item::Integer(4),
                Item::Integer(4),
            ]),
            right: Item::List(vec![
                Item::List(vec![Item::Integer(4), Item::Integer(4)]),
                Item::Integer(4),
                Item::Integer(4),
                Item::Integer(4),
            ]),
        };
        let result = pair.compare();
        assert_eq!(result, true);
    }
}
