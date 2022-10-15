use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let boarding_passes = read_to_string("input.txt")?
        .split("\n")
        .filter_map(|line| {
            let (fst, snd) = line.split_at(7);
            Some((
                usize::from_str_radix(&fst.replace("F", "0").replace("B", "1"), 2).ok()?,
                usize::from_str_radix(&snd.replace("L", "0").replace("R", "1"), 2).ok()?,
            ))
        })
        .collect::<Vec<_>>();

    let seat_ids = boarding_passes
        .iter()
        .map(|(row, column)| row * 8 + column)
        .collect::<HashSet<_>>();

    let lowest_seat_id = seat_ids.iter().min().ok_or("no id")?;
    let highest_seat_id = seat_ids.iter().max().ok_or("no id")?;

    println!("2020-12-05 Part 1: {highest_seat_id}");

    for seat_id in *lowest_seat_id..*highest_seat_id {
        if !seat_ids.contains(&seat_id)
            && seat_ids.contains(&(seat_id - 1))
            && seat_ids.contains(&(seat_id + 1))
        {
            println!("2020-12-05 Part 2: {seat_id}");
            break;
        }
    }

    Ok(())
}
