use super::Solution;
use std::str::FromStr;

pub struct Day05;

impl Solution for Day05 {
    fn solve_part_1(input: String) -> String {
        let mut stacks = Stacks::new(&[
            "GFVHPS", "GJFBVDZM", "GMLJN", "NGZVDWP", "VRCB", "VRSMPWLZ", "THP", "QRSNCHZV",
            "FLGPVQJ",
        ]);
        Self::part_1(&mut stacks, input)
    }
    fn solve_part_2(input: String) -> String {
        unimplemented!("")
    }
}

impl Day05 {
    fn part_1(stacks: &mut Stacks, input: String) -> String {
        for line in input.lines() {
            let command: Command = line.parse().unwrap();
            stacks.execute(command);
        }
        stacks.get_top_crates()
    }
}

struct Command {
    num_crate_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Command, ()> {
        let iter = &mut s.split_whitespace();
        let num_crate_to_move = iter.skip(1).next().unwrap().parse().unwrap();
        let mut from_stack = iter.skip(1).next().unwrap().parse().unwrap();
        from_stack -= 1;
        let mut to_stack = iter.skip(1).next().unwrap().parse().unwrap();
        to_stack -= 1;
        Ok(Command {
            num_crate_to_move,
            from_stack,
            to_stack,
        })
    }
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new(input: &[&str]) -> Self {
        Self {
            stacks: input.iter().map(|s| s.chars().collect()).collect(),
        }
    }

    fn execute(&mut self, command: Command) {
        for _ in 0..command.num_crate_to_move {
            let crate_to_move = self.stacks[command.from_stack].pop().unwrap();
            self.stacks[command.to_stack].push(crate_to_move);
        }
    }

    fn get_top_crates(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }
}

#[cfg(test)]
mod day05_test {
    use super::*;

    const SAMPLE_INPUT: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_day05_part_1() {
        let mut sample_stacks = Stacks::new(&["ZN", "MCD", "P"]);
        assert_eq!(
            Day05::part_1(&mut sample_stacks, SAMPLE_INPUT.to_string()),
            "CMZ"
        );
    }
    #[test]
    fn test_day05_part_2() {
        assert_eq!(Day05::solve_part_2("".to_string()), "".to_string());
    }
}
