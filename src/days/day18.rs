use super::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn solve_part_1(input: String) -> String {
        let grid = Grid::from(&input);
        grid.surface_area().to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[derive(Debug, Copy, Clone)]
struct Point(usize, usize, usize);

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.trim().split(',');
        let x = words.next().unwrap().parse().unwrap();
        let y = words.next().unwrap().parse().unwrap();
        let z = words.next().unwrap().parse().unwrap();
        Ok(Point(x, y, z))
    }
}

struct Grid {
    size: Point,
    grid: Vec<Vec<Vec<bool>>>,
}

impl Grid {
    fn from(input: &str) -> Self {
        let lava: Vec<Point> = input
            .lines()
            .map(|l| l.parse::<Point>().unwrap())
            .map(|p| Point(p.0 + 1, p.1 + 1, p.2 + 1))
            .collect();
        let max_x = lava.iter().map(|p| p.0).max().unwrap();
        let max_x = max_x + 2;
        let max_y = lava.iter().map(|p| p.1).max().unwrap();
        let max_y = max_y + 2;
        let max_z = lava.iter().map(|p| p.2).max().unwrap();
        let max_z = max_z + 2;
        let mut grid = vec![vec![vec![false; max_z]; max_y]; max_x];
        for p in lava.iter() {
            grid[p.0][p.1][p.2] = true;
        }
        Self {
            grid,
            size: Point(max_x, max_y, max_z),
        }
    }

    fn adjacent_points(&self, point: Point) -> Vec<Point> {
        let mut ans = vec![];

        let Point(x, y, z) = point;

        if x > 0 {
            ans.push(Point(x - 1, y, z));
        }
        if x < self.size.0 - 1 {
            ans.push(Point(x + 1, y, z));
        }
        if y > 0 {
            ans.push(Point(x, y - 1, z));
        }
        if y < self.size.1 - 1 {
            ans.push(Point(x, y + 1, z));
        }
        if z > 0 {
            ans.push(Point(x, y, z - 1));
        }
        if z < self.size.2 - 1 {
            ans.push(Point(x, y, z + 1));
        }

        ans
    }

    fn surface_area(&self) -> usize {
        let mut ans = 0;
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                for z in 0..self.size.2 {
                    if self.grid[x][y][z] {
                        ans += self
                            .adjacent_points(Point(x, y, z))
                            .iter()
                            .filter(|p| !self.grid[p.0][p.1][p.2])
                            .count();
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
        )
    }

    fn get_smaller_sample() -> String {
        String::from(
            "1,1,1
        2,1,1",
        )
    }

    #[test]
    fn part1() {
        let smaller = get_smaller_sample();
        let ans = Day18::solve_part_1(smaller);
        let expected = "10";
        assert_eq!(ans, expected);
        let input = get_sample_input();
        let ans = Day18::solve_part_1(input);
        let expected = "64";
        assert_eq!(ans, expected);
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day18::solve_part_2(input);
        let expected = "";
        assert_eq!(ans, expected);
    }
}
