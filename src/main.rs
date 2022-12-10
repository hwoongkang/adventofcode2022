mod days;
use days::*;

type Today = Day10;

fn main() {
    let input = read_input("input.txt");
    println!("Part 1: {}", Today::solve_part_1(input.clone()));
    println!("Part 2: {}", Today::solve_part_2(input));
}
