use super::Solution;

use std::str::FromStr;

#[derive(Copy, Clone)]
enum RSP {
    Rock,
    Paper,
    Scissors,
}
impl FromStr for RSP {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<RSP, ()> {
        match s {
            "A" | "X" => Ok(RSP::Rock),
            "B" | "Y" => Ok(RSP::Paper),
            "C" | "Z" => Ok(RSP::Scissors),
            _ => Err(()),
        }
    }
}

impl RSP {
    fn get_strategy(&self, result: &Result) -> Self {
        match (self, result) {
            (RSP::Rock, Result::Win) => RSP::Paper,
            (RSP::Rock, Result::Lose) => RSP::Scissors,
            (RSP::Paper, Result::Win) => RSP::Scissors,
            (RSP::Paper, Result::Lose) => RSP::Rock,
            (RSP::Scissors, Result::Win) => RSP::Rock,
            (RSP::Scissors, Result::Lose) => RSP::Paper,
            _ => self.clone(),
        }
    }
}

enum Result {
    Win,
    Lose,
    Draw,
}

impl FromStr for Result {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Result, ()> {
        match s {
            "X" => Ok(Result::Lose),
            "Y" => Ok(Result::Draw),
            "Z" => Ok(Result::Win),
            _ => Err(()),
        }
    }
}

fn get_score(rsp: &RSP, result: &Result) -> u32 {
    let rsp_score: u32 = match rsp {
        RSP::Rock => 1,
        RSP::Paper => 2,
        RSP::Scissors => 3,
    };
    let result_score: u32 = match result {
        Result::Win => 6,
        Result::Lose => 0,
        Result::Draw => 3,
    };
    rsp_score + result_score
}

fn get_result(opponent: &RSP, me: &RSP) -> Result {
    match (opponent, me) {
        (RSP::Rock, RSP::Rock) => Result::Draw,
        (RSP::Rock, RSP::Paper) => Result::Win,
        (RSP::Rock, RSP::Scissors) => Result::Lose,
        (RSP::Paper, RSP::Rock) => Result::Lose,
        (RSP::Paper, RSP::Paper) => Result::Draw,
        (RSP::Paper, RSP::Scissors) => Result::Win,
        (RSP::Scissors, RSP::Rock) => Result::Win,
        (RSP::Scissors, RSP::Paper) => Result::Lose,
        (RSP::Scissors, RSP::Scissors) => Result::Draw,
    }
}

pub struct Day02 {}

impl Solution for Day02 {
    fn solve_part_1(input: String) -> String {
        let mut ans = 0;

        for line in input.lines() {
            let mut words = line.split_whitespace();
            let opponent: RSP = words.next().unwrap().parse().unwrap();
            let me: RSP = words.next().unwrap().parse().unwrap();
            let result = get_result(&opponent, &me);
            ans += get_score(&me, &result);
        }
        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut ans = 0;

        for line in input.lines() {
            let mut words = line.split_whitespace();
            let opponent: RSP = words.next().unwrap().parse().unwrap();
            let strategy: Result = words.next().unwrap().parse().unwrap();
            let me = opponent.get_strategy(&strategy);
            let score = get_score(&me, &strategy);
            ans += score;
        }
        ans.to_string()
    }
}
