use std::fs;

pub fn read_input(input_file_name: &str) -> String {
    fs::read_to_string(input_file_name).expect("Input Error")
}
pub trait Solution {
    fn solve_part_1(input: String) -> String;
    fn solve_part_2(input: String) -> String;
}

mod day_2021_1;
pub use day_2021_1::*;
mod day_2021_25;
pub use day_2021_25::*;

mod day01;
pub use day01::*;
mod day02;
pub use day02::*;
mod day03;
pub use day03::*;
mod day04;
pub use day04::*;
mod day05;
pub use day05::*;
mod day06;
pub use day06::*;
mod day07;
pub use day07::*;
mod day08;
pub use day08::*;
mod day09;
pub use day09::*;
mod day10;
pub use day10::*;
mod day11;
mod day11_inputs;
pub use day11::*;
mod day12;
pub use day12::*;
mod day13;
pub use day13::*;
mod day14;
pub use day14::*;
