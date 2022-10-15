use std::{error::Error, fs::read_to_string};

use lazy_static::lazy_static;
use regex::Regex;

struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
}

impl Passport {
    fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref BIRTH_YEAR_REGEX: Regex = Regex::new(r"(?s)\bbyr:(?P<byr>\d+)\b").unwrap();
            static ref ISSUE_YEAR_REGEX: Regex = Regex::new(r"(?s)\biyr:(?P<iyr>\d+)\b").unwrap();
            static ref EXPIRATION_YEAR_REGEX: Regex =
                Regex::new(r"(?s)\beyr:(?P<eyr>\d+)\b").unwrap();
            static ref HEIGHT_REGEX: Regex = Regex::new(r"(?s)\bhgt:(?P<hgt>\S+)\b").unwrap();
            static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"(?s)\bhcl:(?P<hcl>\S+)\b").unwrap();
            static ref EYE_COLOR_REGEX: Regex = Regex::new(r"(?s)\becl:(?P<ecl>\S+)\b").unwrap();
            static ref PASSPORT_ID_REGEX: Regex = Regex::new(r"(?s)\bpid:(?P<pid>\S+)\b").unwrap();
        }

        Some(Passport {
            birth_year: BIRTH_YEAR_REGEX.captures(input)?["byr"].parse().ok()?,
            issue_year: ISSUE_YEAR_REGEX.captures(input)?["iyr"].parse().ok()?,
            expiration_year: EXPIRATION_YEAR_REGEX.captures(input)?["eyr"].parse().ok()?,
            height: HEIGHT_REGEX.captures(input)?["hgt"].to_string(),
            hair_color: HAIR_COLOR_REGEX.captures(input)?["hcl"].to_string(),
            eye_color: EYE_COLOR_REGEX.captures(input)?["ecl"].to_string(),
            passport_id: PASSPORT_ID_REGEX.captures(input)?["pid"].to_string(),
        })
    }

    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HEIGHT_REGEX: Regex =
                Regex::new(r"^(?P<number>\d+)(?P<unit>cm|in)$").unwrap();
            static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref EYE_COLOR_REGEX: Regex =
                Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PASSPORT_ID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        (self.birth_year >= 1920 && self.birth_year <= 2002)
            && (self.issue_year >= 2010 && self.issue_year <= 2020)
            && (self.expiration_year >= 2020 && self.expiration_year <= 2030)
            && (HEIGHT_REGEX
                .captures(&self.height)
                .and_then(|h| {
                    let number = h["number"].parse::<usize>().ok()?;
                    Some(
                        (&h["unit"] == "cm" && number >= 150 && number <= 193)
                            || (&h["unit"] == "in" && number >= 59 && number <= 76),
                    )
                })
                .unwrap_or_default())
            && HAIR_COLOR_REGEX.is_match(&self.hair_color)
            && EYE_COLOR_REGEX.is_match(&self.eye_color)
            && PASSPORT_ID_REGEX.is_match(&self.passport_id)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let passports = read_to_string("input.txt")?
        .split("\n\n")
        .filter_map(Passport::parse)
        .collect::<Vec<_>>();
    println!("2020-12-04 Part 1: {}", passports.len());

    let valid_passports = passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();
    println!("2020-12-04 Part 2: {}", valid_passports);
    Ok(())
}
