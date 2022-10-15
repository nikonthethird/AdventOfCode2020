use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
};

use regex::Regex;

fn bags_containing_bag<'a>(
    rules: &'a HashMap<String, HashMap<String, usize>>,
    bag_name: &'a str,
) -> impl Iterator<Item = &'a String> + 'a {
    rules.iter().filter_map(|(a, b)| {
        if b.contains_key(bag_name) {
            Some(a)
        } else {
            None
        }
    })
}

fn all_bags_containing_bag<'a>(
    rules: &'a HashMap<String, HashMap<String, usize>>,
    bag_name: &'a str,
) -> HashSet<&'a String> {
    let bags = bags_containing_bag(rules, bag_name).collect::<HashSet<_>>();
    bags.clone().into_iter().fold(bags, |acc, next_bag_name| {
        acc.union(&all_bags_containing_bag(rules, next_bag_name))
            .cloned()
            .collect::<HashSet<_>>()
    })
}

fn count_bag_contents<'a>(
    rules: &'a HashMap<String, HashMap<String, usize>>,
    bag_name: &'a str,
) -> usize {
    rules
        .get(bag_name)
        .into_iter()
        .flat_map(|contained_bags| contained_bags.iter())
        .fold(0, |acc, (next_bag_name, next_bag_count)| {
            acc + next_bag_count * (1 + count_bag_contents(rules, next_bag_name))
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    let rule_regex = Regex::new(
        r"^(?P<color>.+?) bags contain(?: no other bags.|(?P<inner>(?: \d+ .+? bags?[,.])+))$",
    )?;
    let inner_rule_regex = Regex::new(r"^ (?P<count>\d+) (?P<color>.+?) bags?")?;
    let rules =
        HashMap::<_, _>::from_iter(read_to_string("input.txt")?.split("\n").filter_map(|rule| {
            let rule_match = rule_regex.captures(rule)?;
            Some((
                rule_match["color"].to_string(),
                HashMap::<_, _>::from_iter(rule_match.name("inner").into_iter().flat_map(
                    |inner_match| {
                        inner_match.as_str().split(',').filter_map(|inner| {
                            let inner_match = inner_rule_regex.captures(inner)?;
                            Some((
                                inner_match["color"].to_string(),
                                inner_match["count"].parse::<usize>().ok()?,
                            ))
                        })
                    },
                )),
            ))
        }));

    println!(
        "2020-12-07 Part 1: {}",
        all_bags_containing_bag(&rules, "shiny gold").len()
    );

    println!(
        "2020-12-07 Part 2: {}",
        count_bag_contents(&rules, "shiny gold")
    );

    Ok(())
}
