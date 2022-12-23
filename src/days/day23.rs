use std::collections::{HashMap, HashSet};

use super::Solution;

pub struct Day23;

impl Solution for Day23 {
    fn solve_part_1(input: String) -> String {
        let mut map = Solver::from(&input);
        for _ in 0..10 {
            map.tick();
        }
        map.ans().to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut map = Solver::from(&input);
        let mut ans = 1;
        while map.tick() > 0 {
            ans += 1;
        }
        ans.to_string()
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

    fn first_half(&mut self) -> HashMap<Pos, Vec<Pos>> {
        let mut candidates: HashMap<Pos, Vec<Pos>> = HashMap::new();

        for elf in self.elves.iter() {
            if self.alone(&elf) {
                candidates.entry(*elf).or_default().push(*elf);
                continue;
            }
            for i in 0..4 {
                let index = self.direction_index + i;
                let dir = Direction::nth(index);
                if let Some(pos) = self.next_pos(elf, &dir) {
                    candidates.entry(pos).or_default().push(*elf);
                    break;
                }

                // staying still
            }
        }

        self.direction_index += 1;
        candidates
    }

    fn tick(&mut self) -> usize {
        let candidates = self.first_half();

        let mut moved = 0;

        for (next_pos, elves_want_to_move) in candidates.iter() {
            if elves_want_to_move.len() > 1 {
                continue;
            }
            let elf = elves_want_to_move[0];
            if &elf != next_pos {
                moved += 1;
            }
            self.elves.remove(&elf);
            self.elves.insert(*next_pos);
        }
        moved
    }

    fn alone(&self, elf: &Pos) -> bool {
        for i in 0..4 {
            let dir = Direction::nth(i);
            let Some(_) = self.next_pos(elf, &dir) else {
				return false;
			};
        }
        true
    }

    fn next_pos(&self, elf: &Pos, direction: &Direction) -> Option<Pos> {
        match direction {
            Direction::North => {
                let Pos(x, y) = elf;
                let north = Pos(x - 1, *y);
                let notrh_east = Pos(x - 1, y + 1);
                let notrh_west = Pos(x - 1, y - 1);
                let exists = self.elves.contains(&north)
                    || self.elves.contains(&notrh_east)
                    || self.elves.contains(&notrh_west);
                if !exists {
                    Some(north)
                } else {
                    None
                }
            }
            Direction::South => {
                let Pos(x, y) = elf;
                let south = Pos(x + 1, *y);
                let south_east = Pos(x + 1, y + 1);
                let south_west = Pos(x + 1, y - 1);
                let exists = self.elves.contains(&south)
                    || self.elves.contains(&south_east)
                    || self.elves.contains(&south_west);
                if !exists {
                    Some(south)
                } else {
                    None
                }
            }
            Direction::West => {
                let Pos(x, y) = elf;
                let west = Pos(*x, y - 1);
                let west_north = Pos(x - 1, y - 1);
                let west_south = Pos(x + 1, y - 1);
                let exists = self.elves.contains(&west)
                    || self.elves.contains(&west_north)
                    || self.elves.contains(&west_south);
                if !exists {
                    Some(west)
                } else {
                    None
                }
            }
            Direction::East => {
                let Pos(x, y) = elf;
                let east = Pos(*x, y + 1);
                let east_north = Pos(x - 1, y + 1);
                let east_south = Pos(x + 1, y + 1);

                let exists = self.elves.contains(&east)
                    || self.elves.contains(&east_north)
                    || self.elves.contains(&east_south);
                if !exists {
                    Some(east)
                } else {
                    None
                }
            }
        }
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

    fn get_smaller_input() -> String {
        String::from(
            ".....
..##.
..#..
.....
..##.
.....
",
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
    fn part1_first_tick() {
        let input = get_smaller_input();
        let mut map = Solver::from(&input);

        let after = String::from(
            "..##.
.....
..#..
...#.
..#..
.....
",
        );

        let after_tick = Solver::from(&after);

        map.tick();

        assert_eq!(map.elves, after_tick.elves)
    }

    #[test]
    fn part1_second_tick() {
        let input = get_smaller_input();
        let mut map = Solver::from(&input);

        map.tick();
        map.tick();

        let after = String::from(
            ".....
..##.
.#...
....#
.....
..#..",
        );

        let after_tick = Solver::from(&after);

        assert_eq!(map.elves, after_tick.elves)
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day23::solve_part_2(input);
        let expected = "20";
        assert_eq!(ans, expected);
    }
}
