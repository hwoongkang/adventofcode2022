use super::Solution;

use super::day11_inputs::*;

pub struct Day11;

impl Solution for Day11 {
    fn solve_part_1(_input: String) -> String {
        let mut monkey_cage = get_input();
        for _ in 0..20 {
            monkey_cage.tick();
        }
        let mut inspect_count = monkey_cage
            .monkeys
            .iter()
            .map(|m| m.inspect_count)
            .collect::<Vec<_>>();
        inspect_count.sort();
        let ans = inspect_count
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, &x| acc * x);

        ans.to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

type Operation = fn(i64) -> i64;

pub struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    if_true: usize,
    if_false: usize,
    inspect_count: usize,
}

impl Monkey {
    pub fn new(
        items: Vec<i64>,
        operation: Operation,
        test: i64,
        if_true: usize,
        if_false: usize,
    ) -> Monkey {
        Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspect_count: 0,
        }
    }
    fn tick(&mut self) -> Vec<(usize, i64)> {
        self.items
            .drain(..)
            .map(|worry_level| {
                self.inspect_count += 1;
                let increased = (self.operation)(worry_level);
                let increased = increased / 3;
                if increased % self.test == 0 {
                    (self.if_true, increased)
                } else {
                    (self.if_false, increased)
                }
            })
            .collect()
    }
}

pub struct MonkeyCage {
    pub monkeys: Vec<Monkey>,
}

impl MonkeyCage {
    fn tick(&mut self) {
        for i in 0..self.monkeys.len() {
            let thrown_items = self.monkeys[i].tick();
            for &(monkey_index, worry_level) in thrown_items.iter() {
                self.monkeys[monkey_index].items.push(worry_level);
            }
        }
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut monkey_cage = get_sample_input();
        for _ in 0..20 {
            monkey_cage.tick();
        }
        let mut inspect_count = monkey_cage
            .monkeys
            .iter()
            .map(|m| m.inspect_count)
            .collect::<Vec<_>>();
        inspect_count.sort();
        let ans = inspect_count.iter().rev().take(2).fold(1, |acc, x| acc * x);
        assert_eq!(ans, 10605)
    }
}
