use super::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn solve_part_1(input: String) -> String {
        let height_map = HeightMap::from(&input);
        height_map.num_visible_trees().to_string()
    }
    fn solve_part_2(input: String) -> String {
        let height_map = HeightMap::from(&input);
        height_map.optimal_scenic_score().to_string()
    }
}
#[derive(Debug)]
struct HeightMap {
    trees: Vec<Vec<u8>>,
}

impl HeightMap {
    fn from(input: &str) -> Self {
        let mut trees = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c as u8 - '0' as u8 + 1);
            }
            trees.push(row);
        }
        Self { trees }
    }

    fn num_visible_trees(&self) -> usize {
        let height = self.trees.len();
        let width = self.trees[0].len();
        let mut visibility = vec![vec![false; width]; height];
        for r in 0..height {
            let mut max = 0;
            for c in 0..width {
                let now = self.trees[r][c];
                if now > max {
                    visibility[r][c] = true;
                    max = now;
                }
            }
            max = 0;
            for c in (0..width).rev() {
                let now = self.trees[r][c];
                if now > max {
                    visibility[r][c] = true;
                    max = now;
                }
            }
        }

        for c in 0..width {
            let mut max = 0;
            for r in 0..height {
                let now = self.trees[r][c];
                if now > max {
                    visibility[r][c] = true;
                    max = now;
                }
            }
            max = 0;
            for r in (0..height).rev() {
                let now = self.trees[r][c];
                if now > max {
                    visibility[r][c] = true;
                    max = now;
                }
            }
        }

        visibility
            .iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .sum()
    }

    fn scenic_score(&self, r: usize, c: usize) -> usize {
        let height = self.trees.len();
        let width = self.trees[0].len();
        let me = self.trees[r][c];
        let mut ans: usize = 1;
        for row in (0..r).rev() {
            let now = self.trees[row][c];
            let num_trees = r - row;
            if now >= me || row == 0 {
                ans *= num_trees;
                break;
            }
        }
        for row in r + 1..height {
            let now = self.trees[row][c];
            let num_trees = row - r;
            if now >= me || row == height - 1 {
                ans *= num_trees;
                break;
            }
        }
        for col in (0..c).rev() {
            let now = self.trees[r][col];
            let num_trees = c - col;
            if now >= me || col == 0 {
                ans *= num_trees;
                break;
            }
        }
        for col in c + 1..width {
            let now = self.trees[r][col];
            let num_trees = col - c;
            if now >= me || col == width - 1 {
                ans *= num_trees;
                break;
            }
        }
        ans
    }

    fn optimal_scenic_score(&self) -> usize {
        let height = self.trees.len();
        let width = self.trees[0].len();
        (1..height - 1)
            .flat_map(|r| (1..width - 1).map(move |c| (r, c)))
            .map(|(r, c)| {
                let score = self.scenic_score(r, c);
                score
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    fn get_test_input() -> String {
        String::from(
            "30373
25512
65332
33549
35390",
        )
    }

    #[test]
    fn test_part_1() {
        let input = get_test_input();
        let ans = Day08::solve_part_1(input);
        assert_eq!(ans, "21".to_string());
    }
    #[test]
    fn test_part_2() {
        let input = get_test_input();
        let ans = Day08::solve_part_2(input);
        assert_eq!(ans, "8".to_string());
    }
}
