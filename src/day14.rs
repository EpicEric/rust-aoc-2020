use std::{
    collections::HashMap,
    convert::TryFrom,
    iter::from_fn,
};
use regex::Regex;

#[derive(Debug, Clone)]
struct MaskDataPart1 {
    and_mask: usize,
    or_mask: usize,
}

impl MaskDataPart1 {
    fn apply_to(&self, value: &usize) -> usize {
        (value & self.and_mask) | self.or_mask
    }
}

#[derive(Debug, Clone)]
struct MaskDataPart2 {
    or_mask: usize,
    floating_bits: Vec<usize>
}

impl MaskDataPart2 {
    fn apply_to<'a>(&'a self, address: &'a usize) -> impl Iterator<Item=usize> + 'a {
        // TODO
        let mut combination = 0usize;
        let base_address = *address | self.or_mask;
        from_fn(move || {
            if combination >= (1 << self.floating_bits.len()) {
                None
            } else {
                let mut curr_address = base_address;
                for (i, bit) in self.floating_bits.iter().enumerate() {
                    if (1 << i) & combination == 0 {
                        curr_address &= !(1 << bit);
                    } else {
                        curr_address |= 1 << bit;
                    }
                }
                combination += 1;
                Some(curr_address)
            }
        })
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    MaskPart1(MaskDataPart1),
    MaskPart2(MaskDataPart2),
    Mem(usize, usize),
}

impl TryFrom<String> for Instruction {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.splitn(2, " = ");
        let operation = split.next().expect("missing operation in instruction");
        let argument = split.next().expect("missing argument in instruction");
        match operation {
            "mask" => {
                // Part 1
                // match (usize::from_str_radix(&argument.replace('X', "1"), 2), usize::from_str_radix(&argument.replace('X', "0"), 2)) {
                //     (Ok(and_mask), Ok(or_mask)) => Ok(Instruction::MaskPart1(MaskDataPart1{
                //         and_mask: and_mask | ((-1isize as usize) << &argument.len()),
                //         or_mask: or_mask,
                //     })),
                //     _ => Err(())
                // }
                match usize::from_str_radix(&argument.replace('X', "0"), 2) {
                    Ok(or_mask) => {
                        let floating_bits = (&argument).chars().rev().enumerate().filter(|(_, c)| c == &'X').map(|(i, _)| i).collect();
                        Ok(Instruction::MaskPart2(MaskDataPart2{
                            or_mask: or_mask,
                            floating_bits: floating_bits,
                        }))
                    },
                    _ => Err(())
                }
            },
            mem_op => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^mem\[(\d+)\]$").expect("invalid mem regex");
                }
                match RE.captures(&mem_op) {
                    Some(caps) => {
                        match (caps[1].parse(), argument.parse()) {
                            (Ok(mem_address), Ok(mem_value)) => Ok(Instruction::Mem(mem_address, mem_value)),
                            _ => Err(())
                        }
                    },
                    _ => Err(()),
                }
            }
        }
    }
}

fn run_program() -> HashMap<usize, usize> {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut program = super::file::read_file("./inputs/day14.txt").map(|l| Instruction::try_from(l).expect("invalid instruction"));
    let first_instruction = program.next();
    // println!("{:?}", first_instruction);
    let mut current_mask = match first_instruction {
        // Some(Instruction::MaskPart1(data)) => data,
        Some(Instruction::MaskPart2(data)) => data,
        _ => panic!("First instruction is not a valid mask"),
    };
    for instruction in program {
        // println!("{:?}", instruction);
        match instruction {
            // Instruction::MaskPart1(mask_data) => current_mask = mask_data,
            Instruction::MaskPart2(mask_data) => current_mask = mask_data,
            Instruction::Mem(address, value) => {
                // memory.insert(address, current_mask.apply_to(&value));
                for addr in current_mask.apply_to(&address) {
                    memory.insert(addr, value);
                }
            }
            _ => panic!("Unexpected instruction"),
        }
    }
    memory
}

pub fn main () {
    let memory = run_program();
    // println!("{:?}", memory);
    println!("Sum of values in memory: {}", memory.values().sum::<usize>())
}
