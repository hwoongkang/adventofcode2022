use std::collections::HashMap;

use super::Solution;

pub struct Day19;

impl Solution for Day19 {
    fn solve_part_1(input: String) -> String {
        let blueprints: Vec<Blueprint> = input.lines().map(|l| l.parse().unwrap()).collect();

        blueprints
            .iter()
            .map(|b| b.maximize())
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

type PerResource = [u32; 4];

type State = (PerResource, PerResource);

#[derive(Debug)]
struct Blueprint {
    costs: (u32, u32, (u32, u32), (u32, u32)),
    max_ore_cost: u32,
}

impl Blueprint {
    fn maximize(&self) -> u32 {
        let mut dp: Vec<Vec<State>> = vec![vec![([1, 0, 0, 0], [0; 4])]];

        for i in 0..24 {
            let prev_states = &dp[dp.len() - 1];

            println!("processing {} minutes... length: {}", i, prev_states.len());
            let mut next_states = vec![];

            for &(robots, stocks) in prev_states.iter() {
                let mut branches: Vec<State> = self
                    .possible_buildings((robots, stocks))
                    .iter()
                    .map(|(r, s)| {
                        let mut new_robots = robots;
                        for i in 0..4 {
                            new_robots[i] += r[i];
                        }

                        (new_robots, *s)
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

            dp.push(next_states);
        }

        dp.last().unwrap().iter().map(|(_, s)| s[3]).max().unwrap()
    }

    fn possible_buildings(
        &self,
        (robots, stocks): (PerResource, PerResource),
    ) -> Vec<(PerResource, PerResource)> {
        let ores = |stocks: PerResource| stocks[0] / self.costs.0;

        let clays = |stocks: PerResource| stocks[0] / self.costs.1;

        let obsidians = |stocks: PerResource| {
            let max_by_ores = stocks[0] / self.costs.2 .0;
            let max_by_clays = stocks[1] / self.costs.2 .1;

            max_by_ores.min(max_by_clays)
        };

        let geodes = |stocks: PerResource| {
            let max_by_ores = stocks[0] / self.costs.3 .0;
            let max_by_obsidians = stocks[2] / self.costs.3 .1;

            max_by_ores.min(max_by_obsidians)
        };

        let mut ans = vec![];

        let max_geodes = geodes(stocks);

        for geode in 1.min(max_geodes)..=max_geodes {
            let mut stock = stocks.clone();
            stock[0] -= geode * self.costs.3 .0;
            stock[2] -= geode * self.costs.3 .1;

            let max_obsidians = obsidians(stock);

            for obsidian in 1.min(max_obsidians)..=max_obsidians {
                let mut stock = stock.clone();
                stock[0] -= obsidian * self.costs.2 .0;
                stock[1] -= obsidian * self.costs.2 .1;

                let ores = if self.costs.0 > 3 {
                    0..=0
                } else {
                    0..=ores(stock)
                };

                for ore in ores {
                    let mut stock = stock.clone();
                    stock[0] -= ore * self.costs.0;

                    let clays = if robots[1] >= 4 {
                        0..=0
                    } else {
                        0..=clays(stock)
                    };

                    for clay in clays {
                        let mut stock = stock.clone();
                        stock[0] -= clay * self.costs.1;

                        ans.push(([ore, clay, obsidian, geode], stock));
                    }
                }
            }
        }

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

        let max_ore_cost = ore_robot
            .max(clay_robot)
            .max(obsidian_robot.0)
            .max(geode_robot.0);

        Ok(Self {
            costs: (ore_robot, clay_robot, obsidian_robot, geode_robot),
            max_ore_cost,
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
        let expected = "";
        assert_eq!(ans, expected);
    }
}
