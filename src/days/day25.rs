use super::Solution;

pub struct Day25;

impl Solution for Day25 {
    fn solve_part_1(input: String) -> String {
        input
            .lines()
            .map(|l| l.parse::<Snafu>().unwrap())
            .sum::<Snafu>()
            .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("There is no part 2 for this day!")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Snafu(Vec<i32>);

impl Snafu {
    fn _from(decimal: i32) -> Self {
        let mut num = decimal;
        let mut pow = (5, 1);
        let mut ans = vec![];
        while num > 0 {
            let mut digit = (num % pow.0) / pow.1;
            if digit >= 3 {
                digit -= 5;
            }
            ans.push(digit);
            num -= digit * pow.1;
            pow = (pow.0 * 5, pow.1 * 5);
        }
        Self(ans.into_iter().rev().collect())
    }
    fn _to_decimal(&self) -> i32 {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(pow, digit)| digit * 5i32.pow(pow as u32))
            .sum()
    }
}

impl std::ops::Add for Snafu {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut carry = 0;
        let me: Vec<i32> = self.0.into_iter().rev().collect();
        let rhs: Vec<i32> = rhs.0.into_iter().rev().collect();

        let mut ans = vec![];

        for i in 0..(me.len().max(rhs.len())) {
            let mut sum = carry;
            if i < me.len() {
                sum += me[i];
            }
            if i < rhs.len() {
                sum += rhs[i];
            }

            match sum {
                -2..=2 => {
                    carry = 0;
                }
                3.. => {
                    carry = 1;
                    sum -= 5;
                }
                _ => {
                    carry = -1;
                    sum += 5;
                }
            }

            ans.push(sum);
        }

        if carry != 0 {
            ans.push(carry);
        }

        Self(ans.into_iter().rev().collect())
    }
}

impl std::iter::Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Snafu(vec![]), |acc, x| acc + x)
    }
}

impl std::str::FromStr for Snafu {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|ch| match ch {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => unreachable!(),
                })
                .collect(),
        ))
    }
}

impl std::string::ToString for Snafu {
    fn to_string(&self) -> String {
        let mut ans = String::new();
        for num in self.0.iter() {
            let ch = match num {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            };
            ans.push(ch);
        }
        ans
    }
}

#[cfg(test)]
mod day25_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122",
        )
    }

    #[test]
    fn snafu_to_decimal() {
        let snafu = Snafu(vec![1, -2, 1, 1, -1, 2]);
        let num = 2022;
        assert_eq!(snafu._to_decimal(), num);
    }

    #[test]
    fn decimal_to_snafu() {
        let snafu = Snafu(vec![1, -2, 1, 1, -1, 2]);
        let num = 2022;
        assert_eq!(Snafu::_from(num), snafu);
    }

    #[test]
    fn snafu_add() {
        let first = Snafu::_from(17);
        let second = Snafu::_from(81);
        let ans = Snafu::_from(98);
        assert_eq!(first + second, ans);
    }

    #[test]
    fn snafu_stringify() {
        let snafu = Snafu(vec![1, -2, 1, 1, -1, 2]);
        assert_eq!(snafu.to_string(), "1=11-2")
    }

    #[test]
    fn part1() {
        let input = get_sample_input();
        let ans = Day25::solve_part_1(input);
        let expected = "2=-1=0";
        assert_eq!(ans, expected);
    }

    #[test]
    fn part2() {
        let input = get_sample_input();
        let ans = Day25::solve_part_2(input);
        let expected = String::from("There is no part 2 for this day!");
        assert_eq!(ans, expected);
    }
}
