use std::{collections::HashMap, fs};

type Coordiantes = (usize, usize);

struct Grid {
    rows: Vec<Vec<char>>,
    timeline_map: HashMap<Coordiantes, u64>
}

impl Grid {    
    fn ima_firin_mah_lazer(&mut self)  {
        let start_index = self.rows[0].iter().position(|c| *c =='S').unwrap();
        self.timeline_map.insert((start_index, 0), 1);
        
        for (y, _) in self.rows.clone().iter().enumerate().skip(1) {
            for (x, _) in self.rows[y].clone().iter().enumerate() {
                let coordinates = (x, y);
                match self.rows[y][x] {
                    '.' => { 
                        let value_above = self.timeline_map.get(&(x, y -1)).unwrap().to_owned();
                        let value = self.timeline_map.get(&coordinates).unwrap();
                        self.timeline_map.insert(coordinates, value + value_above);
                    },
                    '^' => {
                        let value_above = self.timeline_map.get(&(x, y -1)).unwrap().to_owned();
                        let value_left = self.timeline_map.get(&(x -1, y)).unwrap();
                        self.timeline_map.insert((x -1, y), value_left + value_above);
                        
                        let value_right = self.timeline_map.get(&(x +1, y)).unwrap();
                        self.timeline_map.insert((x + 1, y), value_right + value_above);
                    },
                    _ => {},
                }
            }
        }
    }
    
    fn solve(&self) -> u64 {
        self.timeline_map.iter().filter(|&(k, _)| k.1 == &self.rows.len() - 1).map(|(_, v)| *v).sum()
    }
}

impl From<String> for Grid {
    fn from(s: String) -> Self {
        let rows = s.lines()
            .map(|line| {
                line.chars().collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();
        
        let mut timeline_map: HashMap<Coordiantes, u64> = HashMap::new();
        
        for (y, _) in rows.iter().enumerate() {
            for (x, _) in rows[y].iter().enumerate() {
                timeline_map.insert((x, y), 0);
            }
        }

        Grid { rows, timeline_map }
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    
    let mut grid = Grid::from(input);
    grid.ima_firin_mah_lazer();
    
    let answer = grid.solve();
    
    println!("voila {:?}", answer);
}
