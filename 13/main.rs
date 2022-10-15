use std::{error::Error, fs::read_to_string};

use modinverse::modinverse;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_to_string("input.txt")?
        .split("\n")
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let earliest_timestamp = lines[0].parse::<i64>()?;
    let bus_ids = lines[1]
        .split(",")
        .map(|bus_id| bus_id.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let best_bus = bus_ids.iter().filter_map(|bus_id| *bus_id).fold(
        (i64::MAX, 0),
        |(time_to_wait, found_bus_id), bus_id| {
            let cur_time_to_wait = bus_id - earliest_timestamp % bus_id;
            if cur_time_to_wait < time_to_wait {
                (cur_time_to_wait, bus_id)
            } else {
                (time_to_wait, found_bus_id)
            }
        },
    );
    println!("2020-12-13 Part 1: {}", best_bus.0 * best_bus.1);

    let equations = bus_ids
        .into_iter()
        .enumerate()
        .filter_map(|(index, bus_id)| bus_id.map(|id| (-(index as i64), id)))
        .collect::<Vec<_>>();

    let n = equations.iter().fold(1, |acc, (_, id)| acc * id);

    let chinese_remainder_theorem = equations.into_iter().fold(0, |acc, (a, id)| {
        let m = n / id;
        acc + a * m * modinverse(m, id).unwrap_or_default()
    });

    println!(
        "2020-12-13 Part 2: {}",
        (n + (chinese_remainder_theorem % n) % n)
    );

    Ok(())
}
