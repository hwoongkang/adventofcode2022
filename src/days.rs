use std::fs;
pub trait Solution {
    fn read_input(input_file_name: &str) -> String {
        fs::read_to_string(input_file_name).expect("Input Error")
    }
    fn solve_part_1(input_file_name: &str) -> String;
    fn solve_part_2(input_file_name: &str) -> String;
}

mod day_2021_1;
pub use day_2021_1::*;
mod day_2021_25;
pub use day_2021_25::*;

mod day01;
pub use day01::*;
mod day02;
pub use day02::*;
