use super::Solution;
use std::collections::BinaryHeap;

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
        let calories = Self::parse_input(Self::read_input(input_file_name));
        let mut sorted_calories = BinaryHeap::from(calories);
        let mut ans = 0;
        for _ in 0..3 {
            ans += sorted_calories.pop().unwrap();
        }
        ans.to_string()
    }
}
