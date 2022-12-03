use super::Solution;
use std::collections::HashSet;

pub struct Day03 {}

impl Solution for Day03 {
    fn solve_part_1(input_file_name: &str) -> String {
        let input = Self::read_input(input_file_name);
        Self::part_1(input)
    }
    fn solve_part_2(input_file_name: &str) -> String {
        let input = Self::read_input(input_file_name);
        Self::part_2(input)
    }
}
impl Day03 {
    fn get_priority(c: char) -> i32 {
        let a = 'a' as i32;
        let z = 'z' as i32;
        let big_a = 'A' as i32;
        let num = c as i32;

        if a <= num && num <= z {
            num - a + 1
        } else {
            num - big_a + 27
        }
    }

    fn get_duplicate(line: &str) -> HashSet<char> {
        let mut duplicates = HashSet::new();
        let l = line.len();
        let first_half: HashSet<char> = line.chars().take(l / 2).collect();

        for ch in line.chars().skip(l / 2) {
            if first_half.contains(&ch) {
                duplicates.insert(ch);
            }
        }

        duplicates
    }
    fn part_1(input: String) -> String {
        let mut ans = 0;
        for line in input.lines() {
            let duplicates = Self::get_duplicate(line);
            for ch in duplicates {
                ans += Self::get_priority(ch);
            }
        }
        ans.to_string()
    }

    fn part_2(input: String) -> String {
        let mut ans = 0;

        let mut set: HashSet<char> = HashSet::new();
        for (index, line) in input.lines().enumerate() {
            let now: HashSet<char> = line.chars().collect();
            if index % 3 == 0 {
                set = now
            } else if index % 3 == 1 {
                set = set.intersection(&now).map(|a| *a).collect();
            } else {
                set = set.intersection(&now).map(|a| *a).collect();
                for ch in set.iter() {
                    ans += Self::get_priority(*ch);
                }
            }
        }

        ans.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(Day03::get_priority('a'), 1);
        assert_eq!(Day03::get_priority('z'), 26);
        assert_eq!(Day03::get_priority('A'), 27);
        assert_eq!(Day03::get_priority('Z'), 52);
    }
    #[test]
    fn test_day03_part_1() {
        let input = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(Day03::part_1(input), "157");
    }
    #[test]
    fn test_day03_part_2() {
        let input = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(Day03::part_2(input), "70");
    }
}
