use super::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn solve_part_1(input: String) -> String {
        String::new()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    fn get_test_input() -> String {
        String::from("")
    }

    #[test]
    fn test_part_1() {
        let input = get_test_input();
        let ans = Day09::solve_part_1(input);
        assert_eq!(ans, "".to_string());
    }
    #[test]
    fn test_part_2() {
        let input = get_test_input();
        let ans = Day09::solve_part_2(input);
        assert_eq!(ans, "".to_string());
    }
}
