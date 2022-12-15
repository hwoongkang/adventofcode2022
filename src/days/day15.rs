use super::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn solve_part_1(input: String) -> String {
        String::new()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from("")
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = Day15::solve_part_1(input);
        let expected = String::from("");
        assert_eq!(ans, expected)
    }
}
