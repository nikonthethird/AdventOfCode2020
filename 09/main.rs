use std::{collections::VecDeque, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let numbers = read_to_string("input.txt")?
        .split("\n")
        .filter_map(|number| number.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let mut index = 25;
    let mut preamble = numbers.iter().take(index).copied().collect::<VecDeque<_>>();
    let mut invalid_number = 0;

    while index < numbers.len() {
        invalid_number = numbers[index];
        let mut number_constructed = false;
        'part1: for x in 0..preamble.len() - 1 {
            for y in x + 1..preamble.len() {
                if preamble[x] + preamble[y] == invalid_number {
                    number_constructed = true;
                    break 'part1;
                }
            }
        }
        if !number_constructed {
            println!("2020-12-09 Part 1: {invalid_number}");
            break;
        }
        preamble.pop_front();
        preamble.push_back(invalid_number);
        index += 1;
    }

    'part2: for x in 0..numbers.len() - 1 {
        for y in x + 1..numbers.len() {
            if invalid_number == numbers[x..=y].iter().sum::<u64>() {
                let min = numbers[x..=y].iter().min().ok_or("no min")?;
                let max = numbers[x..=y].iter().max().ok_or("no max")?;
                println!("2020-12-09 Part 2: {}", min + max);
                break 'part2;
            }
        }
    }

    Ok(())
}
