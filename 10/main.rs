use std::{collections::HashMap, error::Error, fs::read_to_string, iter::once};

fn main() -> Result<(), Box<dyn Error>> {
    let mut adapters = read_to_string("input.txt")?
        .split("\n")
        .filter_map(|number| number.parse::<u8>().ok())
        .collect::<Vec<_>>();
    adapters.sort();

    let mut count_map = adapters
        .windows(2)
        .map(|window| window[1] - window[0])
        .fold(HashMap::new(), |mut map, n| {
            map.entry(n).and_modify(|count| *count += 1).or_insert(1);
            map
        });

    count_map.entry(adapters[0]).and_modify(|count| *count += 1);
    count_map.entry(3).and_modify(|count| *count += 1);

    println!(
        "2020-12-10 Part 1: {}",
        count_map.get(&1).ok_or("no 1")? * count_map.get(&3).ok_or("no 3")?
    );

    let adapter_arrangements = adapters
        .into_iter()
        .chain(once(u8::MAX))
        .fold(
            (1u64, 0, 1),
            |(arrangement_count, prev_adapter, bundle_length), adapter| {
                if prev_adapter + 1 == adapter {
                    (arrangement_count, adapter, bundle_length + 1)
                } else {
                    match bundle_length {
                        5 => (arrangement_count * 7, adapter, 1),
                        4 => (arrangement_count * 4, adapter, 1),
                        3 => (arrangement_count * 2, adapter, 1),
                        _ => (arrangement_count, adapter, 1),
                    }
                }
            },
        )
        .0;

    println!("2020-12-10 Part 2: {adapter_arrangements}");
    Ok(())
}
