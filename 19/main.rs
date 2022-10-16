use std::{collections::HashMap, error::Error, fs::read_to_string};

use lazy_static::lazy_static;
use regex::Regex;

fn build_rule_regex(rules: &HashMap<u16, String>, rule: u16, part2: bool) -> String {
    lazy_static! {
        static ref RULE_REGEX: Regex =
            Regex::new(r"^ (\d+(?: \d+)*)(?: \| (\d+(?: \d+)*))*$").unwrap();
        static ref CHAR_REGEX: Regex = Regex::new(r#"^ "(.)"$"#).unwrap();
    }
    if part2 && rule == 8 {
        let rule_42_regex = build_rule_regex(rules, 42, part2);
        return format!("({rule_42_regex})+");
    } else if part2 && rule == 11 {
        let rule_42_regex = build_rule_regex(rules, 42, part2);
        let rule_31_regex = build_rule_regex(rules, 31, part2);
        let iterated_regex = (1..=5)
            .map(|count| format!("({rule_42_regex}{{{count}}}{rule_31_regex}{{{count}}})"))
            .collect::<Vec<_>>()
            .join("|");
        return format!("({iterated_regex})");
    }
    if let Some(rule_match) = RULE_REGEX.captures(&rules[&rule]) {
        let rule_regex = rule_match
            .iter()
            .skip(1)
            .filter_map(|rule_arm_opt| {
                rule_arm_opt.map(|rule_arm| {
                    rule_arm
                        .as_str()
                        .split(' ')
                        .map(|rule_str| {
                            build_rule_regex(rules, rule_str.parse::<u16>().unwrap(), part2)
                        })
                        .collect::<String>()
                })
            })
            .collect::<Vec<_>>()
            .join("|");
        format!("({rule_regex})")
    } else if let Some(char_match) = CHAR_REGEX.captures(&rules[&rule]) {
        char_match[1].to_string()
    } else {
        unreachable!()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let rules_and_messages = read_to_string("input.txt")?
        .split("\n\n")
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let rules = rules_and_messages[0]
        .split('\n')
        .map(|rule_line| {
            let rule_parts = rule_line.split(':').collect::<Vec<_>>();
            (
                rule_parts[0].parse::<u16>().unwrap(),
                rule_parts[1].to_string(),
            )
        })
        .collect::<HashMap<_, _>>();

    let part1_regex = Regex::new(&format!("^{}$", build_rule_regex(&rules, 0, false)))?;
    let part1_count = rules_and_messages[1]
        .split('\n')
        .filter(|message| part1_regex.is_match(message))
        .count();
    println!("2020-12-19 Part 1: {part1_count}");

    let part2_regex = Regex::new(&format!("^{}$", build_rule_regex(&rules, 0, true)))?;
    let part2_count = rules_and_messages[1]
        .split('\n')
        .filter(|message| part2_regex.is_match(message))
        .count();
    Ok(println!("2020-12-19 Part 2: {part2_count}"))
}
