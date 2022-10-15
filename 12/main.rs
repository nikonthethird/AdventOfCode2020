use std::{error::Error, fs::read_to_string};

use nalgebra::{Matrix2, Vector2};

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = read_to_string("input.txt")?
        .split("\n")
        .filter_map(|line| {
            let (inst, param_text) = line.split_at(1);
            let param = param_text.parse::<i32>().ok()?;
            Some((inst.chars().last()?, param))
        })
        .collect::<Vec<_>>();

    let part1 = instructions
        .iter()
        .fold((0, 0, 0), |(x, y, dir), (inst, param)| match (inst, dir) {
            ('N', _) | ('F', 270) => (x, y - param, dir),
            ('S', _) | ('F', 90) => (x, y + param, dir),
            ('E', _) | ('F', 0) => (x + param, y, dir),
            ('W', _) | ('F', 180) => (x - param, y, dir),
            ('L', _) => (x, y, (360 + dir - param) % 360),
            ('R', _) => (x, y, (dir + param) % 360),
            _ => unreachable!(),
        });
    println!("2020-12-12 Part 1: {}", part1.0.abs() + part1.1.abs());

    let rotate_right = Matrix2::new(0, 1, -1, 0);
    let part2 = instructions.iter().fold(
        (Vector2::new(0, 0), Vector2::new(10, 1)),
        |(s, w), (inst, param)| match (inst, param) {
            ('N', _) => (s, w + Vector2::new(0, *param)),
            ('S', _) => (s, w + Vector2::new(0, -*param)),
            ('E', _) => (s, w + Vector2::new(*param, 0)),
            ('W', _) => (s, w + Vector2::new(-*param, 0)),
            ('L', 90) | ('R', 270) => (s, rotate_right * rotate_right * rotate_right * w),
            ('L', 180) | ('R', 180) => (s, rotate_right * rotate_right * w),
            ('L', 270) | ('R', 90) => (s, rotate_right * w),
            ('F', _) => (s + w * *param, w),
            _ => unreachable!(),
        },
    );
    println!("2020-12-12 Part 2: {}", part2.0.x.abs() + part2.0.y.abs());

    Ok(())
}
