use std::str::FromStr;

use super::Solution;

pub struct Day2021_25 {}

#[derive(PartialEq, Eq)]
enum SeaCucumber {
    Right,
    Down,
    Empty,
}

impl FromStr for SeaCucumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(SeaCucumber::Right),
            "v" => Ok(SeaCucumber::Down),
            "." => Ok(SeaCucumber::Empty),
            _ => Err(()),
        }
    }
}

struct Map {
    map: Vec<Vec<SeaCucumber>>,
    width: usize,
    height: usize,
}

impl Map {
    fn tick(&mut self) -> usize {
        let mut changed = 0;
        let w = self.width;
        let h = self.height;
        let mut can_move: Vec<(usize, usize)> = vec![];
        for r in 0..h {
            for c in 0..w {
                if self.map[r][c] != SeaCucumber::Right {
                    continue;
                }
                let nc = (c + 1) % w;
                if self.map[r][nc] == SeaCucumber::Empty {
                    can_move.push((r, c));
                    changed += 1;
                }
            }
        }

        for &(r, c) in can_move.iter() {
            self.map[r][c] = SeaCucumber::Empty;
            self.map[r][(c + 1) % w] = SeaCucumber::Right;
        }

        let mut can_move: Vec<(usize, usize)> = vec![];

        for r in 0..h {
            for c in 0..w {
                if self.map[r][c] != SeaCucumber::Down {
                    continue;
                }
                let nr = (r + 1) % h;
                if self.map[nr][c] == SeaCucumber::Empty {
                    can_move.push((r, c));
                    changed += 1;
                }
            }
        }

        for &(r, c) in can_move.iter() {
            self.map[r][c] = SeaCucumber::Empty;
            self.map[(r + 1) % h][c] = SeaCucumber::Down;
        }

        changed
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<SeaCucumber>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect()
            })
            .collect();
        let height = map.len();
        let width = map[0].len();
        Ok(Map { map, width, height })
    }
}

impl Solution for Day2021_25 {
    fn solve_part_1(input: String) -> String {
        let mut map = Map::from_str(&input).unwrap();
        let mut ans = 1;
        while map.tick() > 0 {
            ans += 1;
        }
        ans.to_string()
    }
    fn solve_part_2(_input: String) -> String {
        unimplemented!("")
    }
}
