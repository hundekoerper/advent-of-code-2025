use std::fs;

#[derive(Debug, Clone)]
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

pub fn solve() {
    let puzzle_input =fs::read_to_string("src/input.txt").unwrap();
    let puzzle_input = puzzle_input.lines().collect::<Vec<_>>();
    let puzzle_input = puzzle_input.iter().map(|l| l.chars().rev().collect::<Vec<_>>()).collect::<Vec<_>>();
    
    let mut cephalopod_inputs: Vec<Problem> = Vec::new();
    let mut current_group: Vec<u64> = Vec::new();
    let mut current_operation: Option<Operation> = None;
    
    for index in 0..puzzle_input[0].len() {
        let mut digits = Vec::new();
        
        for line in &puzzle_input {
            match line[index] {
                ' ' => continue,
                '+' => { current_operation = Some(Operation::Add); },
                '*' => { current_operation = Some(Operation::Multiply); },
                value => { digits.push(value); }
            }
        }
        
        if index == puzzle_input[0].len() - 1  {
            let number = digits.iter().collect::<String>().parse::<u64>().unwrap();
            current_group.push(number);
            let problem = Problem {
                values: current_group.clone(),
                operation: current_operation.clone().unwrap()
            };
            cephalopod_inputs.push(problem);
        } 
        
        if digits.is_empty() {
            let problem = Problem {
                values: current_group.clone(),
                operation: current_operation.clone().unwrap()
            };
            cephalopod_inputs.push(problem);
            current_group = Vec::new();
            current_operation = None;
        } else {
            let number = digits.iter().collect::<String>().parse::<u64>().unwrap();
            current_group.push(number);
        }
    }
        
    let solutions: Vec<u64> = cephalopod_inputs.iter().map(|p| p.solve()).collect();
    let answer = solutions.iter().sum::<u64>();
    
    println!("voila {:?}", answer);
}
