use std::collections::HashSet;

use super::Solution;

pub struct Day14;

impl Solution for Day14 {
    fn solve_part_1(input: String) -> String {
        let mut cave = Cave::new();
        for line in input.lines() {
            cave.add_wall(line);
        }
        let mut ans = 0;
        while !cave.add_sand() {
            ans += 1;
        }
        ans.to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim().split(",").collect();
        Ok(Point(
            words[0].parse::<i32>().unwrap(),
            words[1].parse::<i32>().unwrap(),
        ))
    }
}

struct Cave {
    highest: i32,
    walls: HashSet<Point>,
}
impl Cave {
    fn new() -> Self {
        Self {
            highest: 0,
            walls: HashSet::new(),
        }
    }

    fn add_wall(&mut self, input: &str) {
        let points: Vec<Point> = input.split("->").map(|w| w.parse().unwrap()).collect();
        for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
            let &Point(x1, y1) = p1;
            let &Point(x2, y2) = p2;

            if x1 == x2 {
                let (y1, y2) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
                for y in y1..=y2 {
                    self.walls.insert(Point(x1, y));
                    self.highest = self.highest.max(y);
                }
            } else if y1 == y2 {
                self.highest = self.highest.max(y1);
                let (x1, x2) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                for x in x1..=x2 {
                    self.walls.insert(Point(x, y1));
                }
            }
        }
    }

    fn add_sand(&mut self) -> bool {
        let mut sand = Point(500, 0);

        'sand_loop: loop {
            if sand.1 == self.highest {
                return true;
            }
            let next_points = [
                Point(sand.0, sand.1 + 1),
                Point(sand.0 - 1, sand.1 + 1),
                Point(sand.0 + 1, sand.1 + 1),
            ];
            for point in next_points {
                if !self.walls.contains(&point) {
                    sand = point;
                    continue 'sand_loop;
                }
            }
            self.walls.insert(sand);
            return false;
        }
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        )
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = Day14::solve_part_1(input);
        let expected = "24";
        assert_eq!(ans, expected);
    }
}
