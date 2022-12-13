use super::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn solve_part_1(input: String) -> String {
        let mut pairs: Vec<Vec<Packet>> = vec![vec![]];
        for line in input.lines() {
            if line == "" {
                pairs.push(vec![]);
            } else {
                pairs.last_mut().unwrap().push(line.parse().unwrap());
            }
        }
        pairs
            .iter()
            .enumerate()
            .filter_map(|(ind, pair)| {
                let a = &pair[0];
                let b = &pair[1];
                if a <= b {
                    Some(ind + 1)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[derive(Debug)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl std::str::FromStr for Packet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = s.len();

        match s.chars().next().unwrap() {
            '[' => {
                let sub_packet = Self::sub_packet(&s[1..l - 1]);
                Ok(Self::List(sub_packet))
            }
            _ => Ok(Self::Integer(s.parse().unwrap())),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => {
                let l = a.len().min(b.len());
                for i in 0..l {
                    let cmp = a[i].partial_cmp(&b[i]);
                    if cmp != Some(std::cmp::Ordering::Equal) {
                        return cmp;
                    }
                }
                a.len().partial_cmp(&b.len())
            }
            (Self::Integer(a), Self::List(_)) => {
                Self::List(vec![Self::Integer(*a)]).partial_cmp(other)
            }
            (Self::List(_), Self::Integer(b)) => {
                self.partial_cmp(&Self::List(vec![Self::Integer(*b)]))
            }
        }
    }
}

impl Packet {
    fn sub_packet(s: &str) -> Vec<Packet> {
        let mut ans = vec![];

        let mut sub_num = String::new();
        let mut sub_packet = String::new();

        let mut ind = 0;
        let mut chars: Vec<char> = s.chars().collect();
        chars.push(',');
        let l = chars.len();
        while ind < l {
            let ch = chars[ind];
            match ch {
                '[' => {
                    let mut depth = 1;
                    sub_packet.push(ch);
                    while depth > 0 {
                        ind += 1;
                        let ch = chars[ind];
                        match ch {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                        sub_packet.push(ch);
                    }
                }
                ',' => {
                    if !sub_packet.is_empty() {
                        ans.push(sub_packet.parse().unwrap());
                        sub_packet = String::new();
                    } else if !sub_num.is_empty() {
                        ans.push(sub_num.parse().unwrap());
                        sub_num = String::new();
                    }
                }
                _ => {
                    sub_num.push(ch);
                }
            }
            ind += 1;
        }

        ans
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    fn get_sample_input() -> String {
        String::from(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        )
    }
    #[test]
    fn part_1() {
        let input = get_sample_input();
        assert_eq!(Day13::solve_part_1(input), "13");
    }

    #[test]
    fn partial_ord() {
        assert_eq!(2.partial_cmp(&3), Some(std::cmp::Ordering::Less));
    }
    #[test]
    fn part_2() {
        assert_eq!(Day13::solve_part_2(String::new()), String::new());
    }
}
