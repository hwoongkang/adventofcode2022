use super::Solution;

pub struct Day20;

impl Solution for Day20 {
    fn solve_part_1(input: String) -> String {
        solve(input, 1, 1)
    }

    fn solve_part_2(input: String) -> String {
        solve(input, 811589153, 10)
    }
}

fn solve(input: String, key: i64, iter: usize) -> String {
    let nums: Vec<i64> = input.lines().map(|n| n.parse().unwrap()).collect();
    let nums: Vec<i64> = nums.iter().map(|&n| n * key).collect();
    let mut indices: Vec<usize> = (0..nums.len()).collect();
    let l = nums.len();
    for _ in 0..iter {
        for i in 0..l {
            let curr_index = indices.iter().position(|&x| x == i).unwrap();
            indices.remove(curr_index);

            let num = nums[i];

            let new_index = match num.signum() {
                0 => curr_index,
                1 => (curr_index + num as usize) % (l - 1),
                -1 => rem(curr_index as i64 + num, l - 1),
                _ => unreachable!(),
            };

            indices.insert(new_index, i);
        }
    }
    let zero = nums.iter().position(|&x| x == 0).unwrap();
    let zero_index = indices.iter().position(|&x| x == zero).unwrap();
    (1..=3)
        .map(|n| nums[indices[(zero_index + n * 1000) % l]])
        .map(|n| {
            println!("{}", n);
            n
        })
        .sum::<i64>()
        .to_string()
}

fn rem(me: i64, other: usize) -> usize {
    let rem = me.rem_euclid(other as i64);
    if rem < 0 {
        (rem + other as i64) as usize
    } else {
        rem as usize
    }
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "1
2
-3
3
-2
0
4",
        )
    }
    #[test]
    fn modulo() {
        assert_eq!(rem(-21, 4), 3);
        assert_eq!(rem(-2, 7), 5);
    }

    #[test]
    fn part1() {
        let input = get_sample_input();
        let ans = Day20::solve_part_1(input);
        let expected = "3";
        assert_eq!(ans, expected);
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day20::solve_part_2(input);
        let expected = "1623178306";
        assert_eq!(ans, expected);
    }
}
