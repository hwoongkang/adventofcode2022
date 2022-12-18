use std::{
    cell::RefCell,
    collections::HashMap,
    ops::DivAssign,
    rc::{Rc, Weak},
};

use super::Solution;

pub struct Day16;

impl Solution for Day16 {
    fn solve_part_1(input: String) -> String {
        let mut cave = Cave::from(input);
        cave.part_1().to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

struct Cave {
    connections: HashMap<String, Vec<String>>,
    flow_rates: HashMap<String, usize>,
    bit_masks: HashMap<String, usize>,
    distances: HashMap<String, HashMap<String, usize>>,
}

impl Cave {
    fn from(input: String) -> Self {
        let mut valves: Vec<(String, usize, Vec<String>)> = input
            .lines()
            .map(|line| {
                let words: Vec<&str> = line.split_whitespace().collect();
                let name = words[1].to_string();
                let flow_rate = words[4];
                let flow_rate = flow_rate[5..flow_rate.len() - 1].parse().unwrap();
                let neighbors = words[9..]
                    .iter()
                    .map(|w| w.trim_end_matches(",").to_string())
                    .collect();
                (name, flow_rate, neighbors)
            })
            .collect();
        let connections: HashMap<String, Vec<String>> = valves
            .iter()
            .map(|(name, _, neighbors)| (name.clone(), neighbors.clone()))
            .collect();
        let flow_rates: HashMap<String, usize> = valves
            .iter()
            .filter_map(|(name, flow_rate, _)| {
                if flow_rate == &0 {
                    None
                } else {
                    Some((name.clone(), *flow_rate))
                }
            })
            .collect();
        let bit_masks: HashMap<String, usize> = flow_rates
            .keys()
            .enumerate()
            .map(|(index, name)| (name.clone(), 1 << index))
            .collect();
        let mut distances: HashMap<String, HashMap<String, usize>> = connections
            .keys()
            .map(|name| {
                (name.clone(), {
                    connections
                        .keys()
                        .map(|n2| {
                            (
                                n2.clone(),
                                if connections.get(name).unwrap().contains(&n2) {
                                    1
                                } else {
                                    1_000_000
                                },
                            )
                        })
                        .collect()
                })
            })
            .collect();
        let keys: Vec<&String> = connections.keys().clone().collect();
        for &k in keys.iter() {
            for &i in keys.iter() {
                for &j in keys.iter() {
                    let d1 = distances[i][j];
                    let d2 = distances[i][k] + distances[k][j];
                    if d2 < d1 {
                        distances.get_mut(i).unwrap().insert(j.to_string(), d2);
                    }
                }
            }
        }
        Self {
            connections,
            flow_rates,
            bit_masks,
            distances,
        }
    }

    fn part_1(&self) -> usize {
        let mut ans: HashMap<usize, usize> = HashMap::new();
        self.visit("AA", 30, 0, 0, &mut ans);
        *ans.values().max().unwrap()
    }

    fn visit(
        &self,
        valve: &str,
        budget: usize,
        state: usize,
        flow: usize,
        ans: &mut HashMap<usize, usize>,
    ) -> usize {
        let prev_state = ans.get(&state).unwrap_or(&0);
        if flow > *prev_state {
            ans.insert(state, flow);
        }

        for (name, flow_rate) in self.flow_rates.iter() {
            let distance = self.distances[valve][name];
            if distance + 1 >= budget {
                continue;
            }
            if self.bit_masks[name] & state != 0 {
                continue;
            }
            let newbudget = budget - distance - 1;
            self.visit(
                name,
                newbudget,
                state | self.bit_masks[name],
                flow + newbudget * flow_rate,
                ans,
            );
        }

        0
    }
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        )
    }

    #[test]
    fn test_part_1() {
        let input = get_sample_input();
        let ans = Day16::solve_part_1(input);
        let expected = "1651";
        assert_eq!(ans, expected)
    }
}
