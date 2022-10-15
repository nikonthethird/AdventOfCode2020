use std::{collections::HashMap, error::Error, fs::read_to_string, iter::once};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Mask { and_mask: u64, or_mask: u64 },
    Mem { address: u64, value: u64 },
}

impl Instruction {
    fn parse(instr_text: &str) -> Option<Self> {
        lazy_static! {
            static ref MASK_REGEX: Regex = Regex::new(r"^mask = (?P<mask>[10X]{36})$").unwrap();
            static ref MEM_REGEX: Regex =
                Regex::new(r"^mem\[(?P<address>\d+)\] = (?P<value>\d+)$").unwrap();
        }

        if let Some(mask_captures) = MASK_REGEX.captures(instr_text) {
            return Some(Self::Mask {
                and_mask: u64::from_str_radix(&mask_captures["mask"].replace('X', "1"), 2).ok()?,
                or_mask: u64::from_str_radix(&mask_captures["mask"].replace('X', "0"), 2).ok()?,
            });
        }

        if let Some(mem_captures) = MEM_REGEX.captures(instr_text) {
            return Some(Self::Mem {
                address: mem_captures["address"].parse().ok()?,
                value: mem_captures["value"].parse().ok()?,
            });
        }

        None
    }

    fn execute_part1(&self, cur_mask: &mut (u64, u64), cur_mem: &mut HashMap<u64, u64>) {
        match self {
            Self::Mask { and_mask, or_mask } => *cur_mask = (*and_mask, *or_mask),
            Self::Mem { address, value } => {
                cur_mem.insert(*address, value & cur_mask.0 | cur_mask.1);
            }
        }
    }

    fn execute_part2(&self, cur_mask: &mut (u64, u64), cur_mem: &mut HashMap<u64, u64>) {
        fn get_floating_masks(template: u64) -> Box<dyn Iterator<Item = u64>> {
            match template.trailing_zeros() {
                64 => Box::new(once(0)),
                bit_index => Box::new(
                    get_floating_masks(template & !(1 << bit_index))
                        .map(move |mask| mask | 1 << bit_index)
                        .chain(get_floating_masks(template & !(1 << bit_index))),
                ),
            }
        }
        match self {
            Self::Mask { and_mask, or_mask } => *cur_mask = (*and_mask, *or_mask),
            Self::Mem { address, value } => {
                let floating_mask_template = cur_mask.0 ^ cur_mask.1;
                for floating_mask in get_floating_masks(floating_mask_template) {
                    cur_mem.insert(
                        ((*address | cur_mask.1) & !floating_mask_template) | floating_mask,
                        *value,
                    );
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let program = read_to_string("input.txt")?
        .split("\n")
        .filter_map(Instruction::parse)
        .collect::<Vec<_>>();

    let mut mask = (0, 0);
    let mut mem = HashMap::new();
    for instruction in program.iter() {
        instruction.execute_part1(&mut mask, &mut mem);
    }
    println!("2020-12-14 Part 1: {}", mem.values().sum::<u64>());

    mem.clear();
    for instruction in program {
        instruction.execute_part2(&mut mask, &mut mem);
    }
    println!("2020-12-14 Part 2: {}", mem.values().sum::<u64>());

    Ok(())
}
