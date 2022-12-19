use super::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn solve_part_1(input: String) -> String {
        String::new()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from("")
    }

    #[test]
    fn part1() {
        let input = get_sample_input();
        let ans = Day18::solve_part_1(input);
        let expected = "";
        assert_eq!(ans, expected);
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day18::solve_part_2(input);
        let expected = "";
        assert_eq!(ans, expected);
    }
}
