use regex::Regex;
use std::{
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

fn manhattan(x: &Vector, y: &Vector) -> i32 {
    // Take the sum of the absolute values of the differences of the coordinates.
    // For example, if x=(a,b) and y=(c,d), the Manhattan distance between x and y is
    // |a-c| + |b-d|

    let (a, b) = x.as_tuple();
    let (c, d) = y.as_tuple();

    (a - c).abs() + (b - d).abs()
}

fn find_distress_beacon(sensors: Vec<(Vector, Vector)>, x_max: i32, y_max: i32) -> Option<Vector> {
    for y in 0..y_max {
        let mut x = 0;
        'search: loop {
            x += 1;
            let v = Vector::new(x, y);

            for (s, b) in &sensors {
                let r = manhattan(s, b);
                let dist = manhattan(s, &v);
                if r >= dist {
                    x = s.x + r - (s.y - y).abs();
                    continue 'search; // excluded by sensor
                }
            }
            if x >= x_max {
                break;
            }

            return Some(Vector::new(x, y));
        }
    }
    None
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

    let mut set = vec![];
    for (sensor, beacon) in zip(&sensors, &beacons) {
        set.push((*sensor, *beacon));
    }

    let beacon = find_distress_beacon(set, 4000000, 4000000);
    match beacon {
        None => {
            println!("fail");
        }
        Some(beacon) => {
            let x = beacon.x as i128;
            let y = beacon.y as i128;
            let f = x * 4000000 + y;
            println!("Result: {}", f);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{manhattan, parse_row, Vector};

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
}
