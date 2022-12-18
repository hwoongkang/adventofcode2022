use super::Solution;

pub struct Day16;

impl Solution for Day16 {
    fn solve_part_1(input: String) -> String {
        String::new()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from("")
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = Day16::solve_part_1(input);
        let expected = "";
        assert_eq!(ans, expected)
    }
}
