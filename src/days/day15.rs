use std::collections::HashSet;

use super::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn solve_part_1(input: String) -> String {
        part_1(input, 2_000_000).to_string()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

fn part_1(input: String, at_y: i32) -> usize {
    let sensors: Vec<Sensor> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut seen: HashSet<i32> = HashSet::new();
    for sensor in sensors.iter() {
        let dist = sensor.pos.distance(&sensor.closest_beacon);
        let dy = (at_y - sensor.pos.1).abs();
        if dy > dist {
            continue;
        }
        seen.insert(sensor.pos.0);
        for dx in 1..(dist - dy + 1) {
            seen.insert(sensor.pos.0 + dx);
            seen.insert(sensor.pos.0 - dx);
        }
    }
    for sensor in sensors.iter() {
        let beacon = &sensor.closest_beacon;
        if beacon.1 == at_y {
            seen.remove(&beacon.0);
        }
    }
    seen.len()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

impl std::str::FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        fn parse_eq(eq: &str, last: bool) -> i32 {
            if last {
                eq[2..eq.len()].parse().unwrap()
            } else {
                eq[2..eq.len() - 1].parse().unwrap()
            }
        }
        let x = parse_eq(words[2], false);
        let y = parse_eq(words[3], false);
        let pos = Point(x, y);
        let x = parse_eq(words[8], false);
        let y = parse_eq(words[9], true);
        let closest_beacon = Point(x, y);
        Ok(Sensor {
            pos,
            closest_beacon,
        })
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        )
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = part_1(input, 10).to_string();
        let expected = String::from("26");
        assert_eq!(ans, expected)
    }

    #[test]
    fn range() {
        assert_eq!((0..0).len(), 0)
    }
}
