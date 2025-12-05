use std::fs;

type Coordiantes = (usize, usize);

struct Grid {
    rows: Vec<Vec<char>>,
    paper_stacks: Vec<Coordiantes>,
}

impl Grid {
    fn get(&self, coordinates: Coordiantes) -> Option<char> {
        let (x, y) = coordinates;
        self.rows.get(y).and_then(|row| row.get(x)).cloned()
    }
    
    fn remove(&mut self, coordinates: Coordiantes) {
        let (x, y) = coordinates;
        if let Some(row) = self.rows.get_mut(y) {
            row[x] = '.';
        };
    }

    fn gather_stacks(&mut self) {
        for (y, row) in self.rows.iter().enumerate() {
            for (x, &cell_value) in row.iter().enumerate() {
                if cell_value == '@' && self.look_around((x, y)) < 4 {
                    self.paper_stacks.push((x, y));
                }
            }
        }
    }
    
    pub fn solve(&mut self) -> u64 {
        let mut gathered_stacks = 0;
        
        loop {
            for (x, y) in self.paper_stacks.clone().iter() {
                self.remove((*x, *y));
            }
            self.paper_stacks.dedup();
            self.gather_stacks();
            
            if self.paper_stacks.len() == gathered_stacks {
                break;
            }
            gathered_stacks = self.paper_stacks.len();
        }
        
        gathered_stacks as u64
    }

    pub fn look_around(&self, coordinates: Coordiantes) -> u32 {
        let (x, y) = coordinates;

        let deltas: [(isize, isize); 8] = [
                (0, 1),   // N
                (1, 1),   // NE
                (-1, 1),  // NW
                (-1, 0),  // W
                (1, 0),   // E
                (0, -1),  // S
                (1, -1),  // SE
                (-1, -1), // SW
            ];

        deltas.iter().filter(|(delta_x, delta_y)| {
            let look_at_x = (x as isize + delta_x) as usize;
            let look_at_y = (y as isize + delta_y) as usize;
            self.get((look_at_x, look_at_y)) == Some('@')
        }).count() as u32
    }
}

impl From<String> for Grid {
    fn from(s: String) -> Self {
        let rows = s.lines()
            .map(|line| {
                line.chars().collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();
        Grid { rows, paper_stacks: Vec::new() }
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    
    let mut grid = Grid::from(input);
    let answer = grid.solve();

    println!("voila {:?}", answer);
}
