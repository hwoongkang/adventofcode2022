use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use super::Solution;

pub struct Day16;

impl Solution for Day16 {
    fn solve_part_1(input: String) -> String {
        let mut cave = Cave::new();
        cave.add_valves(&input);
        println!("{:?}", cave);
        String::new()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    neighbors: Vec<Weak<RefCell<Valve>>>,
}

impl Valve {
    fn new(flow_rate: usize) -> Self {
        Self {
            flow_rate,
            neighbors: vec![],
        }
    }
}

#[derive(Debug)]
struct Cave {
    valves: HashMap<String, Rc<RefCell<Valve>>>,
}

impl Cave {
    fn new() -> Self {
        Self {
            valves: HashMap::new(),
        }
    }

    fn add_valves(&mut self, input: &str) {
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        for line in input.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            let name = words[1].to_string();
            let flow_rate = words[4];
            let flow_rate = flow_rate[5..flow_rate.len() - 1].parse().unwrap();
            let valve = Rc::new(RefCell::new(Valve::new(flow_rate)));
            let neighbors: Vec<String> = words[9..]
                .iter()
                .map(|w| w.trim_end_matches(",").to_string())
                .collect();
            edges.insert(name.clone(), neighbors);
            self.valves.insert(name, valve.clone());
        }
        for (from, to) in edges.iter() {
            let mut neighbors = vec![];
            for name in to.iter() {
                let t = self.valves.get(name).unwrap();
                neighbors.push(Rc::downgrade(t));
            }
            let from = &mut self.valves.get_mut(from).unwrap();
            from.borrow_mut().neighbors = neighbors;
        }
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
