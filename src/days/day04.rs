use super::Solution;
use std::str::FromStr;

pub struct Day04 {}

impl Solution for Day04 {
    fn solve_part_1(input_file_name: &str) -> String {
        Self::part_1(Self::read_input(input_file_name))
    }
    fn solve_part_2(input_file_name: &str) -> String {
        Self::part_2(Self::read_input(input_file_name))
    }
}

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.min <= other.max && other.min <= self.max
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Range, ()> {
        let mut parts = s.split('-');
        let min = parts.next().unwrap().parse::<u32>().unwrap();
        let max = parts.next().unwrap().parse::<u32>().unwrap();
        Ok(Range { min, max })
    }
}

impl Day04 {
    fn parse_line(line: &str) -> (Range, Range) {
        let mut elves = line.split(",");
        let first = elves.next().unwrap().parse().unwrap();
        let second = elves.next().unwrap().parse().unwrap();
        (first, second)
    }
    fn part_1(input: String) -> String {
        input
            .lines()
            .map(|line| Self::parse_line(line))
            .filter(|(a, b)| a.contains(b) || b.contains(a))
            .count()
            .to_string()
    }
    fn part_2(input: String) -> String {
        input
            .lines()
            .map(|line| Self::parse_line(line))
            .filter(|(a, b)| a.overlaps(b))
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod day04_test {
    use super::*;

    #[test]
    fn part_1() {
        let sample_input = String::from(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );
        assert_eq!(Day04::part_1(sample_input), "2");
    }

    #[test]
    fn part_2() {
        let sample_input = String::from(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );
        assert_eq!(Day04::part_2(sample_input), "4");
    }
}
