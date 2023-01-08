use std::io;

fn read_input() -> Vec<i32> {
    io::stdin()
        .lines()
        .map(|line| match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => value.as_str().parse::<i32>().unwrap(),
        })
        .collect()
}

fn mix(input: Vec<i32>) -> Vec<i32> {
    let mut mix = input.clone();

    // order of mixing is based on INPUT
    for (_, value) in input.iter().enumerate() {
        // 0 has zero effect in mixing
        if *value == 0 {
            continue;
        }

        // find a current value in mixed list
        let mut start = 0;
        let mut dest = 0;
        for (j, value_in_mixed) in mix.iter().enumerate() {
            if value == value_in_mixed {
                start = j;
                dest = (j as i32 + value) as isize;
                break;
            }
        }

        swap(&mut mix, start, dest);

        // input.
        // 1, 2, -3, 3, -2, 0, 4,
        // 0, 1,  2, 3,  4, 5, 6,
    }
    mix
}

fn swap(items: &mut Vec<i32>, start: usize, dest: isize) {
    let mut dest = if dest < 0 {
        (isize::MAX + dest - 1) as usize
    } else {
        dest as usize
    };

    // i dont know why -1 but it works
    dest = dest % (items.len() - 1);

    // i dont know why but 0 destination should move at tne end
    if dest == 0 {
        dest = items.len() - 1;
    }

    let value = items[start];
    items.remove(start);
    items.insert(dest, value);
}

fn value_at(items: &Vec<i32>, start: usize, index: usize) -> i32 {
    let i = (start + index) % items.len();
    items[i]
}

fn part_one(input: Vec<i32>) -> i32 {
    let mixed = mix(input);

    // find zero index
    let mut zero = 0;
    for (i, value) in mixed.iter().enumerate() {
        if *value == 0 {
            zero = i;
            break;
        }
    }

    // find groove coordinate at 1000th, 2000th, 3000th
    let x = value_at(&mixed, zero, 1000);
    let y = value_at(&mixed, zero, 2000);
    let z = value_at(&mixed, zero, 3000);

    x + y + z
}

fn main() {
    let items = read_input();

    let result = part_one(items);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{mix, swap, value_at};

    #[test]
    fn vec_insert_at_index() {
        let mut xs = vec![1, 2, -3, 3, -2, 0, 4];
        xs.insert(3, 100);
        assert_eq!(xs, vec![1, 2, -3, 100, 3, -2, 0, 4]);
    }

    #[test]
    fn vec_insert_at_index_and_remove() {
        let start = 3;
        let dest = 5;
        //                          s      d
        let mut xs = vec![1, 2, -3, 3, -2, 0, 4];
        let value = xs[start];

        xs.insert(dest, value);
        assert_eq!(xs, vec![1, 2, -3, 3, -2, 3, 0, 4]);

        xs.remove(start);
        assert_eq!(xs, vec![1, 2, -3, -2, 3, 0, 4]);
    }

    #[test]
    fn swap_positive() {
        let start = 0;
        let dest = 1;
        let mut xs = vec![1, 2, -3, 3, -2, 0, 4];
        swap(&mut xs, start, dest);
        assert_eq!(xs, vec![2, 1, -3, 3, -2, 0, 4]);

        let start = 2;
        let dest = 5;
        let mut xs = vec![1, 2, 3, -2, -3, 0, 4];
        swap(&mut xs, start, dest);
        assert_eq!(xs, vec![1, 2, -2, -3, 0, 3, 4]);
    }

    #[test]
    fn swap_negative() {
        let mut xs = vec![1, 2, -3, 3, -2, 0, 4];
        swap(&mut xs, 4, 0);
        assert_eq!(xs, vec![1, 2, -3, 3, 0, 4, -2]);
    }

    #[test]
    fn swap_positive_with_module() {
        let mut xs = vec![1, 2, -3, 0, 3, 4, -2];

        // 4 moves between -3 and 0:
        swap(&mut xs, 5, 9); // add 4 to its index
        assert_eq!(xs, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn swap_negative_with_module() {
        // -3 moves between -2 and 0:
        let mut xs = vec![1, -3, 2, 3, -2, 0, 4];
        swap(&mut xs, 1, -2); // -2 = 1 + -3
        assert_eq!(xs, vec![1, 2, 3, -2, -3, 0, 4]);

        // -2 moves between 4 and 1:
        let mut xs = vec![1, 2, -2, -3, 0, 3, 4];
        swap(&mut xs, 2, 0); // 4 = 2 + -2
        assert_eq!(xs, vec![1, 2, -3, 0, 3, 4, -2]);
    }

    #[test]
    fn swap_test_data() {
        // Initial arrangement:
        let mut xs = vec![1, 2, -3, 3, -2, 0, 4];

        // 1 moves between 2 and -3:
        swap(&mut xs, 0, 1);
        assert_eq!(xs, vec![2, 1, -3, 3, -2, 0, 4]);

        // 2 moves between -3 and 3:
        swap(&mut xs, 0, 2);
        assert_eq!(xs, vec![1, -3, 2, 3, -2, 0, 4]);

        // -3 moves between -2 and 0:
        swap(&mut xs, 1, 4);
        assert_eq!(xs, vec![1, 2, 3, -2, -3, 0, 4]);

        // 3 moves between 0 and 4:
        swap(&mut xs, 2, 5);
        assert_eq!(xs, vec![1, 2, -2, -3, 0, 3, 4]);

        // -2 moves between 4 and 1:
        swap(&mut xs, 2, 6);
        assert_eq!(xs, vec![1, 2, -3, 0, 3, 4, -2]);

        // 0 does not move:
        swap(&mut xs, 3, 3);
        assert_eq!(xs, vec![1, 2, -3, 0, 3, 4, -2]);

        // 4 moves between -3 and 0:
        swap(&mut xs, 5, 3);
        assert_eq!(xs, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn mix_test_data() {
        assert_eq!(
            mix(vec![1, 2, -3, 3, -2, 0, 4]),
            vec![1, 2, -3, 4, 0, 3, -2]
        );
    }

    #[test]
    fn value_at_1000() {
        let xs = vec![1, 2, -3, 4, 0, 3, -2];
        assert_eq!(value_at(&xs, 4, 1000), 4);
        assert_eq!(value_at(&xs, 4, 2000), -3);
        assert_eq!(value_at(&xs, 4, 3000), 2);
    }
}
