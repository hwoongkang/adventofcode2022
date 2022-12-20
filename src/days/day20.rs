use super::Solution;

pub struct Day20;

impl Solution for Day20 {
    fn solve_part_1(input: String) -> String {
        let mut dll = DoublyLinkedList2::from(&input, 1);

        dll.mix();

        dll.ans().to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut dll = DoublyLinkedList2::from(&input, 811589153);

        for _ in 0..10 {
            dll.mix();
        }

        dll.ans().to_string()
    }
}

#[derive(Debug)]
struct Node {
    prev: usize,
    next: usize,
    value: i64,
}

struct DoublyLinkedList2 {
    nums: Vec<i64>,
    indices: Vec<usize>,
}

impl DoublyLinkedList2 {
    fn from(input: &str, key: i64) -> Self {
        let nums: Vec<i64> = input
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .map(|n| n * key)
            .collect();
        let indices = (0..nums.len()).collect();
        Self { nums, indices }
    }

    fn mix(&mut self) {
        let l = self.indices.len();
        for i in 0..l {
            println!("{}, {:?}", i, self.indices);
            let j = self.indices.iter().position(|&n| n == i).unwrap();

            self.indices.remove(j);

            let num = self.nums[i];

            let k = match num.signum() {
                1 => (j + num as usize) % l,
                -1 => (j + 10 * l - (num.abs() as usize) % l) % l,
                _ => j,
            };

            self.indices.insert(k, i);
        }
    }

    fn ans(&self) -> i64 {
        let index_zero = self.nums.iter().position(|&n| n == 0).unwrap();
        let zero = self.indices.iter().position(|&i| i == index_zero).unwrap();
        (1..=3)
            .map(|p| {
                let i = (zero + p) % self.nums.len();
                self.nums[self.indices[i]]
            })
            .sum()
    }
}

#[derive(Debug)]
struct DoublyLinkedList {
    nodes: Vec<Node>,
}

impl DoublyLinkedList {
    fn from(input: &str) -> Self {
        let mut nodes: Vec<Node> = input
            .lines()
            .map(|l| l.parse().unwrap())
            .map(|value| Node {
                prev: 0,
                next: 0,
                value,
            })
            .collect();
        let l = nodes.len();
        for j in 0..nodes.len() {
            let i = (j + l - 1) % l;
            let k = (j + 1) % l;
            nodes[j].prev = i;
            nodes[j].next = k;
        }
        Self { nodes }
    }

    fn _print(&self) -> String {
        let mut nums: Vec<i64> = vec![];
        let mut head = 0;
        let returned = head;
        loop {
            nums.push(self.nodes[head].value);
            head = self.nodes[head].next;
            if head == returned {
                break;
            }
        }

        nums.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn tick(&mut self, at: usize) {
        let head = at;
        let val = self.nodes[head].value;
        match val.signum() {
            1 => {
                for _ in 0..val {
                    let j = head;
                    let i = self.nodes[j].prev;
                    let k = self.nodes[j].next;
                    let l = self.nodes[k].next;

                    // i j k l
                    // becomes
                    // i k j l

                    self.nodes[k].prev = i;
                    self.nodes[j].prev = k;
                    self.nodes[l].prev = j;

                    self.nodes[i].next = k;
                    self.nodes[k].next = j;
                    self.nodes[j].next = l;
                }
            }
            -1 => {
                for _ in 0..(-val) {
                    let j = head;
                    let i = self.nodes[j].prev;
                    let h = self.nodes[i].prev;
                    let k = self.nodes[j].next;

                    // h i j k
                    // becomes
                    // h j i k

                    self.nodes[j].prev = h;
                    self.nodes[i].prev = j;
                    self.nodes[k].prev = i;

                    self.nodes[h].next = j;
                    self.nodes[j].next = i;
                    self.nodes[i].next = k;
                }
            }
            0 => {}
            _ => unreachable!(),
        }
    }

    fn part_1(&self) -> i64 {
        let mut cursor = self.nodes.iter().position(|n| n.value == 0).unwrap();
        let mut ans = 0;

        for i in 1..3001 {
            cursor = self.nodes[cursor].next;

            if i % 1000 == 0 {
                println!("{}", self.nodes[cursor].value);
                ans += self.nodes[cursor].value;
            }
        }
        ans
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
