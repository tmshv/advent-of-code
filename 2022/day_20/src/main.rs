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
    // let length = input.len() as i32;
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
                // let shift = j as i32 + value;
                // dest = if shift < 0 {
                //     // if length + shift == 0 {
                //     //     ((length + shift - 2) % length) as usize
                //     // } else {
                //     // }
                //     ((length + shift - 1) % length) as usize
                // } else {
                //     (shift % length) as usize
                // };
                // dest = dest;
            }
        }

        println!("mix {}: {} -> {}", value, start, dest);

        print!("{:?}", mix);

        swap(&mut mix, start, dest);
        // mix.remove(start);
        // mix.insert(dest, *value);
        // mix.swap(start, dest);

        print!(" -> {:?}", mix);
        println!("");

        // input.
        // 1, 2, -3, 3, -2, 0, 4,
        // 0, 1,  2, 3,  4, 5, 6,
    }
    mix
}

fn swap(items: &mut Vec<i32>, start: usize, dest: isize) {
    let len = items.len() as isize;
    let mut dest = if dest < 0 {
        (len + dest - 1) as usize
    } else if dest > len - 1 {
        (dest % len + 1) as usize
    } else {
        dest as usize
    };

    if dest == 0 {
        dest = items.len() - 1;
    }

    let value = items[start];
    items.remove(start);
    items.insert(dest, value);
}

fn part_one(input: Vec<i32>) -> i32 {
    let mut mixed = mix(input);
    0
}

fn main() {
    let items = read_input();

    let result = part_one(items);
    println!("Part one: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{mix, swap};

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
}
