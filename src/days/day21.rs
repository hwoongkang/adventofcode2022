use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Solution;

pub struct Day21;

impl Solution for Day21 {
    fn solve_part_1(input: String) -> String {
        let tree = MonkeyTree::from(&input);
        tree.root.unwrap().borrow().evaluate().to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::new()
    }
}
#[derive(Debug)]
enum Op {
    Plus,
    Times,
    Minus,
    Div,
}

type Link<T> = Option<Rc<RefCell<T>>>;
#[derive(Debug)]
struct Monkey {
    op: Option<Op>,
    val: Option<i64>,
    left: Link<Monkey>,
    right: Link<Monkey>,
}

impl Monkey {
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn evaluate(&self) -> i64 {
        if self.is_leaf() {
            self.val.unwrap()
        } else {
            let left = self.left.as_ref().unwrap().borrow().evaluate();
            let right = self.right.as_ref().unwrap().borrow().evaluate();

            match self.op.as_ref().unwrap() {
                Op::Plus => left + right,
                Op::Times => left * right,
                Op::Minus => left - right,
                Op::Div => left / right,
            }
        }
    }
}

#[derive(Debug)]
struct MonkeyTree {
    root: Link<Monkey>,
}

impl MonkeyTree {
    fn from(input: &str) -> Self {
        let monkeys: HashMap<String, (Rc<RefCell<Monkey>>, Option<(String, String)>)> = input
            .lines()
            .map(|l| {
                let words: Vec<&str> = l.split_whitespace().collect();
                let name = words[0][0..4].to_string();
                let (op, val) = if words.len() == 2 {
                    (None, Some(words[1].parse().unwrap()))
                } else {
                    let op = match words[2] {
                        "+" => Some(Op::Plus),
                        "*" => Some(Op::Times),
                        "-" => Some(Op::Minus),
                        "/" => Some(Op::Div),
                        _ => None,
                    };
                    (op, None)
                };
                let monkey = Monkey {
                    op,
                    val,
                    left: None,
                    right: None,
                };
                let children: Option<(String, String)> = if words.len() == 2 {
                    None
                } else {
                    Some((words[1][0..4].to_string(), words[3][0..4].to_string()))
                };
                (name, (Rc::new(RefCell::new(monkey)), children))
            })
            .collect();
        let mut new_monkeys = monkeys.clone();
        for (monkey, children) in new_monkeys.values_mut() {
            if let Some((left, right)) = children {
                monkey.borrow_mut().left = Some(monkeys[left].0.clone());
                monkey.borrow_mut().right = Some(monkeys[right].0.clone());
            }
        }
        Self {
            root: Some(monkeys.get("root").unwrap().0.clone()),
        }
    }
}
#[cfg(test)]
mod day21_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
        )
    }

    #[test]
    fn part1() {
        let input = get_sample_input();
        let ans = Day21::solve_part_1(input);
        let expected = "152";
        assert_eq!(ans, expected);
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day21::solve_part_2(input);
        let expected = "";
        assert_eq!(ans, expected);
    }
}
