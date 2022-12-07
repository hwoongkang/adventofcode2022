use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use super::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn solve_part_1(input: String) -> String {
        let mut fs = FileSystem::from(input);
        let ans: usize = fs.get_sizes().iter().filter(|&&size| size <= 100_000).sum();
        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}
#[derive(Debug)]
enum Node {
    Dir {
        size: Option<usize>,
        parent: Option<Weak<RefCell<Node>>>,
        children: HashMap<String, Rc<RefCell<Node>>>,
    },
    File(usize),
}

impl Node {
    fn get_size(&mut self) -> usize {
        match self {
            Node::File(size) => *size,
            Node::Dir { size, children, .. } => {
                if let Some(size) = size {
                    *size
                } else {
                    let calculated_size: usize = children
                        .values()
                        .map(|child| child.borrow_mut().get_size())
                        .sum();
                    *size = Some(calculated_size);
                    calculated_size
                }
            }
        }
    }
}
#[derive(Debug)]
struct FileSystem {
    root: Rc<RefCell<Node>>,
    head: Weak<RefCell<Node>>,
}

impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Node::Dir {
            size: None,
            parent: None,
            children: HashMap::new(),
        }));
        let head = Rc::downgrade(&root);
        Self { root, head }
    }

    fn from(input: String) -> Self {
        let mut fs = Self::new();
        for line in input.lines().skip(1) {
            let cmd: Vec<&str> = line.split_whitespace().collect();
            match (cmd[0], cmd[1]) {
                ("$", "cd") => {
                    fs.cd(cmd[2]);
                }
                ("$", "ls") => {}
                _ => fs.ls((cmd[0], cmd[1])),
            }
        }
        fs
    }
    fn cd(&mut self, dir: &str) {
        let new_head = match &*self.head.upgrade().unwrap().borrow() {
            Node::Dir {
                children, parent, ..
            } => match dir {
                ".." => parent.clone(),
                _ => Some(Rc::downgrade(&children.get(dir).unwrap())),
            },
            Node::File(_) => None,
        };
        if let Some(new_head) = new_head {
            self.head = new_head;
        }
    }
    fn ls(&mut self, cmd: (&str, &str)) {
        let new_node = match cmd.0 {
            "dir" => Node::Dir {
                size: None,
                parent: Some(self.head.clone()),
                children: HashMap::new(),
            },
            size => Node::File(size.parse().unwrap()),
        };
        match &mut *self.head.upgrade().unwrap().borrow_mut() {
            Node::Dir { children, .. } => {
                children.insert(cmd.1.to_string(), Rc::new(RefCell::new(new_node)));
            }
            Node::File(_) => {}
        }
    }

    fn get_sizes(&mut self) -> Vec<usize> {
        let mut sizes = vec![];
        self.root.borrow_mut().get_size();
        let mut stack = vec![self.root.clone()];
        while let Some(node) = stack.pop() {
            match &*node.borrow() {
                Node::Dir { size, children, .. } => {
                    sizes.push(size.unwrap());
                    for (_, next_node) in children.iter() {
                        stack.push(next_node.clone());
                    }
                }
                Node::File(_) => {
                    sizes.push(100_001);
                }
            }
        }
        sizes
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
        let ans = Day07::solve_part_1(input);
        assert_eq!(ans, 95437.to_string());
    }
    #[test]
    fn test_part_2() {
        assert_eq!(Day07::solve_part_2("".to_string()), "".to_string());
    }
}
