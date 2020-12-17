use std::{
    collections::HashMap,
};

fn get_starting_numbers() -> Vec<usize> {
    super::file::read_file("./inputs/day15.txt").next().unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().expect("not a valid number"))
        .collect()
}

fn run_memory_game(last_turn: usize) -> usize {
    let mut last_spoken: HashMap<usize, usize> = HashMap::new();
    let starting_numbers = get_starting_numbers();
    for (i, starting_number) in starting_numbers.iter().enumerate() {
        last_spoken.insert(*starting_number, i + 1);
    }
    let mut curr_turn = starting_numbers.len() + 1;
    let mut curr_value = 0usize;
    while curr_turn < last_turn {
        let entry = last_spoken.entry(curr_value).or_insert(0);
        curr_value = match &entry {
            0 => 0,
            prev_turn => curr_turn - **prev_turn,
        };
        *entry = curr_turn;
        curr_turn += 1;
        // println!("{}) {}! {:?}", curr_turn, curr_value, last_spoken);
    }
    curr_value
}

pub fn main () {
    // let turns: usize = 2020;
    let turns: usize = 30000000; // A bit slow but still under 30 seconds
    let final_value = run_memory_game(turns);
    println!("The {}th value is: {}", turns, final_value);
}
