use super::Solution;

pub struct Day20;

impl Solution for Day20 {
    fn solve_part_1(input: String) -> String {
        let mut dll = DoublyLinkedList::from(&input);
        for i in 0..dll.nodes.len() {
            dll.tick(i);
        }
        dll.ans().to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut dll = DoublyLinkedList::from(&input);
        dll.nodes = dll
            .nodes
            .iter()
            .map(|n| Node {
                value: n.value * 811589153,
                ..*n
            })
            .collect();
        for _ in 0..10 {
            for i in 0..dll.nodes.len() {
                dll.tick(i);
            }
        }
        dll.ans().to_string()
    }
}

struct Node {
    next: usize,
    prev: usize,
    value: i64,
}

struct DoublyLinkedList {
    nodes: Vec<Node>,
}

impl DoublyLinkedList {
    fn from(input: &str) -> Self {
        let mut nodes: Vec<Node> = input
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .map(|n| Node {
                next: 0,
                prev: 0,
                value: n,
            })
            .collect();
        let l = nodes.len();
        for j in 0..l {
            let i = (j + l - 1) % l;
            let k = (j + 1) % l;
            nodes[j].next = k;
            nodes[j].prev = i;
        }
        Self { nodes }
    }

    fn tick(&mut self, at: usize) {
        let j = at;
        let l = self.nodes.len();

        match self.nodes[at].value.signum() {
            0 => {}
            1 => {
                let num_to_move = self.nodes[at].value as usize % (l - 1);
                for _ in 0..num_to_move {
                    // i j k l
                    // becomes
                    // i k j l
                    let i = self.nodes[j].prev;
                    let k = self.nodes[j].next;
                    let l = self.nodes[k].next;

                    self.nodes[i].next = k;
                    self.nodes[k].next = j;
                    self.nodes[j].next = l;

                    self.nodes[l].prev = j;
                    self.nodes[j].prev = k;
                    self.nodes[k].prev = i;
                }
            }
            -1 => {
                let num_to_move = self.nodes[at].value.abs() as usize % (l - 1);
                for _ in 0..num_to_move {
                    // h i j k
                    // becomes
                    // h j i k
                    let i = self.nodes[j].prev;
                    let h = self.nodes[i].prev;
                    let k = self.nodes[j].next;

                    self.nodes[h].next = j;
                    self.nodes[j].next = i;
                    self.nodes[i].next = k;

                    self.nodes[k].prev = i;
                    self.nodes[i].prev = j;
                    self.nodes[j].prev = h;
                }
            }
            _ => unreachable!(),
        }
    }

    fn ans(&self) -> i64 {
        let mut ans = 0;
        let mut head = self.nodes.iter().position(|n| n.value == 0).unwrap();
        for i in 1..3001 {
            head = self.nodes[head].next;
            if i % 1000 == 0 {
                ans += self.nodes[head].value;
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
