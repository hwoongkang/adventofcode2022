mod days;
use days::*;

fn main() {
    let input = read_input("input.txt");
    println!("Part 1: {}", Day07::solve_part_1(input.clone()));
    println!("Part 2: {}", Day07::solve_part_2(input));
}
