use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
};

use hex_grid::{Coordinate, CENTER, DOWN_LEFT, DOWN_RIGHT, LEFT, RIGHT, UP_LEFT, UP_RIGHT};

fn translate_coordinates(mut coordinate_string: &[char]) -> Coordinate {
    let mut target = CENTER;
    while coordinate_string.len() > 0 {
        if coordinate_string.starts_with(&['n', 'e']) {
            coordinate_string = &coordinate_string[2..];
            target = target + UP_RIGHT;
        } else if coordinate_string.starts_with(&['n', 'w']) {
            coordinate_string = &coordinate_string[2..];
            target = target + UP_LEFT;
        } else if coordinate_string.starts_with(&['s', 'e']) {
            coordinate_string = &coordinate_string[2..];
            target = target + DOWN_RIGHT;
        } else if coordinate_string.starts_with(&['s', 'w']) {
            coordinate_string = &coordinate_string[2..];
            target = target + DOWN_LEFT;
        } else if coordinate_string.starts_with(&['e']) {
            coordinate_string = &coordinate_string[1..];
            target = target + RIGHT;
        } else {
            coordinate_string = &coordinate_string[1..];
            target = target + LEFT;
        }
    }
    target
}

fn get_neighbors(coordinate: Coordinate) -> Vec<Coordinate> {
    vec![
        coordinate + LEFT,
        coordinate + UP_LEFT,
        coordinate + UP_RIGHT,
        coordinate + RIGHT,
        coordinate + DOWN_RIGHT,
        coordinate + DOWN_LEFT,
    ]
}

fn count_black_neighbors(grid: &HashMap<Coordinate, bool>, coordinate: Coordinate) -> usize {
    get_neighbors(coordinate)
        .into_iter()
        .filter(|neighbor| grid.get(neighbor) == Some(&true))
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let coordinate_strings = read_to_string("input.txt")?
        .split('\n')
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let mut grid = HashMap::<_, bool>::new();
    for coordinate_string in coordinate_strings {
        let coordinate = translate_coordinates(&coordinate_string.chars().collect::<Vec<_>>());
        grid.entry(coordinate)
            .and_modify(|v| *v = !*v)
            .or_insert(true);
    }
    println!(
        "2020-12-24 Part 1: {}",
        grid.values().filter(|v| **v).count()
    );

    for _ in 0..100 {
        let coordinates_to_check = grid
            .iter()
            .flat_map(|(coordinate, is_black)| {
                if *is_black {
                    let mut neighbors_and_self = vec![*coordinate];
                    neighbors_and_self.append(&mut get_neighbors(*coordinate));
                    neighbors_and_self
                } else {
                    Vec::new()
                }
            })
            .collect::<HashSet<_>>();

        let coordinates_to_flip = coordinates_to_check
            .into_iter()
            .filter(|coordinate| {
                let black_neighbors = count_black_neighbors(&grid, *coordinate);
                if grid.get(coordinate) == Some(&true) {
                    black_neighbors == 0 || black_neighbors > 2
                } else {
                    black_neighbors == 2
                }
            })
            .collect::<Vec<_>>();

        for coordinate in coordinates_to_flip.into_iter() {
            grid.entry(coordinate)
                .and_modify(|v| *v = !*v)
                .or_insert(true);
        }
    }
    println!(
        "2020-12-24 Part 2: {}",
        grid.values().filter(|v| **v).count()
    );

    Ok(())
}
