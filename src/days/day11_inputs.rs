use super::day11::{Monkey, MonkeyCage};
pub fn get_input() -> MonkeyCage {
    MonkeyCage {
        monkeys: vec![
            // Form INPUT_STRING to vec of monkeys
            Monkey::new(vec![83, 62, 93], |x| x * 17, 2, 1, 6),
            Monkey::new(vec![90, 55], |x| x + 1, 17, 6, 3),
            Monkey::new(vec![91, 78, 80, 97, 79, 88], |x| x + 3, 19, 7, 5),
            Monkey::new(vec![64, 80, 83, 89, 59], |x| x + 5, 3, 7, 2),
            Monkey::new(vec![98, 92, 99, 51], |x| x * x, 5, 0, 1),
            Monkey::new(vec![68, 57, 95, 85, 98, 75, 98, 75], |x| x + 2, 13, 4, 0),
            Monkey::new(vec![74], |x| x + 4, 7, 3, 2),
            Monkey::new(vec![68, 64, 60, 68, 87, 80, 82], |x| x * 19, 11, 4, 5),
        ],
    }
}

pub fn get_sample_input() -> MonkeyCage {
    MonkeyCage {
        monkeys: vec![
            Monkey::new(vec![79, 98], |x| x * 19, 23, 2, 3),
            Monkey::new(vec![54, 65, 75, 74], |x| x + 6, 19, 2, 0),
            Monkey::new(vec![79, 60, 97], |x| x * x, 13, 1, 3),
            Monkey::new(vec![74], |x| x + 3, 17, 0, 1),
        ],
    }
}
