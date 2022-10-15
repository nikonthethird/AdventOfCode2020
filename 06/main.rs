use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let answers = read_to_string("input.txt")?
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let any_answer_sum = answers
        .iter()
        .map(|group| {
            HashSet::<char>::from_iter(group.iter().flat_map(|person| person.chars())).len()
        })
        .sum::<usize>();
    println!("2020-12-06 Part 1: {any_answer_sum}");

    let all_answer_sum = answers
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(('a'..='z').collect::<HashSet<_>>(), |set, person| {
                    set.intersection(&person.chars().collect())
                        .copied()
                        .collect()
                })
                .len()
        })
        .sum::<usize>();
    println!("2020-12-06 Part 2: {all_answer_sum}");

    Ok(())
}
