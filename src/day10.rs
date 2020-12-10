use std::{
    collections::{HashMap},
    cmp::min,
};

fn get_sorted_joltage_vec() -> Vec<usize> {
    let mut adapter_list: Vec<usize> = super::file::read_file("./inputs/day10.txt")
            .map(|n| n.parse::<usize>().expect("line is not an usize"))
            .collect();
    adapter_list.sort_unstable();
    adapter_list.insert(0, 0); // Add plug's joltage
    adapter_list.push(adapter_list.last().unwrap() + 3); // Add built-in adapter's joltage
    adapter_list
}

// Part 1
#[derive(Debug)]
enum JoltageDifferencesError {
    UnexpectedJoltageDifference(usize)
}

fn find_joltage_differences() -> Result<(usize, usize, usize), JoltageDifferencesError> {
    let adapter_list = get_sorted_joltage_vec();
    let mut joltage_differences = (0usize, 0usize, 0usize);
    let mut iter = adapter_list.iter();
    let mut prev_adapter = iter.next().unwrap();
    for adapter in iter {
        match adapter - prev_adapter {
            1 => { joltage_differences.0 += 1; },
            2 => { joltage_differences.1 += 1; },
            3 => { joltage_differences.2 += 1; },
            i => return Err(JoltageDifferencesError::UnexpectedJoltageDifference(i)),
        }
        prev_adapter = adapter;
    }
    Ok(joltage_differences)
}

// Part 2
fn get_reachability_tree(adapter_list: &Vec<usize>) -> Option<HashMap<usize, Vec<usize>>> {
    let mut reachability_tree: HashMap<usize, Vec<usize>> = HashMap::with_capacity(adapter_list.len());
    for i in 0..adapter_list.len() - 1 {
        let adapter = adapter_list[i];
        let mut reachability_vec: Vec<usize> = Vec::new();
        for j in (i + 1)..min(i + 4, adapter_list.len()) {
            if adapter_list[j] - adapter > 3 {
                break
            } else if adapter_list[j] <= adapter {
                return None
            }
            reachability_vec.push(adapter_list[j]);
        }
        reachability_tree.insert(adapter, reachability_vec);
    }
    Some(reachability_tree)
}

fn find_possible_combinations() -> Option<usize> {
    let adapter_list = get_sorted_joltage_vec();
    let reachability_tree = get_reachability_tree(&adapter_list).unwrap();
    let mut reachability_count: HashMap<usize, usize> = HashMap::with_capacity(reachability_tree.len());
    let mut iter = adapter_list.iter().rev();
    reachability_count.insert(*iter.next().unwrap(), 1);
    for adapter in iter {
        let possibilities = reachability_tree.get(adapter).unwrap().iter()
            .fold(0usize, |acc, a| acc + reachability_count.get(a).unwrap());
        reachability_count.insert(*adapter, possibilities);
    }
    Some(*reachability_count.get(&0).unwrap())
}

pub fn main () {
    // Part 1
    // let (diff_one, diff_two, diff_three) = find_joltage_differences().unwrap();
    // println!("Diff by 1: {}", diff_one);
    // println!("Diff by 2: {}", diff_two);
    // println!("Diff by 3: {}", diff_three);
    // println!("  (diff by 1) * (diff by 3) = {}", diff_one * diff_three);

    // Part 2
    let possibilities = find_possible_combinations().unwrap();
    println!("Possible arrangements: {}", possibilities);
}
