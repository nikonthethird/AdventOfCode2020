use std::{collections::HashSet, error::Error, fs::read_to_string};

struct Instruction {
    name: String,
    count: isize,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let parts = input.split(' ').collect::<Vec<_>>();
        Self {
            name: parts[0].to_string(),
            count: parts[1].parse().unwrap(),
        }
    }

    fn execute(&self, instruction_pointer: &mut usize, accumulator: &mut isize) {
        *instruction_pointer += 1;
        match self.name.as_str() {
            "jmp" => {
                *instruction_pointer = (*instruction_pointer as isize + self.count - 1) as usize
            }
            "acc" => *accumulator += self.count,
            _ => (),
        }
    }

    fn toggle(&self) -> Option<Self> {
        match self.name.as_str() {
            "jmp" => Some(Self {
                name: "nop".to_string(),
                count: self.count,
            }),
            "nop" => Some(Self {
                name: "jmp".to_string(),
                count: self.count,
            }),
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut program = read_to_string("input.txt")?
        .split("\n")
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    let (mut instruction_pointer, mut accumulator, mut visited_instructions) =
        (0, 0, HashSet::new());

    while visited_instructions.insert(instruction_pointer) {
        program[instruction_pointer].execute(&mut instruction_pointer, &mut accumulator);
    }
    println!("2020-12-08 Part 1: {accumulator}");

    for toggle_index in 0..program.len() {
        if let Some(toggled_instruction) = program[toggle_index].toggle() {
            program[toggle_index] = toggled_instruction;

            (instruction_pointer, accumulator) = (0, 0);
            visited_instructions.clear();

            while instruction_pointer < program.len()
                && visited_instructions.insert(instruction_pointer)
            {
                program[instruction_pointer].execute(&mut instruction_pointer, &mut accumulator);
            }

            if instruction_pointer >= program.len() {
                break;
            }

            program[toggle_index] = program[toggle_index].toggle().unwrap();
        }
    }
    println!("2020-12-08 Part 2: {accumulator}");

    Ok(())
}
