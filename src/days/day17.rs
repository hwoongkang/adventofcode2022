use std::collections::HashMap;

use super::Solution;

pub struct Day17;

const CHAMBER_WIDTH: usize = 7;

impl Solution for Day17 {
    fn solve_part_1(input: String) -> String {
        let mut tetris = Tetris::from(&input);
        for _ in 0..2022 {
            tetris.add_block();
        }
        tetris.height.to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut tetris = Tetris::from(&input);
        tetris
            .add_blocks_and_get_height(1_000_000_000_000)
            .to_string()
    }
}

#[derive(Debug, Copy, Clone)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Block {
    block_type: BlockTypes,
    origin: Pos, // top_left
}

impl Block {
    fn push(&mut self, jet: &Jet, grid: &[[bool; CHAMBER_WIDTH]]) {
        match jet {
            Jet::Left => {
                for pos in self.print() {
                    let Pos(x, y) = pos;
                    if x == 0 {
                        return;
                    } else if grid[y][x - 1] {
                        return;
                    }
                }
                self.origin.0 -= 1;
            }
            Jet::Right => {
                for pos in self.print() {
                    let Pos(x, y) = pos;
                    if x == CHAMBER_WIDTH - 1 {
                        return;
                    } else if grid[y][x + 1] {
                        return;
                    }
                }
                self.origin.0 += 1;
            }
        }
    }
    fn fall(&mut self) {
        self.origin.1 -= 1;
    }
    fn hit_test(&self, grid: &[[bool; CHAMBER_WIDTH]]) -> bool {
        for pos in self.print() {
            let Pos(x, y) = pos;
            if y == 0 {
                return true;
            } else if grid[y - 1][x] {
                return true;
            }
        }
        false
    }
    fn print(&self) -> Vec<Pos> {
        use BlockTypes::*;
        let dx: Vec<Pos> = match self.block_type {
            Horizontal => {
                vec![Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)]
            }
            Cross => {
                vec![Pos(1, 0), Pos(1, 1), Pos(0, 1), Pos(2, 1), Pos(1, 2)]
            }
            Angle => {
                vec![Pos(0, 2), Pos(1, 2), Pos(2, 2), Pos(2, 1), Pos(2, 0)]
            }
            Vertical => {
                vec![Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(0, 3)]
            }
            Square => {
                vec![Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)]
            }
        };
        dx.iter()
            .map(|d| Pos(self.origin.0 + d.0, self.origin.1 - d.1))
            .collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
enum BlockTypes {
    Horizontal,
    Cross,
    Angle,
    Vertical,
    Square,
}

impl BlockTypes {
    fn nth(index: usize) -> Self {
        match index % 5 {
            0 => BlockTypes::Horizontal,
            1 => BlockTypes::Cross,
            2 => BlockTypes::Angle,
            3 => BlockTypes::Vertical,
            4 => BlockTypes::Square,
            _ => panic!("Invalid block type index"),
        }
    }
    fn get_bounding_box(&self) -> Pos {
        use BlockTypes::*;
        match self {
            Horizontal => Pos(4, 1),
            Cross => Pos(3, 3),
            Angle => Pos(3, 3),
            Vertical => Pos(1, 4),
            Square => Pos(2, 2),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct CycleDetection {
    jet_index: usize,
    block_type: BlockTypes,
    ceiling: [bool; 7],
}

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

impl std::str::FromStr for Jet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Jet::Left),
            ">" => Ok(Jet::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Tetris {
    height: usize,
    grid: Vec<[bool; CHAMBER_WIDTH]>,
    jet_pattern: Vec<Jet>,
    jet_index: usize,
    block_index: usize,
    cache: HashMap<CycleDetection, usize>,
    block_count: usize,
}

impl Tetris {
    fn from(input: &str) -> Self {
        let jet_pattern: Vec<Jet> = input
            .chars()
            .map(|ch| ch.to_string().parse().unwrap())
            .collect();
        let jet_index = jet_pattern.len();
        let block_index = 0;
        Tetris {
            height: 0,
            grid: vec![[false; CHAMBER_WIDTH]],
            jet_pattern,
            jet_index,
            block_index,
            cache: HashMap::new(),
            block_count: 0,
        }
    }

    fn add_blocks_and_get_height(&mut self, num_blocks: usize) -> usize {
        let prev_count = loop {
            if let Some(prev_count) = self.add_block() {
                break prev_count;
            }
        };
        let prev_height = self.height;

        let cycle_length = self.block_count - prev_count;

        println!("{}, {}", prev_count, cycle_length);

        for _ in 0..cycle_length {
            self.add_block();
        }

        let height = self.height;

        let delta_height = height - prev_height;

        let count = self.block_count;

        let remaining = num_blocks - count;

        for _ in 0..remaining % cycle_length {
            self.add_block();
        }

        let mut ans = self.height;

        ans += delta_height * (remaining / cycle_length);

        ans
    }

    fn add_block(&mut self) -> Option<usize> {
        self.block_count += 1;
        let block_type = BlockTypes::nth(self.block_index);
        self.block_index += 1;

        let mut block = Block {
            block_type,
            origin: Pos(2, self.height + 2 + block_type.get_bounding_box().1),
        };

        while self.grid.len() <= block.origin.1 {
            self.grid.push([false; CHAMBER_WIDTH]);
        }

        loop {
            let jet = &self.jet_pattern[self.jet_index % self.jet_pattern.len()];
            self.jet_index += 1;

            block.push(jet, &self.grid);

            if block.hit_test(&self.grid) {
                break;
            }

            block.fall();
        }

        for pos in block.print() {
            let Pos(x, y) = pos;
            self.grid[y][x] = true;
        }
        self.height = self.height.max(block.origin.1 + 1);

        let ceiling = self.grid[self.height - 1].clone();
        let cycle_detection = CycleDetection {
            jet_index: self.jet_index % self.jet_pattern.len(),
            block_type: block.block_type,
            ceiling,
        };
        if let Some(&prev_block_count) = self.cache.get(&cycle_detection) {
            Some(prev_block_count)
        } else {
            self.cache.insert(cycle_detection, self.block_count);
            None
        }
    }

    fn _pretty_print(&self) {
        for row in self.grid.iter().rev() {
            for &cell in row.iter() {
                if cell {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")
    }

    #[test]
    fn part1() {
        let input = get_sample_input();
        let ans = Day17::solve_part_1(input);
        let expected = "3068";
        assert_eq!(ans, expected);
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day17::solve_part_2(input);
        let expected = "1514285714288";
        assert_eq!(ans, expected);
    }
}
