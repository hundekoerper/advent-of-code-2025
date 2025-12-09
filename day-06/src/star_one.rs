use std::fs;

#[derive(Debug)]
enum Operation {
    Add,
    Multiply
}

#[derive(Debug)]
struct Problem {
    values: Vec<u64>,
    operation: Operation
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.values.iter().sum(),
            Operation::Multiply => self.values.iter().product()
        }
    }
}

impl From<Vec<&str>> for Problem {
    fn from(value: Vec<&str>) -> Self {
        let operation = match value[0] {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Unknown operation")
        };
        let values = value[1..]
            .iter()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Problem {
            values,
            operation
        }
    }
}

pub fn solve() {
    let mut input: Vec<Vec<&str>> = Vec::new();
    let puzzle_input = fs::read_to_string("src/input.txt").unwrap();
    for line in puzzle_input.lines() {
        let values = line.split_whitespace().collect::<Vec<&str>>();
        input.push(values);
    }
    let mut problems: Vec<Problem> = Vec::new();
    let input_len = input[0].len();
    for index in 0..input_len {
        let mut problem_values = input.iter().map(|line| line[index]).collect::<Vec<_>>();
        problem_values.reverse();
        let problem = Problem::from(problem_values);
        problems.push(problem);
    }
    
    let solutions: Vec<u64> = problems.iter().map(|p| p.solve()).collect();
    let answer = solutions.iter().sum::<u64>();
    
    println!("voila {:?}", answer);
}