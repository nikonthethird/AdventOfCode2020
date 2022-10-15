use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input_string = read_to_string("input.txt")?;
    let input_vec = input_string
        .split("\n")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>();

    'part1: for x in 0..input_vec.len() - 2 {
        for y in x + 1..input_vec.len() - 1 {
            if input_vec[x] + input_vec[y] == 2020 {
                println!("2020-12-01 Part 1: {}", input_vec[x] * input_vec[y]);
                break 'part1;
            }
        }
    }

    'part2: for x in 0..input_vec.len() - 3 {
        for y in x + 1..input_vec.len() - 2 {
            for z in y + 1..input_vec.len() - 1 {
                if input_vec[x] + input_vec[y] + input_vec[z] == 2020 {
                    println!(
                        "2020-12-01 Part 2: {}",
                        input_vec[x] * input_vec[y] * input_vec[z]
                    );
                    break 'part2;
                }
            }
        }
    }

    Ok(())
}
