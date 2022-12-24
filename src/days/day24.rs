use std::collections::VecDeque;

use super::Solution;

pub struct Day24;

impl Solution for Day24 {
    fn solve_part_1(input: String) -> String {
        let map = Map::from(&input);
        map.part_1().to_string()
    }
    fn solve_part_2(input: String) -> String {
        let map = Map::from(&input);
        map.part_2().to_string()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Blizzard {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty(Vec<Blizzard>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State(Vec<Vec<Tile>>);

type Pos = (usize, usize);

impl State {
    fn get_blizzards(&self) -> Vec<(Pos, Blizzard)> {
        let mut blizzards: Vec<((usize, usize), Blizzard)> = vec![];

        for (r, line) in self.0.iter().enumerate() {
            for (c, tile) in line.iter().enumerate() {
                match tile {
                    Tile::Wall => {}
                    Tile::Empty(v) => {
                        for blizzard in v.iter() {
                            blizzards.push(((r, c), *blizzard));
                        }
                    }
                }
            }
        }
        blizzards
    }

    fn get_size(&self) -> Pos {
        let rows = self.0.len();
        let cols = self.0[0].len();
        (rows, cols)
    }
    fn next_state(&self) -> State {
        let (rows, cols) = self.get_size();
        let mut new_map: Vec<Vec<Tile>> = (0..rows)
            .map(|r| {
                (0..cols)
                    .map(|c| match self.0[r][c] {
                        Tile::Wall => Tile::Wall,
                        Tile::Empty(_) => Tile::Empty(vec![]),
                    })
                    .collect()
            })
            .collect();

        let blizzards = self.get_blizzards();

        for (pos, bliz) in blizzards.iter() {
            let next_pos = self.next_pos(bliz, pos);
            match &mut new_map[next_pos.0][next_pos.1] {
                Tile::Wall => {
                    panic!("should not be here")
                }
                Tile::Empty(v) => v.push(*bliz),
            }
        }

        State(new_map)
    }

    fn next_pos(&self, blizzard: &Blizzard, pos: &Pos) -> Pos {
        let (rows, cols) = self.get_size();
        let dp: Pos = match blizzard {
            Blizzard::Up => (rows - 1, 0),
            Blizzard::Down => (1, 0),
            Blizzard::Left => (0, cols - 1),
            Blizzard::Right => (0, 1),
        };

        let mut p = *pos;

        loop {
            p = self.add(&p, &dp);
            match self.0[p.0][p.1] {
                Tile::Empty(_) => break p,
                Tile::Wall => continue,
            }
        }
    }

    fn add(&self, pos: &Pos, other: &Pos) -> Pos {
        let (rows, cols) = self.get_size();
        ((pos.0 + other.0) % rows, (pos.1 + other.1) % cols)
    }
}

#[derive(Debug)]
struct Map {
    state: State,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn from(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let state: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| match ch {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty(vec![]),
                        '>' => Tile::Empty(vec![Blizzard::Right]),
                        '<' => Tile::Empty(vec![Blizzard::Left]),
                        '^' => Tile::Empty(vec![Blizzard::Up]),
                        'v' => Tile::Empty(vec![Blizzard::Down]),
                        'S' => {
                            start = (r, c);
                            Tile::Empty(vec![])
                        }
                        'E' => {
                            end = (r, c);
                            Tile::Empty(vec![])
                        }
                        _ => panic!("Unknown tile: {}", ch),
                    })
                    .collect()
            })
            .collect();
        let state = State(state);
        Self { state, start, end }
    }

    fn shortest_distance(&self, from: Pos, to: Pos, starting_at: usize) -> usize {
        let mut space: Vec<State> = vec![self.state.clone()];

        let (rows, cols) = self.state.get_size();

        let lcm = lcm(rows - 2, cols - 2);

        while space.len() < lcm {
            let last = space.last().unwrap();
            space.push(last.next_state());
        }

        let mut visited = vec![vec![vec![false; cols]; rows]; lcm];

        let mut queue: VecDeque<(usize, Pos)> = VecDeque::new();

        queue.push_back((starting_at, from));

        visited[starting_at % lcm][from.0][from.1] = true;

        while let Some((dist, pos)) = queue.pop_front() {
            if pos == to {
                return dist;
            }

            let next_dist = dist + 1;

            for next_pos in self.next_positions(pos, &space[next_dist % lcm]) {
                if !visited[next_dist % lcm][next_pos.0][next_pos.1] {
                    visited[next_dist % lcm][next_pos.0][next_pos.1] = true;
                    queue.push_back((next_dist, next_pos));
                }
            }
        }

        0
    }

    fn part_1(&self) -> usize {
        self.shortest_distance(self.start, self.end, 0)
    }

    fn part_2(&self) -> usize {
        let start_to_end = self.shortest_distance(self.start, self.end, 0);

        let end_to_start = self.shortest_distance(self.end, self.start, start_to_end);

        let return_to_end = self.shortest_distance(self.start, self.end, end_to_start);

        return_to_end
    }

    fn next_positions(&self, pos: Pos, state: &State) -> Vec<Pos> {
        let (rows, cols) = self.state.get_size();

        [(0, 0), (1, 0), (0, 1), (rows - 1, 0), (0, cols - 1)]
            .iter()
            .map(|(dr, dc)| ((pos.0 + dr) % rows, (pos.1 + dc) % cols))
            .map(|pos| pos)
            .filter_map(|(r, c)| match &state.0[r][c] {
                Tile::Wall => None,
                Tile::Empty(v) => match v.len() {
                    0 => Some((r, c)),
                    _ => None,
                },
            })
            .collect()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let (a, b) = if a > b { (b, a) } else { (a, b) };
    if b % a == 0 {
        a
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    fn state_from_string(input: String) -> State {
        let map = Map::from(&input);
        let state = map.state;
        state
    }

    #[test]
    fn test_state_lcm() {
        let input = get_sample_input();

        let state = state_from_string(input);

        let (rows, cols) = state.get_size();

        let mut s = state.clone();

        for _ in 0..lcm(rows - 2, cols - 2) {
            s = s.next_state();
        }

        assert_eq!(s, state);
    }

    #[test]
    fn test_state_tick() {
        let input = String::from(
            "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#",
        );

        let state = state_from_string(input);

        let state = state.next_state();

        let input = String::from(
            "#.#####
#.....#
#.>...#
#.....#
#.....#
#...v.#
#####.#",
        );

        let tick_1 = state_from_string(input);

        assert_eq!(state, tick_1);

        let state = state.next_state();

        let tick_2 = state_from_string(String::from(
            "#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#",
        ));

        assert_eq!(state, tick_2);
    }

    fn get_sample_input() -> String {
        String::from(
            "#S######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######E#",
        )
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = Day24::solve_part_1(input);
        let expected = "18";
        assert_eq!(ans, expected);
    }
    #[test]
    fn test_part_1_minute_2() {
        let input = get_sample_input();
        let state = state_from_string(input);

        let state = state.next_state();
        let state = state.next_state();

        assert_eq!(state.0[2][1], Tile::Empty(vec![]));
    }

    #[test]
    fn test_part_2() {
        let input = get_sample_input();
        let ans = Day24::solve_part_2(input);
        let expected = "54";
        assert_eq!(ans, expected);
    }
}
