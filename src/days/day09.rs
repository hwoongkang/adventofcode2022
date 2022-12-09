use super::Solution;

use std::collections::HashSet;

pub struct Day09;

impl Solution for Day09 {
    fn solve_part_1(input: String) -> String {
        let mut head = Position(0, 0);
        let mut tail = Position(0, 0);
        let mut visited: HashSet<Position> = HashSet::new();
        visited.insert(tail);
        for line in input.lines() {
            let (command, dist) = parse_line(line);
            for _ in 0..dist {
                head.exec(&command);
                tail.follow(&head);
                visited.insert(tail);
            }
        }
        visited.len().to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut rope = Rope::new();
        for line in input.lines() {
            let (command, dist) = parse_line(line);
            for _ in 0..dist {
                rope.exec(&command);
                rope.follow();
            }
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
struct Position(i32, i32);

impl Position {
    fn exec(&mut self, command: &Command) {
        match command {
            Command::Right => self.0 += 1,
            Command::Up => self.1 += 1,
            Command::Left => self.0 -= 1,
            Command::Down => self.1 -= 1,
        }
    }

    fn follow(&mut self, head: &Position) {
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
    knots: Vec<Position>,
    tail_trace: HashSet<Position>,
}

impl Rope {
    fn new() -> Self {
        Self {
            knots: (0..10).map(|_| Position(0, 0)).collect(),
            tail_trace: HashSet::from_iter([Position(0, 0)]),
        }
    }

    fn exec(&mut self, command: &Command) {
        self.knots[0].exec(command);
    }

    fn follow(&mut self) {
        for i in 1..10 {
            let front = self.knots[i - 1];
            let back = &mut self.knots[i];
            back.follow(&front);
        }
        self.tail_trace.insert(self.knots[9]);
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
