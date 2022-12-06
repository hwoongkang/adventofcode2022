use super::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn solve_part_1(input: String) -> String {
        first_buffer(&input).to_string()
    }
    fn solve_part_2(input: String) -> String {
        first_message(&input).to_string()
    }
}

fn first_nonduplicate(input: &str, length: usize) -> usize {
    let mut seen = [0usize; 26];
    let indices: Vec<usize> = input
        .chars()
        .map(|c| (c as usize) - ('a' as usize))
        .collect();
    for i in 0..length {
        seen[indices[i]] += 1;
    }
    for i in length..indices.len() {
        if seen.iter().filter(|&&x| x == 1).count() == length {
            return i;
        }
        seen[indices[i - length]] -= 1;
        seen[indices[i]] += 1;
    }
    0
}

fn first_buffer(input: &str) -> usize {
    first_nonduplicate(input, 4)
}

fn first_message(input: &str) -> usize {
    first_nonduplicate(input, 14)
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(first_buffer("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(first_buffer("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(first_buffer("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(first_buffer("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(first_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(first_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(first_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(first_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(first_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
