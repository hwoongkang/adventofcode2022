mod days;
use days::*;

use std::time;

type Today = Day24;

fn with_elapsed_time<T>(f: &dyn Fn() -> T) -> (time::Duration, T) {
    let now = time::Instant::now();
    let ans = f();
    let elapsed = now.elapsed();
    (elapsed, ans)
}

fn main() {
    let input = read_input("input.txt");
    let (elapsed, ans) = with_elapsed_time(&|| Today::solve_part_1(input.clone()));
    println!("Part 1: {}", ans);
    println!("It took {} ms to solve part 1", elapsed.as_millis());
    println!();
    let (elapsed, ans) = with_elapsed_time(&|| Today::solve_part_2(input.clone()));
    println!("Part 2: {}", ans);
    println!("It took {} ms to solve part 2", elapsed.as_millis());
    println!();
}
