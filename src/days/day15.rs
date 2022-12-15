use std::collections::HashSet;

use super::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn solve_part_1(input: String) -> String {
        part_1_v2(input, 2_000_000).to_string()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

fn part_1(input: String, at_y: i64) -> usize {
    let sensors: Vec<Sensor> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut seen: HashSet<i64> = HashSet::new();
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

fn part_1_v2(input: String, at_y: i64) -> usize {
    let sensors: Vec<Sensor> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut r = MultiRange::new();
    for sensor in sensors.iter() {
        let dist = sensor.pos.distance(&sensor.closest_beacon);
        let dy = (at_y - sensor.pos.1).abs();
        if dy > dist {
            continue;
        }
        let dx = (dist - dy).abs();
        r.add(Range {
            start: sensor.pos.0 - dx,
            end: sensor.pos.0 + dx,
        });
    }
    for beacon in sensors.iter().map(|s| &s.closest_beacon) {
        if beacon.1 == at_y {
            r.subtract(Range {
                start: beacon.0,
                end: beacon.0,
            });
        }
    }
    r.len()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl Range {
    fn len(&self) -> usize {
        (self.end - self.start) as usize + 1
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end + 1 && self.end + 1 >= other.start
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn is_valid(&self) -> bool {
        self.start <= self.end
    }

    fn subtract(&self, other: &Self) -> Vec<Range> {
        let mut new_vec = if !self.overlaps(&other) {
            vec![self.clone()]
        } else {
            if self.contains(&other) {
                vec![
                    Range {
                        start: self.start,
                        end: other.start - 1,
                    },
                    Range {
                        start: other.end + 1,
                        end: self.end,
                    },
                ]
            } else if other.contains(&self) {
                vec![]
            } else if self.start <= other.start {
                vec![Range {
                    start: self.start,
                    end: other.start - 1,
                }]
            } else {
                vec![Range {
                    start: other.end + 1,
                    end: self.end,
                }]
            }
        };
        new_vec.drain(..).filter(|r| r.is_valid()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MultiRange {
    ranges: Vec<Range>,
}

impl MultiRange {
    fn new() -> Self {
        Self { ranges: vec![] }
    }
    fn from(range: Range) -> Self {
        Self {
            ranges: vec![range],
        }
    }

    fn len(&self) -> usize {
        self.ranges.iter().map(|r| r.len()).sum()
    }

    fn add(&mut self, other: Range) {
        let loc = binary_search(&self.ranges, &other);
        self.ranges.insert(loc, other);
        let first = Range {
            start: self.ranges[0].start,
            end: self.ranges[0].start,
        };
        let mut prev = first;
        let mut new_ranges = vec![];
        for range in self.ranges.iter() {
            if prev.overlaps(&range) {
                prev.end = std::cmp::max(prev.end, range.end);
            } else {
                new_ranges.push(prev);
                prev = *range;
            }
        }
        new_ranges.push(prev);
        self.ranges = new_ranges;
    }

    fn subtract(&mut self, other: Range) {
        let loc = binary_search(&self.ranges, &other);
        self.ranges = self
            .ranges
            .drain(..)
            .flat_map(|r| r.subtract(&other))
            .collect();
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point(i64, i64);

impl Point {
    fn distance(&self, other: &Point) -> i64 {
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
        fn parse_eq(eq: &str, last: bool) -> i64 {
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

fn binary_search<T: Ord>(list: &[T], target: &T) -> usize {
    let mut low = 0;
    let mut high = list.len();
    while low < high {
        let mid = (low + high) / 2;
        if list[mid] < *target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
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
    fn test_binary_search() {
        let list: [usize; 4] = [0, 3, 6, 9];
        assert_eq!(binary_search(&list, &2), 1);
        assert_eq!(binary_search(&list, &3), 1);
    }

    #[test]
    fn test_binary_search_with_range() {
        let list: [Range; 3] = [
            Range { start: 0, end: 3 },
            Range { start: 5, end: 8 },
            Range { start: 10, end: 13 },
        ];
        assert_eq!(binary_search(&list, &Range { start: 2, end: 6 }), 1);
        assert_eq!(binary_search(&list, &Range { start: 4, end: 6 }), 1);
        assert_eq!(binary_search(&[], &0), 0);
    }

    #[test]
    fn test_ranges_add() {
        let ranges: MultiRange = MultiRange {
            ranges: vec![Range { start: 1, end: 3 }, Range { start: 8, end: 10 }],
        };

        let mut r = ranges.clone();
        r.add(Range { start: 0, end: 2 });
        assert_eq!(
            r,
            MultiRange {
                ranges: vec![Range { start: 0, end: 3 }, Range { start: 8, end: 10 }]
            }
        );

        let mut r = ranges.clone();
        r.add(Range { start: 4, end: 7 });
        assert_eq!(
            r,
            MultiRange {
                ranges: vec![Range { start: 1, end: 10 }]
            }
        );
        let mut r = ranges.clone();
        r.add(Range { start: 5, end: 6 });
        assert_eq!(
            r,
            MultiRange {
                ranges: vec![
                    Range { start: 1, end: 3 },
                    Range { start: 5, end: 6 },
                    Range { start: 8, end: 10 },
                ]
            }
        );

        let mut r = MultiRange::new();
        r.add(Range { start: 0, end: 1 });
        assert_eq!(
            r,
            MultiRange {
                ranges: vec![Range { start: 0, end: 1 }]
            }
        );
    }

    #[test]
    fn test_range_subtract() {
        let range = Range { start: 10, end: 20 };
        let r = range.clone();
        let r = r.subtract(&Range { start: 0, end: 11 });
        assert_eq!(r, vec![Range { start: 12, end: 20 }]);
        let r = range.clone();
        let r = r.subtract(&Range { start: 0, end: 10 });
        assert_eq!(r, vec![Range { start: 11, end: 20 }]);
        let r = range.clone();
        let r = r.subtract(&Range { start: 0, end: 1 });
        assert_eq!(r, vec![range]);
        let r = range.clone();
        let r = r.subtract(&Range { start: 12, end: 15 });
        assert_eq!(
            r,
            vec![Range { start: 10, end: 11 }, Range { start: 16, end: 20 },]
        );
    }

    #[test]
    fn test_multi_range_subtract() {
        let mut ranges: MultiRange = MultiRange::new();
        ranges.add(Range { start: 0, end: 10 });
        ranges.add(Range { start: 20, end: 30 });

        let mut r = ranges.clone();
        r.subtract(Range { start: 3, end: 5 });
        assert_eq!(
            r,
            MultiRange {
                ranges: vec![
                    Range { start: 0, end: 2 },
                    Range { start: 6, end: 10 },
                    Range { start: 20, end: 30 }
                ]
            }
        );

        let mut r = ranges.clone();
        r.subtract(Range {
            start: -1,
            end: 101,
        });
        assert_eq!(r, MultiRange::new());
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = part_1_v2(input, 10).to_string();
        let expected = String::from("26");
        assert_eq!(ans, expected)
    }
}
