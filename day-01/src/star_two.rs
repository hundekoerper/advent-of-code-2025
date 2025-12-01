use std::fs;

#[derive(Debug)]
enum Turn {
    Left(i32),
    Right(i32)
}

impl From<&str> for Turn {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        match chars.next() {
            Some('L') => {
                let value = chars.as_str().parse::<i32>().unwrap();
                Turn::Left(value)
            },
            Some('R') => {
                let value = chars.as_str().parse::<i32>().unwrap();
                Turn::Right(value)
            },
            _ => panic!("Invalid direction"),
        }
    }
}

struct Dial(i32);

impl Dial {
    pub fn turn(&mut self, direction: Turn) {
        match direction {
            Turn::Left(steps) => {
                self.0 = (self.0 + 100 - (steps % 100)) % 100;
            },
            Turn::Right(steps) => {
                self.0 = (self.0 + (steps % 100)) % 100;
            }
        }
    }
}

pub fn solve() {
    let mut answer: usize = 0;
    let mut dial = Dial(50);
    
    for line in fs::read_to_string("src/input.txt").unwrap().lines() {
        let turn = Turn::from(line);
        let passed_zero = match turn {
            Turn::Right(steps) => {
                (steps + dial.0) / 100
            }
            Turn::Left(steps) => {
                let result = (dial.0 - 100 - steps) / -100;
                // (◞‸ ◟ ；)
                if result > 0 && dial.0 == 0 {
                    result - 1
                } else {
                    result
                }
            }
        };
        dial.turn(Turn::from(line));
        answer += passed_zero as usize;
    }
    
    println!("voila {answer}");
}