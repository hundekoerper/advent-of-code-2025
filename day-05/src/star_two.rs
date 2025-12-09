use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    start: u64,
    end: u64,
    fresh_ids: u64,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        if let &[start, end] = &*parts {
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            Self {
                start,
                end,
                fresh_ids: 0,
            }
        } else {
            panic!("invalid range format");
        }
    }
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut merged_ranges: Vec<Range> = Vec::new();
    merged_ranges.push(sorted_ranges[0].clone());
    
    for range in sorted_ranges.into_iter().skip(1) {
        let last_merged_range = merged_ranges.last_mut().unwrap();
        if range.start + 1 <= last_merged_range.end {
            last_merged_range.end = last_merged_range.end.max(range.end);
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
}

impl Range {
    pub fn count_fresh_ids(&mut self) {
        if self.end <= self.start {
            self.fresh_ids = 0;
            return;
        }
        self.fresh_ids = (self.end - self.start) + 1;
    }
}

pub fn solve() {
    let mut ranges: Vec<Range> = Vec::new();

    for line in fs::read_to_string("src/input.txt").unwrap().lines() {
        if line.contains('-') {
            let range = Range::from(line);
            ranges.push(range);
        }
    }
    
    let mut ranges = merge_ranges(ranges.clone());
    ranges.iter_mut().for_each(|r| r.count_fresh_ids());
    
    let answer = ranges.iter().fold(0, |acc, r| r.fresh_ids + acc);
    println!("voila {:?}", answer);
}
