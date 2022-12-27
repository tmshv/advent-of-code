use regex::Regex;
use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    io,
    iter::zip,
    ops::{Add, Sub},
    vec,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }
    fn min() -> Vector {
        Vector {
            x: std::i32::MIN,
            y: std::i32::MIN,
        }
    }
    fn max() -> Vector {
        Vector {
            x: std::i32::MAX,
            y: std::i32::MAX,
        }
    }
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

fn parse_row(row: String) -> (Vector, Vector) {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15

    let pattern = Regex::new(
        r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)",
    )
    .unwrap();
    let cap = pattern.captures(row.as_str()).unwrap();
    let sensor = Vector {
        x: cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        y: cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
    };
    let beacon = Vector {
        x: cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        y: cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
    };
    (sensor, beacon)
}

fn read_input() -> (Vec<Vector>, Vec<Vector>) {
    let mut sensors = vec![];
    let mut beacons = vec![];
    for line in io::stdin().lines() {
        let (s, b) = match line {
            Err(error) => {
                panic!("{}", error);
            }
            Ok(value) => parse_row(value),
        };
        sensors.push(s);
        beacons.push(b);
    }
    (sensors, beacons)
}

fn get_bbox(items: &Vec<Vector>) -> (Vector, Vector) {
    let mut tl = Vector::max();
    let mut br = Vector::min();
    for v in items {
        if v.x < tl.x {
            tl.x = v.x;
        }
        if v.y < tl.y {
            tl.y = v.y;
        }

        if v.x > br.x {
            br.x = v.x;
        }
        if v.y > br.y {
            br.y = v.y;
        }
    }
    (tl, br)
}

fn manhattan(x: &Vector, y: &Vector) -> i32 {
    // Take the sum of the absolute values of the differences of the coordinates.
    // For example, if x=(a,b) and y=(c,d), the Manhattan distance between x and y is
    // |a-c| + |b-d|

    let (a, b) = x.as_tuple();
    let (c, d) = y.as_tuple();

    (a - c).abs() + (b - d).abs()
}

fn main() {
    let (sensors, beacons) = read_input();

    let mut coords = vec![];
    for v in &sensors {
        coords.push(*v);
    }
    for v in &beacons {
        coords.push(*v);
    }
    let (tl, br) = get_bbox(&coords);

    let max_dist = zip(&sensors, &beacons)
        .map(|(s, b)| manhattan(s, b))
        .max()
        .unwrap();

    let target_line = 2000000;
    let min_x = tl.x - max_dist;
    let max_x = br.x + max_dist;

    let mut set = HashSet::new();
    for (sensor, beacon) in zip(&sensors, &beacons) {
        let target_dist = manhattan(sensor, beacon);
        for x in min_x..=max_x {
            let v = Vector::new(x, target_line);
            let dist = manhattan(sensor, &v);

            if manhattan(beacon, &v) == 0 {
                continue;
            }

            let is_cover = dist <= target_dist;
            if is_cover {
                set.insert(v);
            }
        }
    }

    let count = set.len();
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use crate::{get_bbox, manhattan, parse_row, Vector};

    #[test]
    fn parse_row_from_example() {
        let result = parse_row(String::from(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        ));
        assert_eq!(result, (Vector::new(2, 18), Vector::new(-2, 15)));
    }

    #[test]
    fn manhattan9() {
        let dist = manhattan(&Vector::new(8, 7), &Vector::new(2, 10));
        assert_eq!(dist, 9);
    }

    #[test]
    fn get_bbox_from_example() {
        let items = vec![
            Vector { x: 2, y: 18 },
            Vector { x: 9, y: 16 },
            Vector { x: 13, y: 2 },
            Vector { x: 12, y: 14 },
            Vector { x: 10, y: 20 },
            Vector { x: 14, y: 17 },
            Vector { x: 8, y: 7 },
            Vector { x: 2, y: 0 },
            Vector { x: 0, y: 11 },
            Vector { x: 20, y: 14 },
            Vector { x: 17, y: 20 },
            Vector { x: 16, y: 7 },
            Vector { x: 14, y: 3 },
            Vector { x: 20, y: 1 },
        ];
        let result = get_bbox(&items);
        assert_eq!(result, (Vector { x: 0, y: 0 }, Vector { x: 20, y: 20 }));
    }
}
