use std::collections::HashMap;

use super::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn solve_part_1(input: String) -> String {
        let mut map = Map::from(&input, false);

        let command_line = input
            .lines()
            .skip_while(|line| line != &"")
            .skip(1)
            .next()
            .unwrap();
        let commands = parse_command(command_line);

        for command in commands {
            map.exec(&command);
        }

        map.score().to_string()
    }
    fn solve_part_2(input: String) -> String {
        part_2(input, false).to_string()
    }
}

fn part_2(input: String, testing: bool) -> String {
    let mut map = Map::from(&input, testing);

    let command_line = input
        .lines()
        .skip_while(|line| line != &"")
        .skip(1)
        .next()
        .unwrap();
    let commands = parse_command(command_line);

    for command in commands {
        map.exec_v2(&command);
    }

    map.score().to_string()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

impl Facing {
    fn turn(&self, turn: &Turn) -> Facing {
        match turn {
            Turn::Left => match self {
                Facing::Up => Facing::Left,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
                Facing::Right => Facing::Up,
            },
            Turn::Right => match self {
                Facing::Up => Facing::Right,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Right => Facing::Down,
            },
        }
    }

    fn score(&self) -> usize {
        match self {
            Facing::Up => 3,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Right => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Open,
    Solid,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    facing: Facing,
    pos: Pos,
    testing: bool,
}

impl Map {
    fn from(input: &str, testing: bool) -> Self {
        let mut tiles: Vec<Vec<Tile>> = vec![];
        let facing = Facing::Right;
        let mut pos = Pos(0, 0);
        for line in input.lines() {
            if line == "" {
                break;
            }
            let row: Vec<Tile> = line
                .trim_end()
                .chars()
                .map(|ch| match ch {
                    '.' => Tile::Open,
                    '#' => Tile::Solid,
                    _ => Tile::Empty,
                })
                .collect();
            tiles.push(row);
        }
        let width = tiles.iter().map(|row| row.len()).max().unwrap();
        for row in tiles.iter_mut() {
            while row.len() < width {
                row.push(Tile::Empty);
            }
        }
        pos.1 = tiles[0]
            .iter()
            .position(|tile| *tile == Tile::Open)
            .unwrap();
        Self {
            tiles,
            facing,
            pos,
            testing,
        }
    }

    fn exec_turn(&mut self, turn: &Turn) {
        self.facing = self.facing.turn(turn);
    }

    fn exec_move(&mut self, n: usize) {
        for _ in 0..n {
            self.pos = self.next_pos();
        }
    }
    fn exec_move_v2(&mut self, n: usize) {
        for _ in 0..n {
            self.next_pos_v2_testing();
            println!("{} {}", self.pos.0, self.pos.1);
        }
    }

    fn exec(&mut self, command: &Command) {
        match command {
            Command::Turn(turn) => self.exec_turn(turn),
            Command::Move(n) => self.exec_move(*n),
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }

    fn dp(&self) -> Pos {
        let (rows, cols) = self.size();
        match self.facing {
            Facing::Up => Pos(rows - 1, 0),
            Facing::Down => Pos(1, 0),
            Facing::Left => Pos(0, cols - 1),
            Facing::Right => Pos(0, 1),
        }
    }

    fn next_pos(&self) -> Pos {
        let (rows, cols) = self.size();
        let dp = self.dp();

        let now = self.pos;

        loop {
            let next = Pos((now.0 + dp.0) % rows, (now.1 + dp.1) % cols);
            match self.tiles[next.0][next.1] {
                Tile::Open => break next,
                Tile::Solid => break now,
                Tile::Empty => {
                    let mut new_ans = next;
                    while self.tiles[new_ans.0][new_ans.1] == Tile::Empty {
                        new_ans = Pos((new_ans.0 + dp.0) % rows, (new_ans.1 + dp.1) % cols);
                    }
                    if self.tiles[new_ans.0][new_ans.1] == Tile::Open {
                        break new_ans;
                    } else {
                        break now;
                    }
                }
            }
        }
    }

    fn score(&self) -> usize {
        let Pos(row, col) = self.pos;
        1000 * (row + 1) + 4 * (col + 1) + self.facing.score()
    }

    fn face_size(&self) -> usize {
        match self.testing {
            true => 4,
            false => 50,
        }
    }

    fn transform_rel(&self, rel: Pos, from: &Facing, to: &Facing) -> Pos {
        let face_size = self.face_size();
        match (from, to) {
            (Facing::Up, Facing::Up) => Pos(face_size - 1 - rel.0, rel.1),
            (Facing::Up, Facing::Down) => Pos(0, face_size - 1 - rel.1),
            (Facing::Up, Facing::Right) => Pos(rel.1, 0),
            (Facing::Up, Facing::Left) => Pos(face_size - 1 - rel.1, face_size - 1),
            (Facing::Down, Facing::Up) => Pos(face_size - 1, face_size - 1 - rel.1),
            (Facing::Down, Facing::Down) => Pos(face_size - 1 - rel.0, rel.1),
            (Facing::Down, Facing::Right) => Pos(face_size - 1 - rel.1, 0),
            (Facing::Down, Facing::Left) => Pos(rel.1, face_size - 1),
            (Facing::Right, Facing::Up) => Pos(face_size - 1, rel.0),
            (Facing::Right, Facing::Down) => Pos(0, face_size - 1 - rel.0),
            (Facing::Right, Facing::Right) => rel,
            (Facing::Right, Facing::Left) => Pos(face_size - 1 - rel.0, face_size - 1),
            (Facing::Left, Facing::Up) => Pos(face_size - 1, face_size - 1 - rel.0),
            (Facing::Left, Facing::Down) => Pos(0, rel.0),
            (Facing::Left, Facing::Right) => Pos(face_size - 1 - rel.0, 0),
            (Facing::Left, Facing::Left) => rel,
        }
    }

    fn connections(&self) -> HashMap<(Pos, Facing), (Pos, Facing)> {
        if self.testing {
            // ..#
            // ###
            // ..##
            HashMap::from_iter([
                ((Pos(0, 2), Facing::Right), ((Pos(2, 3), Facing::Left))),
                ((Pos(0, 2), Facing::Up), ((Pos(1, 0), Facing::Down))),
                ((Pos(0, 2), Facing::Left), ((Pos(1, 1), Facing::Down))),
                ((Pos(1, 1), Facing::Up), ((Pos(0, 2), Facing::Right))),
                ((Pos(1, 0), Facing::Up), ((Pos(0, 2), Facing::Down))),
                ((Pos(1, 0), Facing::Left), ((Pos(2, 3), Facing::Up))),
                ((Pos(1, 0), Facing::Down), ((Pos(2, 2), Facing::Up))),
                ((Pos(1, 1), Facing::Down), ((Pos(1, 1), Facing::Up))),
                ((Pos(2, 2), Facing::Left), ((Pos(2, 2), Facing::Right))),
                ((Pos(2, 2), Facing::Down), ((Pos(1, 0), Facing::Up))),
                ((Pos(2, 3), Facing::Down), ((Pos(1, 0), Facing::Right))),
                ((Pos(2, 3), Facing::Right), ((Pos(0, 2), Facing::Left))),
                ((Pos(2, 3), Facing::Up), ((Pos(1, 2), Facing::Left))),
                ((Pos(1, 2), Facing::Right), ((Pos(2, 3), Facing::Down))),
            ])
        } else {
            // .##
            // .#
            // ##
            // #
            HashMap::from_iter([
                ((Pos(0, 1), Facing::Left), ((Pos(2, 0), Facing::Right))),
                ((Pos(1, 1), Facing::Left), ((Pos(2, 0), Facing::Down))),
                ((Pos(2, 0), Facing::Up), (Pos(1, 1), Facing::Right)),
                ((Pos(2, 0), Facing::Left), (Pos(0, 1), Facing::Right)),
                ((Pos(3, 0), Facing::Left), (Pos(0, 1), Facing::Down)),
                ((Pos(3, 0), Facing::Down), (Pos(0, 2), Facing::Down)),
                ((Pos(3, 0), Facing::Right), (Pos(2, 1), Facing::Up)),
                ((Pos(2, 1), Facing::Down), (Pos(3, 0), Facing::Left)),
                ((Pos(2, 1), Facing::Right), (Pos(0, 2), Facing::Left)),
                ((Pos(1, 1), Facing::Right), (Pos(0, 2), Facing::Up)),
                ((Pos(0, 2), Facing::Down), (Pos(1, 1), Facing::Left)),
                ((Pos(0, 2), Facing::Right), (Pos(2, 1), Facing::Left)),
                ((Pos(0, 2), Facing::Up), (Pos(3, 0), Facing::Up)),
                ((Pos(0, 1), Facing::Up), (Pos(3, 0), Facing::Right)),
            ])
        }
    }

    fn abs_to_rel(&self, pos: Pos) -> (Pos, Pos) {
        let face_size = self.face_size();
        let rel = Pos(pos.0 % face_size, pos.1 % face_size);
        let index = Pos(pos.0 / face_size, pos.1 / face_size);
        (rel, index)
    }

    fn rel_to_abs(&self, rel: Pos, face_index: Pos) -> Pos {
        let face_size = self.face_size();
        let row = face_size * face_index.0 + rel.0;
        let col = face_size * face_index.1 + rel.1;
        Pos(row, col)
    }

    fn next_pos_v2_testing(&mut self) {
        let (rows, cols) = self.size();
        let dp = self.dp();
        let now = self.pos;

        let on_left = now.1 == 0 && self.facing == Facing::Left;
        let on_top = now.0 == 0 && self.facing == Facing::Up;
        let on_right = now.1 == cols - 1 && self.facing == Facing::Right;
        let on_bottom = now.0 == rows - 1 && self.facing == Facing::Down;

        if on_left || on_top || on_right || on_bottom {
        } else {
            let next = Pos((now.0 + dp.0) % rows, (now.1 + dp.1) % cols);

            match self.tiles[next.0][next.1] {
                Tile::Open => {
                    self.pos = next;
                    return;
                }
                Tile::Solid => {
                    self.pos = now;
                    return;
                }
                _ => {}
            }
        }

        let (rel, face_index) = self.abs_to_rel(now);

        let connections = self.connections();

        let (new_face_index, new_facing) = connections.get(&(face_index, self.facing)).unwrap();

        let new_rel = self.transform_rel(rel, &self.facing, new_facing);

        let new_abs_pos = self.rel_to_abs(new_rel, *new_face_index);

        match self.tiles[new_abs_pos.0][new_abs_pos.1] {
            Tile::Open => {
                self.pos = new_abs_pos;
                self.facing = *new_facing;
                return;
            }
            Tile::Solid => {
                self.pos = now;
                return;
            }
            _ => unreachable!(),
        }
    }

    fn exec_v2(&mut self, command: &Command) {
        match command {
            Command::Turn(turn) => {
                self.exec_turn(turn);
                println!("turn");
            }
            Command::Move(n) => self.exec_move_v2(*n),
        }
    }
}

#[derive(Debug)]
enum Command {
    Turn(Turn),
    Move(usize),
}

fn parse_command(line: &str) -> Vec<Command> {
    let mut num = String::new();
    let mut commands = vec![];

    for ch in line.chars() {
        match ch {
            'L' | 'R' => {
                if let Ok(n) = num.parse() {
                    commands.push(Command::Move(n));
                    num = String::new();
                }
                match ch {
                    'L' => commands.push(Command::Turn(Turn::Left)),
                    'R' => commands.push(Command::Turn(Turn::Right)),
                    _ => (),
                }
            }
            x => {
                num.push(x);
            }
        }
    }

    if let Ok(n) = num.parse() {
        commands.push(Command::Move(n));
    }

    commands
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
        )
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = Day22::solve_part_1(input);
        let expected = "6032";
        assert_eq!(ans, expected)
    }

    #[test]
    fn test_part_2() {
        let input = get_sample_input();

        let ans = part_2(input, true);
        let expected = "5031";
        assert_eq!(ans, expected)
    }
}
