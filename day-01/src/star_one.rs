use std::fs;

enum Turn {
    Left(u32),
    Right(u32)
}

impl From<&str> for Turn {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        match chars.next() {
            Some('L') => {
                let value = chars.as_str().parse::<u32>().unwrap();
                Turn::Left(value)
            },
            Some('R') => {
                let value = chars.as_str().parse::<u32>().unwrap();
                Turn::Right(value)
            },
            _ => panic!("Invalid direction"),
        }
    }
}

struct Dial(u32);

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
        dial.turn(Turn::from(line));
        if let 0 = dial.0 {
            answer += 1;
        }
    }
    
    println!("voila {answer}");
}