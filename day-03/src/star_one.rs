use std::fs;

fn find_max(values: &Vec<u32>) -> (u32, usize) {
    let mut max_value = 0;
    let mut max_index = 0;
    for (index, &value) in values.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = index
        }
    }
    
    (max_value, max_index)
}

fn calculate_joltage(battery_bank: &str) -> u32 {
    let mut batteries = battery_bank.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let last_battery = batteries.pop().unwrap();
    
    let (first_battery, first_battery_index) = find_max(&batteries);
    
    let mut remaining_batteries = batteries.iter().skip(first_battery_index + 1).cloned().collect::<Vec<u32>>();
    remaining_batteries.push(last_battery);
    let (second_battery, _) = find_max(&remaining_batteries);
    
    format!("{first_battery}{second_battery}").parse().unwrap()
}

pub fn solve() {
    let mut answer = Vec::new();
    for battery_banks in fs::read_to_string("src/input.txt").unwrap().lines() {
        let joltage = calculate_joltage(battery_banks);
        answer.push(joltage);
    }
    let answer = answer.iter().fold(0, |a ,b | a + b);
    
    println!("voila {:?}", answer);
}
