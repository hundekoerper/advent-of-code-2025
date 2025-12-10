use std::fs;

type Coordinates = (usize, usize);

fn pair_coordinates(coords: &[Coordinates]) -> Vec<(Coordinates, Coordinates)> {
    let mut pairs = Vec::new();
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            pairs.push((coords[i], coords[j]));
        }
    }
    pairs
}

fn calculate_area(pair: &(Coordinates, Coordinates)) -> usize {
    let (tile_a_x, tile_a_y) = pair.0;
    let (tile_b_x, tile_b_y) = pair.1;
    let width = tile_a_x.abs_diff(tile_b_x) + 1;
    let height = tile_a_y.abs_diff(tile_b_y) + 1;
    width * height
}

pub fn solve() {
    let mut red_tiles: Vec<Coordinates> = Vec::new();
    for line in fs::read_to_string("src/input.txt").unwrap().lines() {
        let values = line
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        red_tiles.push((values[0], values[1]));
    }
    
    let pairs = pair_coordinates(&red_tiles);
    let areas = pairs.iter().map(calculate_area).collect::<Vec<usize>>();

    let answer = areas.iter().max().unwrap();
    
    println!("voila {:?}", answer);
}
