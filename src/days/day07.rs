use std::collections::HashMap;

use super::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn solve_part_1(input: String) -> String {
        String::new()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[derive(Debug)]
enum Node {
    File(usize),
    Dir {
        size: Option<usize>,
        children: HashMap<String, usize>,
        parent: Option<usize>,
    },
}
#[derive(Debug)]
struct FileSystem {
    cursor: usize,
    nodes: Vec<Node>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            cursor: 0,
            nodes: vec![Node::Dir {
                parent: None,
                children: HashMap::new(),
                size: None,
            }],
        }
    }

    fn cd(&mut self, dir: &str) {
        let curr = &self.nodes[self.cursor];
        match curr {
            Node::File(_) => {}
            Node::Dir {
                parent, children, ..
            } => match dir {
                ".." => {
                    self.cursor = parent.unwrap();
                }
                name => {
                    self.cursor = children[name];
                }
            },
        }
    }

    fn ls(&mut self, words: &[&str]) {
        let new_node = match words[0] {
            "dir" => Node::Dir {
                parent: Some(self.cursor),
                children: HashMap::new(),
                size: None,
            },
            size => Node::File(size.parse().unwrap()),
        };
        let l = self.nodes.len();
        if let Node::Dir { children, .. } = &mut self.nodes[self.cursor] {
            children.insert(words[1].to_string(), l);
        }
        self.nodes.push(new_node);
    }

    fn init(&mut self, input: String) {
        for line in input.lines().skip(1) {
            let words: Vec<&str> = line.split_whitespace().collect();
            match (words[0], words[1]) {
                ("$", "cd") => self.cd(words[2]),
                ("$", "ls") => {}
                _ => self.ls(&words),
            }
        }
    }
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from(
            "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        ",
        );
        let input = input
            .lines()
            .map(|l| l.trim())
            .collect::<Vec<_>>()
            .join("\n");
        let mut fs = FileSystem::new();
        fs.init(input);
        println!("{:?}", fs);
        assert_eq!(Day07::solve_part_1("".to_string()), "".to_string());
    }
    #[test]
    fn test_part_2() {
        assert_eq!(Day07::solve_part_2("".to_string()), "".to_string());
    }
}
