use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
    iter::once,
};

use regex::Regex;

#[derive(Debug)]
struct TicketData {
    fields: HashMap<String, (u32, u32, u32, u32)>,
    ticket: Vec<u32>,
    other_tickets: Vec<Vec<u32>>,
}

impl TicketData {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let sections = input.split("\n\n").collect::<Vec<_>>();

        let field_regex = Regex::new(
            r"^(?P<name>[^:]+): (?P<r11>\d+)-(?P<r12>\d+) or (?P<r21>\d+)-(?P<r22>\d+)$",
        )?;
        let fields = HashMap::from_iter(sections[0].split("\n").filter_map(|field_line| {
            let field_match = field_regex.captures(field_line)?;
            let r = |name: &str| field_match[name].parse().ok();
            Some((
                field_match["name"].to_string(),
                (r("r11")?, r("r12")?, r("r21")?, r("r22")?),
            ))
        }));

        let ticket = sections[1]
            .split("\n")
            .last()
            .ok_or("no ticket")?
            .split(",")
            .filter_map(|n| n.parse().ok())
            .collect();

        let other_tickets = sections[2]
            .split("\n")
            .skip(1)
            .map(|ns| ns.split(",").filter_map(|n| n.parse().ok()).collect())
            .collect();

        Ok(Self {
            fields,
            ticket,
            other_tickets,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    let ticket_data = TicketData::parse(&input)?;

    let check_range: Box<dyn Fn(u32) -> bool> = ticket_data
        .fields
        .values()
        .map(|(r11, r12, r21, r22)| |n: u32| n >= *r11 && n <= *r12 || n >= *r21 && n <= *r22)
        .fold(Box::new(|_| false), |chain, f| {
            Box::new(move |n| f(n) || chain(n))
        });

    let error_rate = ticket_data
        .other_tickets
        .iter()
        .flatten()
        .filter(|n| !check_range(**n))
        .sum::<u32>();
    println!("2020-12-16 Part 1: {error_rate}");

    let valid_other_tickets = ticket_data
        .other_tickets
        .iter()
        .filter(|other_ticket| other_ticket.iter().all(|n| check_range(*n)))
        .collect::<Vec<_>>();

    let get_matching_field_indexes = |field_name: &str, known_indexes: &HashSet<u32>| {
        let (r11, r12, r21, r22) = ticket_data.fields.get(field_name).unwrap();
        (0..ticket_data.ticket.len() as u32)
            .filter(|index| !known_indexes.contains(index))
            .filter(|index| {
                once(ticket_data.ticket[*index as usize])
                    .chain(
                        valid_other_tickets
                            .iter()
                            .map(|other_ticket| other_ticket[*index as usize]),
                    )
                    .all(|n| n >= *r11 && n <= *r12 || n >= *r21 && n <= *r22)
            })
            .collect::<Vec<_>>()
    };

    let find_unique_field_index = |field_names: &HashSet<&String>, known_indexes: &HashSet<u32>| {
        field_names.iter().find_map(|field_name| {
            let indexes = get_matching_field_indexes(field_name, known_indexes);
            if indexes.len() == 1 {
                Some(((*field_name).clone(), indexes[0]))
            } else {
                None
            }
        })
    };

    let mut fields_to_identify = HashSet::<_>::from_iter(ticket_data.fields.keys());
    let mut known_fields = HashMap::new();

    while !fields_to_identify.is_empty() {
        if let Some((field_name, field_index)) = find_unique_field_index(
            &fields_to_identify,
            &known_fields.values().copied().collect::<HashSet<_>>(),
        ) {
            fields_to_identify.remove(&field_name);
            known_fields.insert(field_name, field_index);
        }
    }

    let departure_product = known_fields
        .iter()
        .filter(|(field_name, _)| field_name.starts_with("departure"))
        .map(|(_, field_index)| ticket_data.ticket[*field_index as usize])
        .fold(1, |acc, value| acc * value as u64);
    println!("2020-12-16 Part 2: {departure_product}");

    Ok(())
}
