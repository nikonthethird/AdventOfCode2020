use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut starting_numbers = VecDeque::from([0, 1, 4, 13, 15, 12, 16]);

    let mut number_map = HashMap::new();
    let mut last_number = 0;

    for current_turn in 1..=30000000 {
        if let Some(n) = starting_numbers.pop_front() {
            last_number = n;
            number_map.insert(n, (current_turn, None));
            continue;
        }

        let (n, maybe_pn) = number_map[&last_number];
        last_number = maybe_pn.map(|pn| n - pn).unwrap_or_default();
        number_map
            .entry(last_number)
            .and_modify(|(n, pn)| {
                *pn = Some(*n);
                *n = current_turn;
            })
            .or_insert((current_turn, None));

        if current_turn == 2020 {
            println!("2020-12-15 Part 1: {last_number}")
        }
    }

    Ok(println!("2020-12-15 Part 2: {last_number}"))
}
