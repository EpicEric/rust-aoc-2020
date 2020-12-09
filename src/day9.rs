use std::{
    collections::{HashSet, VecDeque},
};
use regex::Regex;

fn find_attack_number(preamble_size: usize) -> usize {
    let mut current_preamble: VecDeque<usize> = VecDeque::new();
    let mut current_preamble_sums: VecDeque<Vec<usize>> = VecDeque::new();
    for number in
        super::file::read_file("./inputs/day9.txt")
            .map(|n| n.parse::<usize>().expect("line is not an int"))
    {
        if current_preamble.len() == preamble_size {
            if !current_preamble_sums.iter().any(
                |sum_vec| sum_vec.iter().any(
                    |sum| sum == &number
                )
            ) {
                return number;
            }
            current_preamble.pop_front();
            current_preamble_sums.pop_front();
        }
        current_preamble_sums.push_back(current_preamble.iter().map(|n| number + n).collect());
        current_preamble.push_back(number);
    }
    panic!("Couldn't find attack number!")
}

fn find_encryption_weakness(attack_number: usize) -> (usize, usize) {
    let number_list: Vec<usize> = super::file::read_file("./inputs/day9.txt")
            .map(|n| n.parse::<usize>().expect("line is not an int"))
            .collect();
    for i in 0..number_list.len() {
        let mut sum = 0usize;
        let mut min = number_list[i];
        if min == attack_number {
            break
        }
        let mut max = min;
        let mut j = i;
        let mut curr = min;
        loop {
            sum += curr;
            if sum == attack_number {
                return (min, max)
            } else if sum > attack_number {
                break
            }
            j += 1;
            curr = number_list[j];
            if curr > max {
                max = curr
            }
            if curr < min {
                min = curr
            }
        }
    }
    panic!("Couldn't find encryption weakness!")
}

pub fn main () {
    let attack_number = find_attack_number(25);
    println!("Number to attack: {}", &attack_number);
    let (weakness_min, weakness_max) = find_encryption_weakness(attack_number);
    println!("Encryption weakness: {} + {} = {}", weakness_min, weakness_max, weakness_min + weakness_max)
}
