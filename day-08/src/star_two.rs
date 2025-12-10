use std::{collections::HashSet, fs};

type Coordinates = (usize, usize, usize);

fn merge_circuits(circuits: Vec<HashSet<Coordinates>>) -> Vec<HashSet<Coordinates>> {
    let mut merged_circuits: Vec<HashSet<Coordinates>> = Vec::new();
    
    for circuit in circuits {
        let existing_circuit_index = merged_circuits.iter().position(|c| circuit.iter().any(|coord| c.contains(coord)));
        
        match existing_circuit_index {
            Some(index) => {
                let existing_circuit = merged_circuits.get_mut(index).unwrap();
                existing_circuit.extend(circuit);
            },
            None => {
                merged_circuits.push(circuit);
            }
        }
    }
    
    merged_circuits
}

fn pair_coordinates(coords: &[Coordinates]) -> Vec<(Coordinates, Coordinates)> {
    let mut pairs = Vec::new();
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            pairs.push((coords[i], coords[j]));
        }
    }
    pairs
}

fn get_distance(a: Coordinates, b: Coordinates) -> f64 {
    let dx = (a.0 as isize - b.0 as isize) as f64;
    let dy = (a.1 as isize - b.1 as isize) as f64;
    let dz = (a.2 as isize - b.2 as isize) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn solve() {
    let mut junction_boxes: Vec<Coordinates> = Vec::new();
    for line in fs::read_to_string("src/input.txt").unwrap().lines() {
        let values = line
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        junction_boxes.push((values[0], values[1], values[2]));
    }

    let mut pairs = pair_coordinates(&junction_boxes);
    pairs.sort_by(|a, b| {
        let distance_a = get_distance(a.0, a.1);
        let distance_b = get_distance(b.0, b.1);
        distance_a.partial_cmp(&distance_b).unwrap()
    });

    let closest = pairs.iter().collect::<Vec<_>>();

    let mut circuits: Vec<HashSet<Coordinates>> = Vec::new();
    let mut current_circuit_a: Coordinates = (0,0,0);
    let mut current_circuit_b: Coordinates = (0,0,0);
    for (a, b) in closest.iter() {
        current_circuit_a = a.to_owned();
        current_circuit_b = b.to_owned();
        let existing_circuit_index = circuits.iter().position(|c| c.contains(a) || c.contains(b));
        match existing_circuit_index {
            Some(index) => {
                let circuit = circuits.get_mut(index).unwrap();
                circuit.insert(*a);
                circuit.insert(*b);
            },
            None => {
                let new_circuit: HashSet<Coordinates> = HashSet::from([*a, *b]);
                circuits.push(new_circuit);
            }
        }
        circuits = merge_circuits(circuits);
        if circuits.first().unwrap().len() == junction_boxes.len() {
            break;
        }
    }
    
    let answer: usize = current_circuit_a.0 * current_circuit_b.0;
    
    println!("voila {:?}", answer);
}
