use super::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn solve_part_1(input: String) -> String {
        String::new()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from("")
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = Day22::solve_part_1(input);
        let expected = "";
        assert_eq!(ans, expected)
    }

    #[test]
    fn test_part_2() {
        let input = get_sample_input();
        let ans = Day22::solve_part_2(input);
        let expected = "";
        assert_eq!(ans, expected)
    }
}
