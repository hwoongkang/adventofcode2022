mod days;
use days::*;

fn main() {
    let input = read_input("input.txt");
    println!("Part 1: {}", Day04::solve_part_1(input.clone()));
    println!("Part 2: {}", Day04::solve_part_2(input));
}
