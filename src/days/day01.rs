use super::Solution;

pub struct Day01 {}

impl Day01 {
    fn parse_input(input: String) -> Vec<i32> {
        let mut ans = vec![];
        let mut prev = 0;
        for line in input.lines() {
            if let Ok(num) = line.parse::<i32>() {
                prev += num;
            } else {
                ans.push(prev);
                prev = 0;
            }
        }
        ans
    }
}

impl Solution for Day01 {
    fn solve_part_1(input_file_name: &str) -> String {
        let nums = Self::parse_input(Self::read_input(input_file_name));
        nums.iter().max().unwrap().to_string()
    }
    fn solve_part_2(input_file_name: &str) -> String {
        let mut calories = Self::parse_input(Self::read_input(input_file_name));
        calories.sort();
        calories.iter().rev().take(3).sum::<i32>().to_string()
    }
}
