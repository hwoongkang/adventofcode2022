use super::Solution;

pub struct Day25;

impl Solution for Day25 {
    fn solve_part_1(input: String) -> String {
        String::new()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day25_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from("")
    }

    #[test]
    fn part1() {
        let input = get_sample_input();
        let ans = Day25::solve_part_1(input);
        let expected = "";
        assert_eq!(ans, expected);
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day25::solve_part_2(input);
        let expected = "";
        assert_eq!(ans, expected);
    }
}
