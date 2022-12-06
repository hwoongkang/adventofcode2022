use super::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn solve_part_1(input: String) -> String {
        first_buffer(&input).to_string()
    }
    fn solve_part_2(input: String) -> String {
        unimplemented!("")
    }
}

fn first_buffer(input: &str) -> usize {
    let mut seen: [i32; 26] = [0; 26];
    let indices: Vec<usize> = input.chars().map(|c| c as usize - 'a' as usize).collect();
    for i in 0..4 {
        seen[indices[i]] += 1;
    }
    for i in 4..indices.len() {
        if seen.iter().filter(|&&x| x == 1).count() == 4 {
            return i;
        }
        seen[indices[i]] += 1;
        seen[indices[i - 4]] -= 1;
    }
    0
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
        assert_eq!(Day06::solve_part_2("".to_string()), "".to_string());
    }
}
