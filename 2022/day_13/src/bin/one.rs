use std::{fmt::Debug, io};

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Integer(u32),
    List(Vec<Item>),
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
    // let x = parse_row("[[],1,[2,3,4],5]");
    println!("{:?}", pairs);

    // parse_row("[]");
    // parse_row("[1, [2, [3, 4]], [], 5]");
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
