use super::Solution;

pub struct Day2021_1 {}

impl Day2021_1 {
    fn count_num_increased(nums: &[u16]) -> usize {
        let mut ans = 0;
        let mut prev = std::u16::MAX;
        for &num in nums {
            if num > prev {
                ans += 1;
            }
            prev = num;
        }
        ans
    }

    fn parse_input(input: String) -> Vec<u16> {
        input
            .lines()
            .map(|line| line.trim().parse().unwrap())
            .collect()
    }
}

impl Solution for Day2021_1 {
    fn solve_part_1(input_file_name: &str) -> String {
        let input = Self::read_input(input_file_name);
        let input = Self::parse_input(input);

        Self::count_num_increased(&input).to_string()
    }
    fn solve_part_2(input_file_name: &str) -> String {
        let input = Self::read_input(input_file_name);
        let input = Self::parse_input(input);
        let input: Vec<u16> = (2..input.len())
            .map(|i| input[i - 2] + input[i - 1] + input[i])
            .collect();
        Self::count_num_increased(&input).to_string()
    }
}
