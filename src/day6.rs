use std::{
    iter::Iterator,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// See "impl Trait" for more information on the return type:
// https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
fn read_file() -> impl Iterator<Item=String> {
    let path = Path::new("./inputs/day6.txt");
    let file = File::open(&path).expect("couldn't open file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("couldn't parse line"))
}

// Part 1
fn count_unique_answers(answers: &Vec<String>) -> usize {
    let mut unique_answers: Vec<char> = Vec::new();
    for answer in answers.iter().flat_map(|a| a.chars()) {
        if !unique_answers.contains(&answer) {
            unique_answers.push(answer)
        }
    }
    unique_answers.len()
}

// Part 2
fn count_matching_answers(answers: &Vec<String>) -> usize {
    let mut answers_iter = answers.iter();
    let mut matching_answers: Vec<char> = answers_iter.next().unwrap().chars().collect();
    for answer in answers_iter.map(|a| -> Vec<char> { a.chars().collect() } ) {
        matching_answers = matching_answers
            .iter()
            .filter(|a| answer.contains(&a))
            .map(|a| a.clone())
            .collect();
    }
    // println!("{:?}\n{:?}\n", answers, matching_answers);
    matching_answers.len()
}

fn get_total_answers() -> usize {
    let mut total_answers: usize = 0;
    let mut current_group: Vec<String> = Vec::new();
    read_file().for_each(|l| {
        if l == "" {
            // total_answers += count_unique_answers(&current_group);
            total_answers += count_matching_answers(&current_group);
            current_group = Vec::new()
        } else {
            current_group.push(l)
        }
    });
    // Final group
    // total_answers += count_unique_answers(&current_group);
    total_answers += count_matching_answers(&current_group);
    total_answers
}

pub fn main () {
    let answers = get_total_answers();
    println!("Total answers: {}", answers)
}
