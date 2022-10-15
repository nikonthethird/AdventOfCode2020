use std::{error::Error, fs::read_to_string};

fn check_slope(tree_map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let (mut x, mut y, mut tree_count) = (0, 0, 0);
    while y < tree_map.len() - down {
        x = (x + right) % tree_map[0].len();
        y += down;

        if tree_map[y][x] == '#' {
            tree_count += 1;
        }
    }
    tree_count
}

fn main() -> Result<(), Box<dyn Error>> {
    let tree_map = read_to_string("input.txt")?
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let c = |right, down| check_slope(&tree_map, right, down);
    println!("2020-12-03 Part 1: {}", c(3, 1));
    println!(
        "2020-12-03 Part 2: {}",
        c(1, 1) * c(3, 1) * c(5, 1) * c(7, 1) * c(1, 2)
    );

    Ok(())
}
