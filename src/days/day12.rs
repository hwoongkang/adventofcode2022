use super::Solution;
use std::collections::VecDeque;

pub struct Day12;

impl Solution for Day12 {
    fn solve_part_1(input: String) -> String {
        let height_map = HeightMap::from(input);

        height_map.shortest_path().to_string()
    }

    fn solve_part_2(input: String) -> String {
        let height_map = HeightMap::from(input);

        height_map.shortest_path_to_a().to_string()
    }
}

struct HeightMap {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<i8>>,
}

impl HeightMap {
    fn from(input: String) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| match ch {
                        'S' => {
                            start = (r, c);
                            0
                        }
                        'E' => {
                            end = (r, c);
                            'z' as i8 - 'a' as i8
                        }
                        x => (x as i8) - ('a' as i8),
                    })
                    .collect()
            })
            .collect();
        Self { start, end, map }
    }

    fn shortest_path(&self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0));
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        visited[self.start.0][self.start.1] = true;

        while let Some(((r, c), dist)) = queue.pop_front() {
            if (r, c) == self.end {
                return dist;
            }
            for (nr, nc) in self.next_cell(r, c) {
                if visited[nr][nc] {
                    continue;
                }
                visited[nr][nc] = true;
                queue.push_back(((nr, nc), dist + 1));
            }
        }
        0
    }

    fn prev_cell(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let now = self.map[r][c];
        if r > 0 {
            let n = (r - 1, c);
            if now - self.map[n.0][n.1] <= 1 {
                ans.push(n);
            }
        }
        if r < self.map.len() - 1 {
            let n = (r + 1, c);
            if now - self.map[n.0][n.1] <= 1 {
                ans.push(n);
            }
        }
        if c > 0 {
            let n = (r, c - 1);
            if now - self.map[n.0][n.1] <= 1 {
                ans.push(n);
            }
        }
        if c < self.map[0].len() - 1 {
            let n = (r, c + 1);
            if now - self.map[n.0][n.1] <= 1 {
                ans.push(n);
            }
        }
        ans
    }

    fn shortest_path_to_a(&self) -> usize {
        let mut dists = vec![];
        let mut queue = VecDeque::new();
        queue.push_back((self.end, 0));
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        while let Some(((r, c), dist)) = queue.pop_front() {
            let height = self.map[r][c];
            if height == 0 {
                dists.push(dist);
            }
            for (nr, nc) in self.prev_cell(r, c) {
                if visited[nr][nc] {
                    continue;
                }
                visited[nr][nc] = true;
                queue.push_back(((nr, nc), dist + 1));
            }
        }

        *dists.iter().min().unwrap_or(&0)
    }

    fn next_cell(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let now = self.map[r][c];
        if r > 0 {
            let n = (r - 1, c);
            if self.map[n.0][n.1] - now <= 1 {
                ans.push(n);
            }
        }
        if r < self.map.len() - 1 {
            let n = (r + 1, c);
            if self.map[n.0][n.1] - now <= 1 {
                ans.push(n);
            }
        }
        if c > 0 {
            let n = (r, c - 1);
            if self.map[n.0][n.1] - now <= 1 {
                ans.push(n);
            }
        }
        if c < self.map[0].len() - 1 {
            let n = (r, c + 1);
            if self.map[n.0][n.1] - now <= 1 {
                ans.push(n);
            }
        }
        ans
    }
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        )
    }

    #[test]
    fn part_1() {
        let input = get_sample_input();
        assert_eq!(Day12::solve_part_1(input), "31");
    }

    #[test]
    fn part_2() {
        let input = get_sample_input();
        assert_eq!(Day12::solve_part_2(input), "29");
    }
}
