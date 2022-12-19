use std::collections::HashMap;

use super::Solution;

pub struct Day19;

impl Solution for Day19 {
    fn solve_part_1(input: String) -> String {
        let blueprints: Vec<Blueprint> = input.lines().map(|l| l.parse().unwrap()).collect();

        blueprints
            .iter()
            .map(|b| b.maximize(24))
            .enumerate()
            .map(|(i, b)| ((i + 1) as u32) * b)
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        let blueprints: Vec<Blueprint> =
            input.lines().take(3).map(|l| l.parse().unwrap()).collect();
        blueprints
            .iter()
            .map(|b| b.maximize(32))
            .fold(1, |a, b| a * b)
            .to_string()
    }
}

type PerResource = [u32; 4];

type State = (PerResource, PerResource);

fn compare_state(s1: &State, s2: &State) -> std::cmp::Ordering {
    let (r1, s1) = s1;
    let (r2, s2) = s2;
    for i in (0..4).rev() {
        match (s2[i] + r2[i]).cmp(&(s1[i] + r1[i])) {
            std::cmp::Ordering::Equal => continue,
            o => return o,
        }
    }
    std::cmp::Ordering::Equal
}

#[derive(Debug)]
struct Blueprint {
    costs: (u32, u32, (u32, u32), (u32, u32)),
}

impl Blueprint {
    fn maximize(&self, minutes: usize) -> u32 {
        let mut dp: Vec<Vec<State>> = vec![vec![([1, 0, 0, 0], [0; 4])]];

        for i in 0..minutes {
            let prev_states = &dp[dp.len() - 1];

            let mut next_states = vec![];

            for &(robots, stocks) in prev_states.iter() {
                let mut branches: Vec<State> = self
                    .possible_buildings((robots, stocks))
                    .iter()
                    .map(|(dr, ds)| {
                        let mut r = robots;
                        let mut s = stocks;

                        for i in 0..4 {
                            r[i] += dr[i];
                            s[i] -= ds[i];
                        }

                        (r, s)
                    })
                    .map(|(r, mut s)| {
                        for (i, num) in robots.iter().enumerate() {
                            s[i] += num;
                        }

                        (r, s)
                    })
                    .collect();

                next_states.append(&mut branches);
            }

            next_states.sort_by(compare_state);

            next_states.truncate(5000);

            dp.push(next_states);
        }

        let ans = dp.last().unwrap().iter().map(|(_, s)| s[3]).max().unwrap();

        ans
    }

    fn possible_buildings(
        &self,
        (robots, stocks): (PerResource, PerResource),
    ) -> Vec<(PerResource, PerResource)> {
        let mut ans = vec![];

        if stocks[0] >= self.costs.3 .0 && stocks[2] >= self.costs.3 .1 {
            ans.push(([0, 0, 0, 1], [self.costs.3 .0, 0, self.costs.3 .1, 0]));
        }
        if stocks[0] >= self.costs.2 .0 && stocks[1] >= self.costs.2 .1 {
            ans.push(([0, 0, 1, 0], [self.costs.2 .0, self.costs.2 .1, 0, 0]));
        }

        if stocks[0] >= self.costs.0 {
            ans.push(([1, 0, 0, 0], [self.costs.0, 0, 0, 0]));
        }

        if stocks[0] >= self.costs.1 {
            ans.push(([0, 1, 0, 0], [self.costs.1, 0, 0, 0]));
        }

        ans.push(([0, 0, 0, 0], [0, 0, 0, 0]));

        ans
    }
}

impl std::str::FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sentences: Vec<Vec<&str>> = s
            .split('.')
            .map(|s| s.split_whitespace().collect())
            .collect();

        let ore_robot: u32;
        let clay_robot;
        let obsidian_robot;
        let geode_robot;

        let first = &sentences[0];

        ore_robot = first[first.len() - 2].parse().unwrap();

        let clay = &sentences[1];

        clay_robot = clay[clay.len() - 2].parse().unwrap();

        let obsidian = &sentences[2];

        obsidian_robot = (
            obsidian[obsidian.len() - 5].parse().unwrap(),
            obsidian[obsidian.len() - 2].parse().unwrap(),
        );

        let geode = &sentences[3];

        geode_robot = (
            geode[geode.len() - 5].parse().unwrap(),
            geode[geode.len() - 2].parse().unwrap(),
        );

        Ok(Self {
            costs: (ore_robot, clay_robot, obsidian_robot, geode_robot),
        })
    }
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
        )
    }

    #[test]
    fn cartesian_product() {
        let from_zero = (0..=0).flat_map(|prev| (0..=1).map(move |next| (prev, next)));
        assert_eq!(from_zero.collect::<Vec<_>>(), vec![(0, 0), (0, 1)])
    }

    #[test]
    fn part1() {
        let input = get_sample_input();
        let ans = Day19::solve_part_1(input);
        let expected = "33";
        assert_eq!(ans, expected);
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day19::solve_part_2(input);
        let expected = (56 * 62).to_string();
        assert_eq!(ans, expected);
    }
}
