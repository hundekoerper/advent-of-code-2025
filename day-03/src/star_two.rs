use std::fs;

fn calculate_joltage(battery_bank: &str, iterations: usize) -> u64 {
    let mut current_batteries = battery_bank
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let mut winners: Vec<u32> = Vec::new();
    
    for _ in 0..iterations {
        let batteries_left = iterations - winners.len();
        let (highest_battery, index) = find_max(&current_batteries, batteries_left);
        winners.push(highest_battery);
        let remaining_batteries = current_batteries.split_off(index + 1);
        current_batteries = remaining_batteries;
    }
    
    winners
        .iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn find_max(values: &Vec<u32>, batteries_left: usize) -> (u32, usize) {
    let mut max_value = 0;
    let mut max_index = 0;
    for (index, &value) in values.iter().enumerate() {
        if value > max_value && index + batteries_left <= values.len() {
            max_value = value;
            max_index = index
        }
    }
    
    (max_value, max_index)
}

pub fn solve() {
    let mut answer = Vec::new();
    for battery_banks in fs::read_to_string("src/input.txt").unwrap().lines() {
        let joltage = calculate_joltage(battery_banks, 12);
        answer.push(joltage);
    }
    let answer = answer.iter().fold(0, |a, b| a + b);

    println!("voila {:?}", answer);
}
