use super::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn solve_part_1(input: String) -> String {
        let mut clock_circuit = ClockCircuit::new();
        let mut ans = 0;
        for line in input.lines() {
            let command = line.parse::<Command>().unwrap();
            if let Some(result) = clock_circuit.exec(&command) {
                ans += result;
            }
        }
        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut crt = CRT::new();
        for line in input.lines() {
            let command = line.parse::<Command>().unwrap();
            crt.exec(&command);
        }
        crt.show();
        String::new()
    }
}

struct CRT {
    cycle: i32,
    register: i32,
    display: Vec<bool>,
}

impl CRT {
    fn new() -> Self {
        Self {
            cycle: 0,
            register: 1,
            display: vec![false; 240],
        }
    }
    fn show(&self) {
        for i in 0..240 {
            print!("{}", if self.display[i] { "#" } else { "." });
            if i % 40 == 39 {
                println!();
            }
        }
    }
    fn draw(&mut self) {
        let cycle = self.cycle % 40;
        println!("{}, {}, {}", cycle, self.register, cycle - self.register);
        if (cycle - self.register).abs() <= 1 {
            self.display[self.cycle as usize] = true;
        }
    }
    fn exec(&mut self, command: &Command) {
        match command {
            Command::Noop => {
                self.draw();
                self.cycle += 1;
            }
            Command::Addx(val) => {
                self.draw();
                self.cycle += 1;
                self.draw();
                self.cycle += 1;
                self.register += val;
            }
        }
    }
}

struct ClockCircuit {
    cycle: i32,
    register: i32,
}

impl ClockCircuit {
    fn new() -> Self {
        Self {
            cycle: 0,
            register: 1,
        }
    }

    fn exec(&mut self, command: &Command) -> Option<i32> {
        let mut ans = None;
        match command {
            Command::Noop => {
                self.cycle += 1;
                if self.cycle % 40 == 20 {
                    ans = Some(self.cycle * self.register);
                }
            }
            Command::Addx(val) => {
                for _ in 0..2 {
                    self.cycle += 1;
                    if self.cycle % 40 == 20 {
                        ans = Some(self.cycle * self.register);
                    }
                }
                self.register += val;
            }
        }

        ans
    }
}

enum Command {
    Addx(i32),
    Noop,
}

impl std::str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command = parts.next().unwrap();
        match command {
            "addx" => Ok(Self::Addx(parts.next().unwrap().parse().unwrap())),
            "noop" => Ok(Self::Noop),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_sample_input();

        let expected = String::from("13140");
        assert_eq!(Day10::solve_part_1(input), expected);
    }

    #[test]
    fn test_part_2() {
        let input = get_sample_input();

        Day10::solve_part_2(input);
    }

    fn get_sample_input() -> String {
        String::from(
            "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop",
        )
    }
}
