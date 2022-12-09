use super::Solution;

use std::collections::HashSet;

pub struct Day09;

impl Solution for Day09 {
    fn solve_part_1(input: String) -> String {
        let mut rope = Rope::new(2);
        for line in input.lines() {
            rope.exec_line(line);
        }
        rope.tail_trace.len().to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut rope = Rope::new(10);
        for line in input.lines() {
            rope.exec_line(line);
        }
        rope.tail_trace.len().to_string()
    }
}

enum Command {
    Right,
    Up,
    Left,
    Down,
}

impl std::str::FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Command::Right),
            "U" => Ok(Command::Up),
            "L" => Ok(Command::Left),
            "D" => Ok(Command::Down),
            _ => Err(()),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Knot(i32, i32);

impl Knot {
    fn exec(&mut self, command: &Command) {
        match command {
            Command::Right => self.0 += 1,
            Command::Up => self.1 += 1,
            Command::Left => self.0 -= 1,
            Command::Down => self.1 -= 1,
        }
    }

    fn follow(&mut self, head: &Knot) {
        let dx = head.0 - self.0;
        let dy = head.1 - self.1;
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        } else {
            self.0 += dx.signum();
            self.1 += dy.signum();
        }
    }
}

struct Rope {
    knots: Vec<Knot>,
    tail_trace: HashSet<Knot>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            knots: (0..length).map(|_| Knot(0, 0)).collect(),
            tail_trace: HashSet::from_iter([Knot(0, 0)]),
        }
    }

    fn exec_line(&mut self, input: &str) {
        let (command, dist) = parse_line(input);
        for _ in 0..dist {
            self.exec(&command);
            self.follow();
        }
    }

    fn exec(&mut self, command: &Command) {
        self.knots[0].exec(command);
    }

    fn follow(&mut self) {
        let length = self.knots.len();
        for i in 1..length {
            let front = self.knots[i - 1];
            let back = &mut self.knots[i];
            back.follow(&front);
        }
        self.tail_trace.insert(self.knots[length - 1]);
    }
}

fn parse_line(line: &str) -> (Command, usize) {
    let mut words = line.split_whitespace();
    let command = words.next().unwrap().parse().unwrap();
    let distance = words.next().unwrap().parse().unwrap();
    (command, distance)
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    fn get_test_input() -> String {
        String::from(
            "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2",
        )
    }

    fn part_2_test_input() -> String {
        String::from(
            "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20",
        )
    }

    #[test]
    fn test_part_1() {
        let input = get_test_input();
        let ans = Day09::solve_part_1(input);
        assert_eq!(ans, "13".to_string());
    }
    #[test]
    fn test_part_2() {
        let input = get_test_input();
        let ans = Day09::solve_part_2(input);
        assert_eq!(ans, "1".to_string());
        let input_2 = part_2_test_input();
        let ans_2 = Day09::solve_part_2(input_2);
        assert_eq!(ans_2, "36".to_string());
    }
}
