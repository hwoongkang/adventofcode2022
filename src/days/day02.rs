use super::Solution;

use std::str::FromStr;

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

enum Result {
    Win,
    Lose,
    Draw,
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
    fn solve_part_1(input_file_name: &str) -> String {
        let mut ans = 0;
        let input = Self::read_input(input_file_name);
        for line in input.lines() {
            let mut words = line.split_whitespace();
            let opponent: RSP = words.next().unwrap().parse().unwrap();
            let me: RSP = words.next().unwrap().parse().unwrap();
            let result = get_result(&opponent, &me);
            ans += get_score(&me, &result);
        }
        ans.to_string()
    }
    fn solve_part_2(input_file_name: &str) -> String {
        unimplemented!("")
    }
}
