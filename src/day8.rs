use std::{
    collections::HashSet,
    iter::Iterator,
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use regex::Regex;

fn read_file() -> impl Iterator<Item=String> {
    let path = Path::new("./inputs/day8.txt");
    let file = File::open(&path).expect("couldn't open file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("couldn't parse line"))
}

#[derive(Debug, Clone)]
enum Instruction {
    Accumulator(isize),
    Jump(isize),
    NoOp(isize),
}

fn parse_instruction(line: String) -> Instruction {
    let mut split = line.splitn(2, " ");
    let operation = split.next().expect("missing operation in instruction");
    let argument = split.next().expect("missing argument in instruction");
    match operation {
        "acc" => Instruction::Accumulator(argument.parse().expect("invalid acc argument")),
        "jmp" => Instruction::Jump(argument.parse().expect("invalid jmp argument")),
        "nop" => Instruction::NoOp(argument.parse().expect("invalid nop argument")),
        _ => panic!("Invalid operation")
    }
}

// Part 1
fn run_code_until_loop() {
    let mut acc = 0isize;
    let mut head = 0usize;
    let mut prev_heads: HashSet<usize> = HashSet::new();
    let code: Vec<Instruction> = read_file().map(parse_instruction).collect();
    loop {
        if prev_heads.contains(&head) {
            println!("LOOP DETECTED!");
            println!("Current acc: {}", acc);
            return
        }
        prev_heads.insert(head);
        match code[head] {
            Instruction::Accumulator(inc) => {
                acc += inc;
                head += 1;
            },
            Instruction::Jump(inc) => {
                head = (head as isize + inc) as usize;
            },
            Instruction::NoOp(_) => {
                head += 1;
            },
        }
    }
}

// Part 2
fn fix_code() {
    let original_code: Vec<Instruction> = read_file().map(parse_instruction).collect();
    for (i, fixable_instruction) in original_code.iter().enumerate() {
        let mut code = original_code.clone();
        match fixable_instruction {
            Instruction::Jump(arg) => code[i] = Instruction::NoOp(arg.clone()),
            Instruction::NoOp(arg) => code[i] = Instruction::Jump(arg.clone()),
            _ => continue,
        }
        let mut acc = 0isize;
        let mut head = 0usize;
        let mut prev_heads: HashSet<usize> = HashSet::new();
        loop {
            if prev_heads.contains(&head) {
                // LOOP DETECTED!
                break
            }
            if head >= code.len() {
                println!("Fixed by changing line {}! Final accumulator is: {}", i + 1, acc);
                return
            }
            prev_heads.insert(head);
            match code[head] {
                Instruction::Accumulator(inc) => {
                    acc += inc;
                    head += 1;
                },
                Instruction::Jump(inc) => {
                    head = (head as isize + inc) as usize;
                },
                Instruction::NoOp(_) => {
                    head += 1;
                },
            }
        }
    }
}

pub fn main () {
    fix_code()
}
