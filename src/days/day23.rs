use std::collections::HashSet;

use super::Solution;

pub struct Day23;

impl Solution for Day23 {
    fn solve_part_1(input: String) -> String {
        let mut map = Solver::from(&input);
        println!("{:?}", map);
        String::new()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn nth(n: usize) -> Self {
        match n % 4 {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

#[derive(Debug)]
struct Solver {
    direction_index: usize,
    elves: HashSet<Pos>,
}

impl Solver {
    fn from(input: &str) -> Self {
        let elves: HashSet<Pos> = input
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(c, ch)| match ch {
                        '#' => Some(Pos(r as i32, c as i32)),
                        '.' => None,
                        _ => unreachable!(),
                    })
            })
            .collect();
        Self {
            direction_index: 0,
            elves,
        }
    }

    fn bbox(&self) -> (Pos, Pos) {
        let min_x = self.elves.iter().map(|Pos(x, _)| x).min().unwrap();
        let max_x = self.elves.iter().map(|Pos(x, _)| x).max().unwrap();
        let min_y = self.elves.iter().map(|Pos(_, y)| y).min().unwrap();
        let max_y = self.elves.iter().map(|Pos(_, y)| y).max().unwrap();
        (Pos(*min_x, *min_y), Pos(*max_x, *max_y))
    }

    fn ans(&self) -> usize {
        let (top_left, bottom_right) = self.bbox();
        let area = (bottom_right.0 - top_left.0 + 1) * (bottom_right.1 - top_left.1 + 1);
        let area = area as usize;
        area - self.elves.len()
    }
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
        )
    }

    fn get_smaller_input() -> String {
        String::from(
            ".....
..##.
..#..
.....
..##.
.....",
        )
    }

    #[test]
    fn part1() {
        let input = get_sample_input();
        let ans = Day23::solve_part_1(input);
        let expected = "110";
        assert_eq!(ans, expected);
    }

    fn get_final_result() -> String {
        String::from(
            "......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..",
        )
    }

    #[test]
    fn part1_answer() {
        let input = get_final_result();
        let map = Solver::from(&input);
        let ans = map.ans();
        assert_eq!(ans, 110)
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day23::solve_part_2(input);
        let expected = "";
        assert_eq!(ans, expected);
    }
}
