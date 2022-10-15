use std::{collections::HashSet, error::Error, fs::read_to_string};

fn neighbor_indexes(
    (x, y, z, w): (i32, i32, i32, i32),
    dim_4: bool,
) -> impl Iterator<Item = (i32, i32, i32, i32)> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1).flat_map(move |dy| {
            (-1..=1).flat_map(move |dz| {
                (if dim_4 { -1..=1 } else { 0..=0 }).filter_map(move |dw| {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        Some((x + dx, y + dy, z + dz, w + dw))
                    } else {
                        None
                    }
                })
            })
        })
    })
}

fn alive_neighbors(
    map: &HashSet<(i32, i32, i32, i32)>,
    coord: (i32, i32, i32, i32),
    dim_4: bool,
) -> usize {
    map.intersection(&neighbor_indexes(coord, dim_4).collect())
        .count()
}

fn iterate_once(map: &HashSet<(i32, i32, i32, i32)>, dim_4: bool) -> HashSet<(i32, i32, i32, i32)> {
    let remaining_cells = map.iter().copied().filter(|coord| {
        let alive_neighbors = alive_neighbors(map, *coord, dim_4);
        alive_neighbors >= 2 && alive_neighbors <= 3
    });
    let born_cells = map
        .iter()
        .copied()
        .flat_map(|coord| neighbor_indexes(coord, dim_4))
        .filter(|neighbor_index| !map.contains(neighbor_index))
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|coord| alive_neighbors(map, *coord, dim_4) == 3);
    remaining_cells.chain(born_cells).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let cell_map = read_to_string("input.txt")?
        .split("\n")
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(char_index, char)| {
                    if char == '#' {
                        Some((char_index as i32, line_index as i32, 0, 0))
                    } else {
                        None
                    }
                })
        })
        .collect::<HashSet<_>>();

    let dim_3 = (0..6).fold(cell_map.clone(), |cell_map, _| {
        iterate_once(&cell_map, false)
    });
    println!("2020-12-17 Part 1: {}", dim_3.len());

    let dim_4 = (0..6).fold(cell_map, |cell_map, _| iterate_once(&cell_map, true));
    println!("2020-12-17 Part 2: {}", dim_4.len());
    Ok(())
}
