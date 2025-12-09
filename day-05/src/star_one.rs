use std::fs;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn is_fresh(&self, ingredient: u64) -> bool {
        ingredient >= self.start && ingredient <= self.end
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        if let &[start, end] = &*parts {
            Self {
                start: start.parse::<u64>().unwrap(),
                end: end.parse::<u64>().unwrap(),
            }
        } else {
            panic!("invalid range format");
        }
    }
}

pub fn solve() {
    let mut ranges: Vec<Range> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    
    for line in fs::read_to_string("src/input.txt").unwrap().lines() {
        if line.contains('-') {
            let range = Range::from(line);
            ranges.push(range);
        } else if !line.trim().is_empty() {
            let ingredient = line.parse::<u64>().unwrap();
            ingredients.push(ingredient);
        }
    }
    
    let answer = ingredients.iter().filter(|&&ingredient| {
        ranges.iter().any(|range| range.is_fresh(ingredient))
    }).collect::<Vec<_>>().len() as u64;
    
    println!("voila {:?}", answer);
}
