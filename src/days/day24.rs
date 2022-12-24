use std::collections::VecDeque;

use super::Solution;

pub struct Day24;

impl Solution for Day24 {
    fn solve_part_1(input: String) -> String {
        let map = Map::from(&input);
        map.solve().to_string()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
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

    fn solve(&self) -> usize {
        let mut ans = 0;

        let mut max_dist = 0;

        let mut states: Vec<State> = vec![self.state.clone()];

        let mut queue: VecDeque<(usize, Pos)> = VecDeque::new();

        queue.push_back((0, self.start));

        while let Some((dist, pos)) = queue.pop_front() {
            if dist > max_dist {
                max_dist = dist;
                println!("iterating... {}", dist);
            }
            if pos == self.end {
                return dist;
            }
            let next_dist = dist + 1;
            while next_dist >= states.len() {
                let last_state = states.last().unwrap();
                states.push(last_state.next_state());
            }
            for np in self.next_positions(pos, &states[next_dist]) {
                queue.push_back((next_dist, np));
            }
        }
        ans
    }

    fn next_positions(&self, pos: Pos, state: &State) -> Vec<Pos> {
        let (rows, cols) = self.state.get_size();

        [(0, 0), (1, 0), (0, 1), (rows - 1, 0), (0, cols - 1)]
            .iter()
            .map(|(dr, dc)| ((pos.0 + dr) % rows, (pos.1 + dc) % cols))
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

#[cfg(test)]
mod day24_tests {
    use super::*;

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
    fn state_from_string(input: String) -> State {
        let map = Map::from(&input);
        let state = map.state;
        state
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

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = Day24::solve_part_1(input);
        let expected = "18";
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_part_2() {
        let input = get_sample_input();
        let ans = Day24::solve_part_2(input);
        let expected = "";
        assert_eq!(ans, expected);
    }
}
