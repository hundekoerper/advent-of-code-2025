use std::fs;

type Coordiantes = (usize, usize);

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn get(&self, coordinates: Coordiantes) -> Option<char> {
        let (x, y) = coordinates;
        self.rows.get(y).and_then(|row| row.get(x)).cloned()
    }

    pub fn solve(&self) -> u64 {
        let mut papter_stacks = 0;
        for (y, row) in self.rows.iter().enumerate() {
            for (x, &cell_value) in row.iter().enumerate() {
                if cell_value == '@' && self.look_around((x, y)) < 4 {
                    papter_stacks += 1;
                }
            }
        }
        
        papter_stacks
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
        Grid { rows }
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    
    let grid = Grid::from(input);
    let answer = grid.solve();

    println!("voila {:?}", answer);
}
