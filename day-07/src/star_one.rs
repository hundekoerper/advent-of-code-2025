use std::fs;

type Coordiantes = (usize, usize);

struct Grid {
    rows: Vec<Vec<char>>,
    splits: u64
}

impl Grid {
    fn beam(&mut self, coordinates: Coordiantes) {
        let (x, y) = coordinates;
        
        if let Some(row) = self.rows.get_mut(y) {
            match row.get_mut(x) {
                Some('.') => {
                    row[x] = '|'
                },
                Some('^') => {
                    row[x - 1] = '|';
                    row[x + 1] = '|';
                    self.splits += 1;
                },
                _ => {}
            }
        };
    }
    
    fn ima_firin_mah_lazer(&mut self) -> u64 {
        let mut beam_locations = self.rows[0].iter().enumerate().filter(|(_, char)| **char == 'S').map(|(index, _)| index).collect::<Vec<_>>();
        for (y, _) in self.rows.clone().iter().enumerate().skip(1) {
            for (x, _) in self.rows[y].clone().iter().enumerate() {
                let coordinates = (x, y);
                if beam_locations.contains(&x) {
                    self.beam(coordinates);
                }
            }
            beam_locations = self.rows[y].clone().iter().enumerate().filter(|(_, char)| **char == '|').map(|(index, _)| index).collect::<Vec<_>>();
        }
        
        self.splits
    }
}

impl From<String> for Grid {
    fn from(s: String) -> Self {
        let rows = s.lines()
            .map(|line| {
                line.chars().collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();
        Grid { rows, splits: 0 }
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/test.txt").unwrap();
    
    let mut grid = Grid::from(input);
    let answer = grid.ima_firin_mah_lazer();
    
    println!("voila {:?}", answer);
}