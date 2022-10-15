use std::{error::Error, fs::read_to_string};

use regex::Regex;

struct ElfPassword {
    min_occurrence: usize,
    max_occurrence: usize,
    character: String,
    password: String,
}

impl ElfPassword {
    fn is_valid_part_1(&self) -> bool {
        let character_count = self.password.matches(&self.character).count();
        character_count >= self.min_occurrence && character_count <= self.max_occurrence
    }

    fn is_valid_part_2(&self) -> bool {
        let characters = self.password.chars().collect::<Vec<_>>();
        let character = self.character.chars().last().unwrap();
        (characters[self.min_occurrence - 1] == character)
            ^ (characters[self.max_occurrence - 1] == character)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_regex = Regex::new(
        r"^(?P<min_occurrence>\d+)-(?P<max_occurrence>\d+) (?P<character>\w): (?P<password>\w+)$",
    )?;
    let elf_passwords = read_to_string("input.txt")?
        .split("\n")
        .filter_map(|s| {
            input_regex.captures(s).map(|c| ElfPassword {
                min_occurrence: c["min_occurrence"].parse().unwrap(),
                max_occurrence: c["max_occurrence"].parse().unwrap(),
                character: c["character"].to_string(),
                password: c["password"].to_string(),
            })
        })
        .collect::<Vec<_>>();

    let valid_elf_passwords_part_1 = elf_passwords
        .iter()
        .filter(|elf_password| elf_password.is_valid_part_1())
        .count();
    println!("2020-12-02 Part 1: {valid_elf_passwords_part_1}");

    let valid_elf_passwords_part_2 = elf_passwords
        .iter()
        .filter(|elf_password| elf_password.is_valid_part_2())
        .count();
    println!("2020-12-02 Part 2: {valid_elf_passwords_part_2}");

    Ok(())
}
