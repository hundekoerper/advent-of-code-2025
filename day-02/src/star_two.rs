use std::fs;

struct IdRange {
    start: u64,
    end: u64,
    invalid_ids: Vec<u64>,
}

impl IdRange {
    pub fn find_invalid_ids(&mut self) {
        for id in self.start..self.end + 1 {
            let id_string = id.to_string();
            let doubled = format!("{id_string}{id_string}");
                
            let slice = &doubled[1..doubled.len() - 1];
            let is_invalid = slice.contains(&id_string);

            if is_invalid {
                self.invalid_ids.push(id);
            }
        }
    }
}

impl From<&str> for IdRange {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        if let &[start, end] = &*parts {
            Self {
                start: start.parse::<u64>().unwrap(),
                end: end.parse::<u64>().unwrap(),
                invalid_ids: Vec::new(),
            }
        } else {
            panic!("invalid range format");
        }
    }
}

pub fn solve() {
    let mut answer = Vec::new();
    for range in fs::read_to_string("src/input.txt").unwrap().split([',']) {
        let mut result = IdRange::from(range.trim());
        result.find_invalid_ids();
        answer.extend(result.invalid_ids);
    }

    let answer = answer.iter().fold(0, |a ,b | a + b);
    
    println!("voila {:?}", answer);
}
